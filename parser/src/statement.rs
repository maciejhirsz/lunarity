use toolshed::list::{List, GrowableList, ListBuilder};

use ast::*;
use {Parser, TopPrecedence, StatementTypeNameContext};
use lexer::Token;

/// A trait that allows for extra statements to be parsed in a specific context.
/// In particular, it's used to differentiate between function and modifier
/// bodies to allow placeholder statements (`_;`) only in the modifier definition.
pub trait StatementContext<'ast> {
    type LoopContext: StatementContext<'ast>;

    #[inline]
    fn pre_parse(&mut Parser<'ast>) -> Option<StatementNode<'ast>> {
        None
    }
}

pub struct FunctionContext;
pub struct ModifierContext;

pub struct FunctionLoopContext;
pub struct ModifierLoopContext;

impl<'ast> StatementContext<'ast> for FunctionContext {
    type LoopContext = FunctionLoopContext;
}

impl<'ast> StatementContext<'ast> for ModifierContext {
    type LoopContext = ModifierLoopContext;

    #[inline]
    fn pre_parse(par: &mut Parser<'ast>) -> Option<StatementNode<'ast>> {
        match par.lexer.token {
            Token::Identifier if par.lexer.slice() == "_" => par.token_statement(Placeholder),
            _ => None
        }
    }
}

impl<'ast> StatementContext<'ast> for FunctionLoopContext {
    type LoopContext = Self;

    #[inline]
    fn pre_parse(par: &mut Parser<'ast>) -> Option<StatementNode<'ast>> {
        match par.lexer.token {
            Token::KeywordContinue => par.token_statement(ContinueStatement),
            Token::KeywordBreak    => par.token_statement(BreakStatement),
            _ => None
        }
    }
}

impl<'ast> StatementContext<'ast> for ModifierLoopContext {
    type LoopContext = Self;

    #[inline]
    fn pre_parse(par: &mut Parser<'ast>) -> Option<StatementNode<'ast>> {
        match par.lexer.token {
            Token::Identifier if par.lexer.slice() == "_" => par.token_statement(Placeholder),
            Token::KeywordContinue => par.token_statement(ContinueStatement),
            Token::KeywordBreak    => par.token_statement(BreakStatement),
            _ => None
        }
    }
}

impl<'ast> Parser<'ast> {
    pub fn statement<Context>(&mut self) -> Option<StatementNode<'ast>>
    where
        Context: StatementContext<'ast>,
    {
        if let statement @ Some(_) = Context::pre_parse(self) {
            return statement;
        }

        match self.lexer.token {
            Token::BraceOpen       => Some(self.block::<Context, _>()),
            Token::KeywordIf       => self.if_statement::<Context>(),
            Token::KeywordWhile    => self.while_statement::<Context>(),
            Token::KeywordFor      => self.for_statement::<Context>(),
            Token::KeywordDo       => self.do_while_statement::<Context>(),
            Token::KeywordReturn   => self.return_statement(),
            Token::KeywordThrow    => self.token_statement(ThrowStatement),
            Token::KeywordAssembly => self.inline_assembly_statement(),
            Token::DeclarationVar  => self.inferred_definition_statement(),

            // _ => match self.expression_statement() {
            //     None => self.variable_definition_statement(),
            //     node => node,
            // }
            _ => match self.variable_definition_statement() {
                None => self.expression_statement(),
                node => node,
            }
        }
    }

    pub fn simple_statement(&mut self) -> Option<SimpleStatementNode<'ast>> {
        match self.lexer.token {
            Token::DeclarationVar => self.inferred_definition_statement(),

            _ => match self.variable_definition_statement() {
                None => self.expression_statement(),
                node => node,
            }
        }
    }

    /// `B` should be either `Statement` or `Block`
    pub fn block<Context, B>(&mut self) -> Node<'ast, B>
    where
        B: From<Block<'ast>> + Copy,
        Context: StatementContext<'ast>,
    {
        let start = self.start_then_advance();
        let body  = GrowableList::new();

        while let Some(statement) = self.statement::<Context>() {
            body.push(self.arena, statement);
        }

        let end = self.expect_end(Token::BraceClose);

        self.node_at(start, end, Block {
            body: body.as_list(),
        })
    }

