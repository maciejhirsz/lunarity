use toolshed::list::{ListBuilder, GrowableList};

use ast::*;
use {Parser, ModifierContext, TopPrecedence, RegularTypeNameContext};
use lexer::Token;

impl<'ast> Parser<'ast> {
    pub fn contract_definition(&mut self) -> Option<SourceUnitNode<'ast>> {
        let start = self.start_then_advance();
        let name = self.expect_str_node(Token::Identifier);

        let inherits = if self.allow(Token::KeywordIs) {
            let builder = ListBuilder::new(self.arena, self.expect_str_node(Token::Identifier));

            while self.allow(Token::Comma) {
                builder.push(self.arena, self.expect_str_node(Token::Identifier));
            }

            builder.as_list()
        } else {
            NodeList::empty()
        };

        self.expect(Token::BraceOpen);

        let builder = GrowableList::new();

        while let Some(part) = self.contract_part() {
            builder.push(self.arena, part);
        }

        let end = self.expect_end(Token::BraceClose);

        self.node_at(start, end, ContractDefinition {
            name,
            inherits,
            body: builder.as_list(),
        })
    }

    fn contract_part(&mut self) -> Option<ContractPartNode<'ast>> {
        match self.lexer.token {
            Token::KeywordUsing        => self.using_for_declaration(),
            Token::DeclarationStruct   => self.struct_defintion(),
            Token::DeclarationModifier => self.modifier_definition(),
            Token::DeclarationFunction => self.function_definition(),
            Token::DeclarationEvent    => self.event_definition(),
            Token::DeclarationEnum     => self.enum_definition(),
            _                          => self.state_variable_declaration(),
        }

    }

    fn state_variable_declaration(&mut self) -> Option<ContractPartNode<'ast>> {
        let type_name = self.type_name::<RegularTypeNameContext>()?;

        let mut visibility = None;
        let mut constant = None;

        for _ in 0..2 {
            match self.lexer.token {
                Token::KeywordPublic   => self.unique_flag(&mut visibility, StateVariableVisibility::Public),
                Token::KeywordInternal => self.unique_flag(&mut visibility, StateVariableVisibility::Internal),
                Token::KeywordPrivate  => self.unique_flag(&mut visibility, StateVariableVisibility::Private),
                Token::KeywordConstant => self.unique_flag(&mut constant, Flag),
                _                      => break,
            }
        }

        let name = self.expect_str_node(Token::Identifier);

        let init = if self.allow(Token::Assign) {
            match self.expression::<TopPrecedence>() {
                None => {
                    self.error();

                    None
                },
                init => init,
            }
        } else {
            None
        };

        let end = self.expect_end(Token::Semicolon);

        self.node_at(type_name.start, end, StateVariableDeclaration {
            type_name,
            visibility,
            constant,
            name,
            init,
        })
    }

    fn using_for_declaration(&mut self) -> Option<ContractPartNode<'ast>> {
        let start = self.start_then_advance();
        let id    = self.expect_str_node(Token::Identifier);

        self.expect(Token::KeywordFor);

        let type_name = match self.type_name::<RegularTypeNameContext>() {
            None => {
                self.expect(Token::OperatorMultiplication);

                None
            },
            type_name => type_name,
        };

        let end = self.expect_end(Token::Semicolon);

        self.node_at(start, end, UsingForDeclaration {
            id,
            type_name,
        })
    }

    fn struct_defintion(&mut self) -> Option<ContractPartNode<'ast>> {
        let start = self.start_then_advance();
        let name  = self.expect_str_node(Token::Identifier);

        self.expect(Token::BraceOpen);

        let body = match self.variable_declaration::<RegularTypeNameContext>() {
            Some(declaration) => {
                let builder = ListBuilder::new(self.arena, declaration);

                self.expect(Token::Semicolon);

                while let Some(declaration) = self.variable_declaration::<RegularTypeNameContext>() {
                    builder.push(self.arena, declaration);

                    self.expect(Token::Semicolon);
                }

                builder.as_list()
            },
            None => {
                // Must have at least one element
                self.error();

                NodeList::empty()
            }
        };

        let end = self.expect_end(Token::BraceClose);

        self.node_at(start, end, StructDefinition {
            name,
            body,
        })
    }

    fn modifier_definition(&mut self) -> Option<ContractPartNode<'ast>> {
        let start = self.start_then_advance();
        let name  = self.expect_str_node(Token::Identifier);

        let params;

        if self.allow(Token::ParenOpen) {
            params = self.parameter_list();

            self.expect(Token::ParenClose);
        } else {
            params = NodeList::empty()
        }

        let block = self.block::<ModifierContext, _>();

        self.node_at(start, block.end, ModifierDefinition {
            name,
            params,
            block,
        })
    }

    fn event_definition(&mut self) -> Option<ContractPartNode<'ast>> {
        let start  = self.start_then_advance();
        let name   = self.expect_str_node(Token::Identifier);

        self.expect(Token::ParenOpen);

        let params = match self.indexed_parameter() {
            Some(param) => {
                let builder = ListBuilder::new(self.arena, param);

                while self.allow(Token::Comma) {
                    match self.indexed_parameter() {
                        Some(param) => builder.push(self.arena, param),
                        None        => self.error(),
                    }
                }

                builder.as_list()
            },
            None => NodeList::empty(),
        };

        self.expect(Token::ParenClose);

        let anonymous = self.allow_flag_node(Token::KeywordAnonymous);
        let end       = self.expect_end(Token::Semicolon);

        self.node_at(start, end, EventDefinition {
            anonymous,
            name,
            params,
        })
    }

    fn indexed_parameter(&mut self) -> Option<Node<'ast, IndexedParameter<'ast>>> {
        let type_name = self.type_name::<RegularTypeNameContext>()?;
        let indexed   = self.allow_flag_node(Token::KeywordIndexed);
        let name      = self.allow_str_node(Token::Identifier);

        let end = name.end()
                      .or_else(|| indexed.end())
                      .unwrap_or_else(|| type_name.end);

        self.node_at(type_name.start, end, IndexedParameter {
            indexed,
            type_name,
            name,
        })
    }

    fn enum_definition(&mut self) -> Option<ContractPartNode<'ast>> {
        let start = self.start_then_advance();
        let name  = self.expect_str_node(Token::Identifier);

        self.expect(Token::BraceOpen);

        let variants = if let Some(variant) = self.allow_str_node(Token::Identifier) {
            let builder = ListBuilder::new(self.arena, variant);

            while self.allow(Token::Comma) {
                builder.push(self.arena, self.expect_str_node(Token::Identifier))
            }

            builder.as_list()
        } else {
            NodeList::empty()
        };

        let end = self.expect_end(Token::BraceClose);

        self.node_at(start, end, EnumDefinition {
            name,
            variants,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mock::{Mock, assert_units};

    #[test]
    fn empty_contract() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {}
            contract Doge is Amazing {}
            contract This is Silly, Kinda {}

        "#, [
            m.node(14, 29, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: NodeList::empty(),
            }),
            m.node(42, 69, ContractDefinition {
                name: m.node(51, 55, "Doge"),
                inherits: m.list([
                    m.node(59, 66, "Amazing"),
                ]),
                body: NodeList::empty(),
            }),
            m.node(82, 114, ContractDefinition {
                name: m.node(91, 95, "This"),
                inherits: m.list([
                    m.node(99, 104, "Silly"),
                    m.node(106, 111, "Kinda"),
                ]),
                body: NodeList::empty(),
            }),
        ]);
    }

    #[test]
    fn state_variable_declaration() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                int32 foo = 10;
                bytes10 public doge;
            }

        "#, [
            m.node(14, 111, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 60, StateVariableDeclaration {
                        type_name: m.node(45, 50, ElementaryTypeName::Int(4)),
                        visibility: None,
                        constant: None,
                        name: m.node(51, 54, "foo"),
                        init: m.node(57, 59, Primitive::IntegerNumber("10", NumberUnit::None)),
                    }),
                    m.node(77, 97, StateVariableDeclaration {
                        type_name: m.node(77, 84, ElementaryTypeName::Byte(10)),
                        visibility: m.node(85, 91, StateVariableVisibility::Public),
                        constant: None,
                        name: m.node(92, 96, "doge"),
                        init: None,
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn using_for_declaration() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                using foo for *;
                using bar for int32;
            }

        "#, [
            m.node(14, 112, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 61, UsingForDeclaration {
                        id: m.node(51, 54, "foo"),
                        type_name: None,
                    }),
                    m.node(78, 98, UsingForDeclaration {
                        id: m.node(84, 87, "bar"),
                        type_name: m.node(92, 97, ElementaryTypeName::Int(4)),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn struct_defintion() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                struct Doge {
                    uint wows;
                    bool memory amaze;
                    address storage moon;
                }
            }

        "#, [
            m.node(14, 202, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 188, StructDefinition {
                        name: m.node(52, 56, "Doge"),
                        body: m.list([
                            m.node(79, 88, VariableDeclaration {
                                type_name: m.node(79, 83, ElementaryTypeName::Uint(32)),
                                location: None,
                                id: m.node(84, 88, "wows"),
                            }),
                            m.node(110, 127, VariableDeclaration {
                                type_name: m.node(110, 114, ElementaryTypeName::Bool),
                                location: m.node(115, 121, StorageLocation::Memory),
                                id: m.node(122, 127, "amaze"),
                            }),
                            m.node(149, 169, VariableDeclaration {
                                type_name: m.node(149, 156, ElementaryTypeName::Address),
                                location: m.node(157, 164, StorageLocation::Storage),
                                id: m.node(165, 169, "moon"),
                            }),
                        ])
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn modifier_definition() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                modifier only_doges { _; }

                modifier foo(uint8 bar) {
                    uint8 baz = bar;
                    _;
                }
            }

        "#, [
            m.node(14, 206, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 71, ModifierDefinition {
                        name: m.node(54, 64, "only_doges"),
                        params: NodeList::empty(),
                        block: m.node(65, 71, Block {
                            body: m.list([
                                m.node(67, 69, Statement::Placeholder),
                            ]),
                        }),
                    }),
                    m.node(89, 192, ModifierDefinition {
                        name: m.node(98, 101, "foo"),
                        params: m.list([
                            m.node(102, 111, Parameter {
                                type_name: m.node(102, 107, ElementaryTypeName::Uint(1)),
                                name: m.node(108, 111, "bar"),
                            }),
                        ]),
                        block: m.node(113, 192, Block {
                            body: m.list([
                                m.node(135, 151, VariableDefinitionStatement {
                                    declaration: m.node(135, 144, VariableDeclaration {
                                        type_name: m.node(135, 140, ElementaryTypeName::Uint(1)),
                                        location: None,
                                        id: m.node(141, 144, "baz"),
                                    }),
                                    init: m.node(147, 150, "bar"),
                                }),
                                m.node(172, 174, Statement::Placeholder),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn empty_events() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                event Horizon();
                event Alcoholics() anonymous;
            }

        "#, [
            m.node(14, 121, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 61, EventDefinition {
                        anonymous: None,
                        name: m.node(51, 58, "Horizon"),
                        params: NodeList::empty(),
                    }),
                    m.node(78, 107, EventDefinition {
                        anonymous: m.node(97, 106, Flag),
                        name: m.node(84, 94, "Alcoholics"),
                        params: NodeList::empty(),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn event_with_parameters() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                event Horizon(int32 indexed, bool);
            }

        "#, [
            m.node(14, 94, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 80, EventDefinition {
                        anonymous: None,
                        name: m.node(51, 58, "Horizon"),
                        params: m.list([
                            m.node(59, 72, IndexedParameter {
                                type_name: m.node(59, 64, ElementaryTypeName::Int(4)),
                                indexed: m.node(65, 72, Flag),
                                name: None,
                            }),
                            m.node(74, 78, IndexedParameter {
                                indexed: None,
                                type_name: m.node(74, 78, ElementaryTypeName::Bool),
                                name: None,
                            }),
                        ]),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn event_with_named_parameters() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                event Horizon(int32 indexed foo, bool bar);
            }

        "#, [
            m.node(14, 102, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 88, EventDefinition {
                        anonymous: None,
                        name: m.node(51, 58, "Horizon"),
                        params: m.list([
                            m.node(59, 76, IndexedParameter {
                                type_name: m.node(59, 64, ElementaryTypeName::Int(4)),
                                indexed: m.node(65, 72, Flag),
                                name: m.node(73, 76, "foo"),
                            }),
                            m.node(78, 86, IndexedParameter {
                                indexed: None,
                                type_name: m.node(78, 82, ElementaryTypeName::Bool),
                                name: m.node(83, 86, "bar"),
                            }),
                        ]),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn enum_definition() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                enum Empty {}
                enum Doge { To, The, Moon }
            }

        "#, [
            m.node(14, 116, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 58, EnumDefinition {
                        name: m.node(50, 55, "Empty"),
                        variants: NodeList::empty(),
                    }),
                    m.node(75, 102, EnumDefinition {
                        name: m.node(80, 84, "Doge"),
                        variants: m.list([
                            m.node(87, 89, "To"),
                            m.node(91, 94, "The"),
                            m.node(96, 100, "Moon"),
                        ])
                    }),
                ]),
            }),
        ]);
    }
}