    #[inline]
    fn token_statement<S>(&mut self, statement: S) -> Option<StatementNode<'ast>>
    where
        S: 'ast + Copy + Into<Statement<'ast>>,
    {
        let start = self.start_then_advance();
        let end   = self.expect_end(Token::Semicolon);

        self.node_at(start, end, statement)
    }

    fn if_statement<Context>(&mut self) -> Option<StatementNode<'ast>>
    where
        Context: StatementContext<'ast>,
    {
        let start = self.start_then_advance();

        self.expect(Token::ParenOpen);

        let test = expect!(self, self.expression::<TopPrecedence>());

        self.expect(Token::ParenClose);

        let consequent = expect!(self, self.statement::<Context>());
        let alternate;

        if self.allow(Token::KeywordElse) {
            alternate = self.statement::<Context>();

            if alternate.is_none() {
                self.error();
            }
        } else {
            alternate = None;
        }

        let end = alternate.end().unwrap_or_else(|| consequent.end);

        self.node_at(start, end, IfStatement {
            test,
            consequent,
            alternate,
        })
    }

    fn while_statement<Context>(&mut self) -> Option<StatementNode<'ast>>
    where
        Context: StatementContext<'ast>,
    {
        let start = self.start_then_advance();

        self.expect(Token::ParenOpen);

        let test = expect!(self, self.expression::<TopPrecedence>());

        self.expect(Token::ParenClose);

        let body = expect!(self, self.statement::<Context::LoopContext>());

        self.node_at(start, body.end, WhileStatement {
            test,
            body,
        })
    }

    fn for_statement<Context>(&mut self) -> Option<StatementNode<'ast>>
    where
        Context: StatementContext<'ast>,
    {
        let start = self.start_then_advance();

        self.expect(Token::ParenOpen);

        let init  = self.simple_statement();

        if init.is_none() {
            self.expect(Token::Semicolon);
        }

        let test = self.expression::<TopPrecedence>();

        self.expect(Token::Semicolon);

        let update = self.expression::<TopPrecedence>();

        self.expect(Token::ParenClose);

        let body = expect!(self, self.statement::<Context::LoopContext>());

        self.node_at(start, body.end, ForStatement {
            init,
            test,
            update,
            body,
        })
    }

    fn do_while_statement<Context>(&mut self) -> Option<StatementNode<'ast>>
    where
        Context: StatementContext<'ast>,
    {
        let start = self.start_then_advance();
        let body  = expect!(self, self.statement::<Context::LoopContext>());

        self.expect(Token::KeywordWhile);
        self.expect(Token::ParenOpen);

        let test = expect!(self, self.expression::<TopPrecedence>());

        self.expect(Token::ParenClose);

        let end = self.expect_end(Token::Semicolon);

        self.node_at(start, end, DoWhileStatement {
            body,
            test,
        })
    }

    fn return_statement(&mut self) -> Option<StatementNode<'ast>> {
        let start = self.start_then_advance();
        let value = self.expression::<TopPrecedence>();
        let end   = self.expect_end(Token::Semicolon);

        self.node_at(start, end, ReturnStatement {
            value,
        })
    }

    fn inline_assembly_statement(&mut self) -> Option<StatementNode<'ast>> {
        let start  = self.start_then_advance();
        let string = self.allow_str_node(Token::LiteralString);

        if self.lexer.token != Token::BraceOpen {
            self.error();
        }

        let block = expect!(self, self.inline_assembly_block());

        self.node_at(start, block.end, InlineAssemblyStatement {
            string,
            block,
        })
    }

    fn expression_statement<S>(&mut self) -> Option<Node<'ast, S>>
    where
        S: From<ExpressionNode<'ast>> + Copy,
    {
        let expression = self.expression::<TopPrecedence>()?;
        let end        = self.expect_end(Token::Semicolon);

        self.node_at(expression.start, end, expression)
    }

    /// `S` should be either `Statement` or `SimpleStatement`
    fn variable_definition_statement<S>(&mut self) -> Option<Node<'ast, S>>
    where
        S: From<VariableDefinitionStatement<'ast>> + Copy,
    {
        let declaration = self.variable_declaration::<StatementTypeNameContext>()?;

        let init;

        if self.allow(Token::Assign) {
            init = self.expression::<TopPrecedence>();

            if init.is_none() {
                self.error();
            }
        } else {
            init = None
        }

        let end = self.expect_end(Token::Semicolon);

        self.node_at(declaration.start, end, VariableDefinitionStatement {
            declaration,
            init,
        })
    }

    /// `S` should be either `Statement` or `SimpleStatement`
    fn inferred_definition_statement<S>(&mut self) -> Option<Node<'ast, S>>
    where
        S: From<InferredDefinitionStatement<'ast>> + Copy,
    {
        let start = self.start_then_advance();

        let ids = if self.allow(Token::ParenOpen) {
            self.tuple_destructing()
        } else {
            List::from(self.arena, Some(self.expect_str_node(Token::Identifier)))
        };

        self.expect(Token::Assign);

        let init = expect!(self, self.expression::<TopPrecedence>());
        let end  = self.expect_end(Token::Semicolon);

        self.node_at(start, end, InferredDefinitionStatement {
            ids,
            init,
        })
    }

    fn tuple_destructing(&mut self) -> List<'ast, Option<IdentifierNode<'ast>>> {
        if self.allow(Token::ParenClose) {
            return List::empty();
        }

        let builder = ListBuilder::new(self.arena, self.allow_str_node(Token::Identifier));

        while self.allow(Token::Comma) {
            builder.push(self.arena, self.allow_str_node(Token::Identifier));
        }

        self.expect(Token::ParenClose);

        builder.as_list()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use mock::{Mock, assert_units};

    #[test]
    fn empty_block() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function wow() {}
            }

        "#, [
            m.node(14, 76, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 62, FunctionDefinition {
                        name: m.node(54, 57, "wow"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 62, Block {
                            body: NodeList::empty(),
                        })
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn no_placeholder_in_functions() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    _;
                }
            }

        "#, [
            m.node(14, 116, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 102, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 102, Block {
                            body: m.list([
                                m.stmt_expr(82, 83, 84,"_")
                            ]),
                        })
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn if_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    if (true) {
                        stuff;
                    }

                    if (true) {
                        foo;
                    } else {
                        bar;
                    }

                    if (true) {
                        doge;
                    } else if (true) {
                        such;
                    } else {
                        moon;
                    }
                }
            }

        "#, [
            m.node(14, 533, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 519, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 519, Block {
                            body: m.list([
                                m.node(82, 146, IfStatement {
                                    test: m.node(86, 90, Primitive::Bool(true)),
                                    consequent: m.node(92, 146, Block {
                                        body: m.list([
                                            m.stmt_expr(118, 123, 124, "stuff"),
                                        ]),
                                    }),
                                    alternate: None,
                                }),
                                m.node(168, 288, IfStatement {
                                    test: m.node(172, 176, Primitive::Bool(true)),
                                    consequent: m.node(178, 230, Block {
                                        body: m.list([
                                            m.stmt_expr(204, 207, 208, "foo"),
                                        ]),
                                    }),
                                    alternate: m.node(236, 288, Block {
                                        body: m.list([
                                            m.stmt_expr(262, 265, 266, "bar"),
                                        ]),
                                    }),
                                }),
                                m.node(310, 501, IfStatement {
                                    test: m.node(314, 318, Primitive::Bool(true)),
                                    consequent: m.node(320, 373, Block {
                                        body: m.list([
                                            m.stmt_expr(346, 350, 351, "doge"),
                                        ]),
                                    }),
                                    alternate: m.node(379, 501, IfStatement {
                                        test: m.node(383, 387, Primitive::Bool(true)),
                                        consequent: m.node(389, 442, Block {
                                            body: m.list([
                                                m.stmt_expr(415, 419, 420, "such"),
                                            ]),
                                        }),
                                        alternate: m.node(448, 501, Block {
                                            body: m.list([
                                                m.stmt_expr(474, 478, 479, "moon"),
                                            ]),
                                        }),
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn while_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    while (true) {
                        neverStopStopping;
                    }
                }
            }

        "#, [
            m.node(14, 193, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 179, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 179, Block {
                            body: m.list([
                                m.node(82, 161, WhileStatement {
                                    test: m.node(89, 93, Primitive::Bool(true)),
                                    body: m.node(95, 161, Block {
                                        body: m.list([
                                            m.stmt_expr(121, 138, 139, "neverStopStopping"),
                                        ]),
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn for_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    for (uint32 i = 0; i < 9000; i++) {
                        whatIsThePowerLevel;
                    }
                }
            }

        "#, [
            m.node(14, 216, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 202, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 202, Block {
                            body: m.list([
                                m.node(82, 184, ForStatement {
                                    init: m.node(87, 100, VariableDefinitionStatement {
                                        declaration: m.node(87, 95, VariableDeclaration {
                                            type_name: m.node(87, 93, ElementaryTypeName::Uint(4)),
                                            location: None,
                                            id: m.node(94, 95, "i"),
                                        }),
                                        init: m.node(98, 99, Primitive::IntegerNumber("0", NumberUnit::None)),
                                    }),
                                    test: m.node(101, 109, BinaryExpression {
                                        left: m.node(101, 102, "i"),
                                        operator: m.node(103, 104, BinaryOperator::Lesser),
                                        right: m.node(105, 109, Primitive::IntegerNumber("9000", NumberUnit::None)),
                                    }),
                                    update: m.node(111, 114, PostfixExpression {
                                        operand: m.node(111, 112, "i"),
                                        operator: m.node(112, 114, PostfixOperator::Increment),
                                    }),
                                    body: m.node(116, 184, Block {
                                        body: m.list([
                                            m.stmt_expr(142, 161, 162, "whatIsThePowerLevel"),
                                        ]),
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn empty_for_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    for (;;) {}
                }
            }

        "#, [
            m.node(14, 125, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 111, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 111, Block {
                            body: m.list([
                                m.node(82, 93, ForStatement {
                                    init: None,
                                    test: None,
                                    update: None,
                                    body: m.node(91, 93, Block {
                                        body: NodeList::empty(),
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn do_while_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    do {
                        neverStopStopping;
                    } while (true);
                }
            }

        "#, [
            m.node(14, 197, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 183, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 183, Block {
                            body: m.list([
                                m.node(82, 165, DoWhileStatement {
                                    body: m.node(85, 151, Block {
                                        body: m.list([
                                            m.stmt_expr(111, 128, 129, "neverStopStopping"),
                                        ]),
                                    }),
                                    test: m.node(159, 163, Primitive::Bool(true)),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn break_and_continue_statements() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    while (true) {
                        continue;
                    }

                    for(;;) {
                        break;
                    }
                }
            }

        "#, [
            m.node(14, 268, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 254, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 254, Block {
                            body: m.list([
                                m.node(82, 152, WhileStatement {
                                    test: m.node(89, 93, Primitive::Bool(true)),
                                    body: m.node(95, 152, Block {
                                        body: m.list([
                                            m.node(121, 130, ContinueStatement),
                                        ]),
                                    }),
                                }),
                                m.node(174, 236, ForStatement {
                                    init: None,
                                    test: None,
                                    update: None,
                                    body: m.node(182, 236, Block {
                                        body: m.list([
                                            m.node(208, 214, BreakStatement),
                                        ]),
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn cannot_use_break_or_continue_outside_loops() {
        use parse;

        assert!(parse(r#"

            contract Foo {
                function bar() {
                    continue;
                }
            }

        "#).is_err());

        assert!(parse(r#"

            contract Foo {
                function bar() {
                    break;
                }
            }

        "#).is_err());
    }

    #[test]
    fn return_and_throw_statements() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function bar() {
                    throw;
                    return;
                    return 100;
                }
            }

        "#, [
            m.node(14, 180, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 166, FunctionDefinition {
                        name: m.node(54, 57, "bar"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 166, Block {
                            body: m.list([
                                m.node(82, 88, ThrowStatement),
                                m.node(109, 116, ReturnStatement {
                                    value: None,
                                }),
                                m.node(137, 148, ReturnStatement {
                                    value: m.node(144, 147, Primitive::IntegerNumber("100", NumberUnit::None)),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn variable_definition_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function wow() {
                    uint foo = 10;
                    bool memory bar = true;
                    string storage baz;
                }
            }

        "#, [
            m.node(14, 212, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 198, FunctionDefinition {
                        name: m.node(54, 57, "wow"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 198, Block {
                            body: m.list([
                                m.node(82, 96, VariableDefinitionStatement {
                                    declaration: m.node(82, 90, VariableDeclaration {
                                        type_name: m.node(82, 86, ElementaryTypeName::Uint(32)),
                                        location: None,
                                        id: m.node(87, 90, "foo"),
                                    }),
                                    init: m.node(93, 95, Primitive::IntegerNumber("10", NumberUnit::None)),
                                }),
                                m.node(117, 140, VariableDefinitionStatement {
                                    declaration: m.node(117, 132, VariableDeclaration {
                                        type_name: m.node(117, 121, ElementaryTypeName::Bool),
                                        location: m.node(122, 128, StorageLocation::Memory),
                                        id: m.node(129, 132, "bar"),
                                    }),
                                    init: m.node(135, 139, Primitive::Bool(true)),
                                }),
                                m.node(161, 180, VariableDefinitionStatement {
                                    declaration: m.node(161, 179, VariableDeclaration {
                                        type_name: m.node(161, 167, ElementaryTypeName::String),
                                        location: m.node(168, 175, StorageLocation::Storage),
                                        id: m.node(176, 179, "baz"),
                                    }),
                                    init: None,
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn inferred_definition_statement() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function wow() {
                    var foo = 10;
                    var (a, b, c) = (1, 2, 3);
                    var () = ();
                    var (,,skip) = (4, 5, 6);
                }
            }

        "#, [
            m.node(14, 253, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 239, FunctionDefinition {
                        name: m.node(54, 57, "wow"),
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(60, 239, Block {
                            body: m.list([
                                m.node(82, 95, InferredDefinitionStatement {
                                    ids: m.list([ m.node(86, 89, "foo") ]),
                                    init: m.node(92, 94, Primitive::IntegerNumber("10", NumberUnit::None)),
                                }),
                                m.node(116, 142, InferredDefinitionStatement {
                                    ids: m.list([
                                        m.node(121, 122, "a"),
                                        m.node(124, 125, "b"),
                                        m.node(127, 128, "c"),
                                    ]),
                                    init: m.node(132, 141, TupleExpression {
                                        expressions: m.list([
                                            m.node(133, 134, Primitive::IntegerNumber("1", NumberUnit::None)),
                                            m.node(136, 137, Primitive::IntegerNumber("2", NumberUnit::None)),
                                            m.node(139, 140, Primitive::IntegerNumber("3", NumberUnit::None)),
                                        ])
                                    }),
                                }),
                                m.node(163, 175, InferredDefinitionStatement {
                                    ids: List::empty(),
                                    init: m.node(172, 174, TupleExpression {
                                        expressions: NodeList::empty(),
                                    }),
                                }),
                                m.node(196, 221, InferredDefinitionStatement {
                                    ids: m.list([
                                        None,
                                        None,
                                        m.node(203, 207, "skip"),
                                    ]),
                                    init: m.node(211, 220, TupleExpression {
                                        expressions: m.list([
                                            m.node(212, 213, Primitive::IntegerNumber("4", NumberUnit::None)),
                                            m.node(215, 216, Primitive::IntegerNumber("5", NumberUnit::None)),
                                            m.node(218, 219, Primitive::IntegerNumber("6", NumberUnit::None)),
                                        ])
                                    }),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }
}
