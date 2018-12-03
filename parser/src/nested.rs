use Parser;
use lexer::{Token, Logos, lookup};
use ast::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Precedence(u8);

type HandlerFn = for<'ast> fn(&mut Parser<'ast>, ExpressionNode<'ast>) -> Option<ExpressionNode<'ast>>;

#[derive(Clone, Copy)]
struct NestedHandler(Precedence, HandlerFn);

pub const TOP: Precedence = Precedence(15);
pub const P14: Precedence = Precedence(14);
pub const P13: Precedence = Precedence(13);
pub const P12: Precedence = Precedence(12);
pub const P11: Precedence = Precedence(11);
pub const P10: Precedence = Precedence(10);
pub const P9: Precedence = Precedence(9);
pub const P8: Precedence = Precedence(8);
pub const P7: Precedence = Precedence(7);
pub const P6: Precedence = Precedence(6);
pub const P5: Precedence = Precedence(5);
pub const P4: Precedence = Precedence(4);
pub const P3: Precedence = Precedence(3);
pub const P2: Precedence = Precedence(2);

const INVALID: Precedence = Precedence(100);

static NESTED_LUT: [NestedHandler; Token::SIZE] = lookup! {
    Token::Accessor               => NestedHandler(P2, MEMBER),
    Token::ParenOpen              => NestedHandler(P2, CALL),
    Token::BracketOpen            => NestedHandler(P2, INDEX),
    Token::OperatorIncrement      => NestedHandler(P2, INC),
    Token::OperatorDecrement      => NestedHandler(P2, DEC),
    Token::OperatorExponent       => NestedHandler(P3, EXPONENT),
    Token::OperatorMultiplication => NestedHandler(P4, MUL),
    Token::OperatorDivision       => NestedHandler(P4, DIV),
    Token::OperatorRemainder      => NestedHandler(P4, REMAINDER),
    Token::OperatorAddition       => NestedHandler(P5, ADD),
    Token::OperatorSubtraction    => NestedHandler(P5, SUB),
    Token::OperatorBitShiftLeft   => NestedHandler(P6, BIT_SHIFT_LEFT),
    Token::OperatorBitShiftRight  => NestedHandler(P6, BIT_SHIFT_RIGHT),
    Token::OperatorBitAnd         => NestedHandler(P7, BIT_AND),
    Token::OperatorBitXor         => NestedHandler(P8, BIT_XOR),
    Token::OperatorBitOr          => NestedHandler(P9, BIT_OR),
    Token::OperatorLesser         => NestedHandler(P10, LESSER),
    Token::OperatorLesserEquals   => NestedHandler(P10, LESSER_EQUALITY),
    Token::OperatorGreater        => NestedHandler(P10, GREATER),
    Token::OperatorGreaterEquals  => NestedHandler(P10, GREATER_EQUALITY),
    Token::OperatorEquality       => NestedHandler(P11, EQUALITY),
    Token::OperatorInequality     => NestedHandler(P11, INEQUALITY),
    Token::OperatorLogicalAnd     => NestedHandler(P12, LOGICAL_AND),
    Token::OperatorLogicalOr      => NestedHandler(P13, LOGICAL_OR),
    Token::OperatorConditional    => NestedHandler(P14, CONDITIONAL),
    Token::Assign                 => NestedHandler(TOP, ASSIGN),
    Token::AssignAddition         => NestedHandler(TOP, ASSIGN_ADD),
    Token::AssignSubtraction      => NestedHandler(TOP, ASSIGN_SUB),
    Token::AssignMultiplication   => NestedHandler(TOP, ASSIGN_MUL),
    Token::AssignDivision         => NestedHandler(TOP, ASSIGN_DIV),
    Token::AssignRemainder        => NestedHandler(TOP, ASSIGN_REM),
    Token::AssignBitShiftLeft     => NestedHandler(TOP, ASSIGN_BIT_SHIFT_LEFT),
    Token::AssignBitShiftRight    => NestedHandler(TOP, ASSIGN_BIT_SHIFT_RIGHT),
    Token::AssignBitAnd           => NestedHandler(TOP, ASSIGN_BIT_AND),
    Token::AssignBitXor           => NestedHandler(TOP, ASSIGN_BIT_XOR),
    Token::AssignBitOr            => NestedHandler(TOP, ASSIGN_BIT_OR),
    _                             => NestedHandler(INVALID, |_, _| None),
};

impl NestedHandler {
    #[inline]
    fn get(self, precedence: Precedence) -> Option<HandlerFn> {
        if self.0 <= precedence {
            Some(self.1)
        } else {
            None
        }
    }
}

const CALL: HandlerFn = |par, callee| {
    par.lexer.advance();

    let arguments = par.expression_list();
    let end       = par.expect_end(Token::ParenClose);

    par.node_at(callee.start, end, CallExpression {
        callee,
        arguments,
    })
};

const MEMBER: HandlerFn = |par, object| {
    par.lexer.advance();

    let member = par.expect_str_node(Token::Identifier);

    par.node_at(object.start, member.end, MemberAccessExpression {
        object,
        member,
    })
};

const INDEX: HandlerFn = |par, array| {
    par.lexer.advance();

    let index = par.expression(TOP);
    let end   = par.expect_end(Token::BracketClose);

    par.node_at(array.start, end, IndexAccessExpression {
        array,
        index,
    })
};

const INC: HandlerFn = |par, operand| {
    let operator: Node<_> = par.node_at_token(PostfixOperator::Increment);

    par.node_at(operand.start, operator.end, PostfixExpression {
        operator,
        operand,
    })
};

const DEC: HandlerFn = |par, operand| {
    let operator: Node<_> = par.node_at_token(PostfixOperator::Decrement);

    par.node_at(operand.start, operator.end, PostfixExpression {
        operator,
        operand,
    })
};

const CONDITIONAL: HandlerFn = |par, test| {
    par.lexer.advance();

    let consequent = expect!(par, par.expression(P14));

    par.expect(Token::Colon);

    let alternate = expect!(par, par.expression(P14));

    par.node_at(test.start, alternate.end, ConditionalExpression {
        test,
        consequent,
        alternate,
    })
};

macro_rules! assign {
    ($name:ident => $op:ident) => {
        const $name: HandlerFn = |par, left| {
            // TODO: check if left is LValue

            let operator = par.node_at_token(AssignmentOperator::$op);
            let right    = expect!(par, par.expression(TOP));

            par.node_at(left.start, right.end, AssignmentExpression {
                operator,
                left,
                right,
            })
        };
    }
}

macro_rules! binary {
    ($name:ident, $precedence:ident => $op:ident) => {
        const $name: HandlerFn = |par, left| {
            let operator = par.node_at_token(BinaryOperator::$op);
            let right    = expect!(par, par.expression($precedence));

            par.node_at(left.start, right.end, BinaryExpression {
                operator,
                left,
                right,
            })
        };
    }
}

assign!(ASSIGN                 => Plain);
assign!(ASSIGN_ADD             => Addition);
assign!(ASSIGN_SUB             => Subtraction);
assign!(ASSIGN_MUL             => Multiplication);
assign!(ASSIGN_DIV             => Division);
assign!(ASSIGN_REM             => Remainder);
assign!(ASSIGN_BIT_SHIFT_LEFT  => BitShiftLeft);
assign!(ASSIGN_BIT_SHIFT_RIGHT => BitShiftRight);
assign!(ASSIGN_BIT_AND         => BitAnd);
assign!(ASSIGN_BIT_XOR         => BitXor);
assign!(ASSIGN_BIT_OR          => BitOr);

binary!(LOGICAL_OR       , P13 => LogicalOr);
binary!(LOGICAL_AND      , P12 => LogicalAnd);
binary!(EQUALITY         , P11 => Equality);
binary!(INEQUALITY       , P11 => Inequality);
binary!(LESSER           , P10 => Lesser);
binary!(LESSER_EQUALITY  , P10 => LesserEquals);
binary!(GREATER          , P10 => Greater);
binary!(GREATER_EQUALITY , P10 => GreaterEquals);
binary!(BIT_OR           , P9  => BitOr);
binary!(BIT_XOR          , P8  => BitXor);
binary!(BIT_AND          , P7  => BitAnd);
binary!(BIT_SHIFT_LEFT   , P6  => BitShiftLeft);
binary!(BIT_SHIFT_RIGHT  , P6  => BitShiftRight);
binary!(ADD              , P5  => Addition);
binary!(SUB              , P5  => Subtraction);
binary!(MUL              , P4  => Multiplication);
binary!(DIV              , P4  => Division);
binary!(REMAINDER        , P4  => Remainder);
binary!(EXPONENT         , P3  => Exponent);


impl<'ast> Parser<'ast> {
    #[inline]
    pub fn nested_expression(&mut self, mut left: ExpressionNode<'ast>, precedence: Precedence) -> ExpressionNode<'ast> {
        while let Some(node) = NESTED_LUT[self.lexer.token as usize].get(precedence).and_then(|handler| handler(self, left)) {
            left = node;
        }

        left
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mock::{Mock, assert_units};

    #[test]
    fn nested_expressions() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function() {
                    doge.moon;
                    add(1, 2);
                    things[1];
                    solidity++;
                    solidity--;
                }
            }

        "#, [
            m.node(14, 246, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 232, FunctionDefinition {
                        name: None,
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(56, 232, Block {
                            body: m.list([
                                m.stmt_expr(78, 87, 88, MemberAccessExpression {
                                    object: m.node(78, 82, "doge"),
                                    member: m.node(83, 87, "moon"),
                                }),
                                m.stmt_expr(109, 118, 119, CallExpression {
                                    callee: m.node(109, 112, "add"),
                                    arguments: m.list([
                                        m.node(113, 114, Primitive::IntegerNumber("1", NumberUnit::None)),
                                        m.node(116, 117, Primitive::IntegerNumber("2", NumberUnit::None)),
                                    ]),
                                }),
                                m.stmt_expr(140, 149, 150, IndexAccessExpression {
                                    array: m.node(140, 146, "things"),
                                    index: m.node(147, 148, Primitive::IntegerNumber("1", NumberUnit::None)),
                                }),
                                m.stmt_expr(171, 181, 182, PostfixExpression {
                                    operand: m.node(171, 179, "solidity"),
                                    operator: m.node(179, 181, PostfixOperator::Increment),
                                }),
                                m.stmt_expr(203, 213, 214, PostfixExpression {
                                    operand: m.node(203, 211, "solidity"),
                                    operator: m.node(211, 213, PostfixOperator::Decrement),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn binary_expressions() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function() {
                    a || b;
                    a && b;
                    a == b;
                    a != b;
                    a < b;
                    a <= b;
                    a > b;
                    a >= b;
                    a | b;
                    a ^ b;
                    a & b;
                    a << b;
                    a >> b;
                    a + b;
                    a - b;
                    a * b;
                    a / b;
                    a % b;
                    a ** b;
                }
            }

        "#, [
            m.node(14, 611, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 597, FunctionDefinition {
                        name: None,
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(56, 597, Block {
                            body: m.list([
                                m.stmt_expr(78, 84, 85, BinaryExpression {
                                    left: m.node(78, 79, "a"),
                                    operator: m.node(80, 82, BinaryOperator::LogicalOr),
                                    right: m.node(83, 84, "b"),
                                }),
                                m.stmt_expr(106, 112, 113, BinaryExpression {
                                    left: m.node(106, 107, "a"),
                                    operator: m.node(108, 110, BinaryOperator::LogicalAnd),
                                    right: m.node(111, 112, "b"),
                                }),
                                m.stmt_expr(134, 140, 141, BinaryExpression {
                                    left: m.node(134, 135, "a"),
                                    operator: m.node(136, 138, BinaryOperator::Equality),
                                    right: m.node(139, 140, "b"),
                                }),
                                m.stmt_expr(162, 168, 169, BinaryExpression {
                                    left: m.node(162, 163, "a"),
                                    operator: m.node(164, 166, BinaryOperator::Inequality),
                                    right: m.node(167, 168, "b"),
                                }),
                                m.stmt_expr(190, 195, 196, BinaryExpression {
                                    left: m.node(190, 191, "a"),
                                    operator: m.node(192, 193, BinaryOperator::Lesser),
                                    right: m.node(194, 195, "b"),
                                }),
                                m.stmt_expr(217, 223, 224, BinaryExpression {
                                    left: m.node(217, 218, "a"),
                                    operator: m.node(219, 221, BinaryOperator::LesserEquals),
                                    right: m.node(222, 223, "b"),
                                }),
                                m.stmt_expr(245, 250, 251, BinaryExpression {
                                    left: m.node(245, 246, "a"),
                                    operator: m.node(247, 248, BinaryOperator::Greater),
                                    right: m.node(249, 250, "b"),
                                }),
                                m.stmt_expr(272, 278, 279, BinaryExpression {
                                    left: m.node(272, 273, "a"),
                                    operator: m.node(274, 276, BinaryOperator::GreaterEquals),
                                    right: m.node(277, 278, "b"),
                                }),
                                m.stmt_expr(300, 305, 306, BinaryExpression {
                                    left: m.node(300, 301, "a"),
                                    operator: m.node(302, 303, BinaryOperator::BitOr),
                                    right: m.node(304, 305, "b"),
                                }),
                                m.stmt_expr(327, 332, 333, BinaryExpression {
                                    left: m.node(327, 328, "a"),
                                    operator: m.node(329, 330, BinaryOperator::BitXor),
                                    right: m.node(331, 332, "b"),
                                }),
                                m.stmt_expr(354, 359, 360, BinaryExpression {
                                    left: m.node(354, 355, "a"),
                                    operator: m.node(356, 357, BinaryOperator::BitAnd),
                                    right: m.node(358, 359, "b"),
                                }),
                                m.stmt_expr(381, 387, 388, BinaryExpression {
                                    left: m.node(381, 382, "a"),
                                    operator: m.node(383, 385, BinaryOperator::BitShiftLeft),
                                    right: m.node(386, 387, "b"),
                                }),
                                m.stmt_expr(409, 415, 416, BinaryExpression {
                                    left: m.node(409, 410, "a"),
                                    operator: m.node(411, 413, BinaryOperator::BitShiftRight),
                                    right: m.node(414, 415, "b"),
                                }),
                                m.stmt_expr(437, 442, 443, BinaryExpression {
                                    left: m.node(437, 438, "a"),
                                    operator: m.node(439, 440, BinaryOperator::Addition),
                                    right: m.node(441, 442, "b"),
                                }),
                                m.stmt_expr(464, 469, 470, BinaryExpression {
                                    left: m.node(464, 465, "a"),
                                    operator: m.node(466, 467, BinaryOperator::Subtraction),
                                    right: m.node(468, 469, "b"),
                                }),
                                m.stmt_expr(491, 496, 497, BinaryExpression {
                                    left: m.node(491, 492, "a"),
                                    operator: m.node(493, 494, BinaryOperator::Multiplication),
                                    right: m.node(495, 496, "b"),
                                }),
                                m.stmt_expr(518, 523, 524, BinaryExpression {
                                    left: m.node(518, 519, "a"),
                                    operator: m.node(520, 521, BinaryOperator::Division),
                                    right: m.node(522, 523, "b"),
                                }),
                                m.stmt_expr(545, 550, 551, BinaryExpression {
                                    left: m.node(545, 546, "a"),
                                    operator: m.node(547, 548, BinaryOperator::Remainder),
                                    right: m.node(549, 550, "b"),
                                }),
                                m.stmt_expr(572, 578, 579, BinaryExpression {
                                    left: m.node(572, 573, "a"),
                                    operator: m.node(574, 576, BinaryOperator::Exponent),
                                    right: m.node(577, 578, "b"),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }


    #[test]
    fn assignment_expressions() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function() {
                    a = b;
                    a += b;
                    a -= b;
                    a *= b;
                    a /= b;
                    a %= b;
                    a <<= b;
                    a >>= b;
                    a &= b;
                    a ^= b;
                    a |= b;
                }
            }

        "#, [
            m.node(14, 398, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 384, FunctionDefinition {
                        name: None,
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(56, 384, Block {
                            body: m.list([
                                m.stmt_expr(78, 83, 84, AssignmentExpression {
                                    left: m.node(78, 79, "a"),
                                    operator: m.node(80, 81, AssignmentOperator::Plain),
                                    right: m.node(82, 83, "b"),
                                }),
                                m.stmt_expr(105, 111, 112, AssignmentExpression {
                                    left: m.node(105, 106, "a"),
                                    operator: m.node(107, 109, AssignmentOperator::Addition),
                                    right: m.node(110, 111, "b"),
                                }),
                                m.stmt_expr(133, 139, 140, AssignmentExpression {
                                    left: m.node(133, 134, "a"),
                                    operator: m.node(135, 137, AssignmentOperator::Subtraction),
                                    right: m.node(138, 139, "b"),
                                }),
                                m.stmt_expr(161, 167, 168, AssignmentExpression {
                                    left: m.node(161, 162, "a"),
                                    operator: m.node(163, 165, AssignmentOperator::Multiplication),
                                    right: m.node(166, 167, "b"),
                                }),
                                m.stmt_expr(189, 195, 196, AssignmentExpression {
                                    left: m.node(189, 190, "a"),
                                    operator: m.node(191, 193, AssignmentOperator::Division),
                                    right: m.node(194, 195, "b"),
                                }),
                                m.stmt_expr(217, 223, 224, AssignmentExpression {
                                    left: m.node(217, 218, "a"),
                                    operator: m.node(219, 221, AssignmentOperator::Remainder),
                                    right: m.node(222, 223, "b"),
                                }),
                                m.stmt_expr(245, 252, 253, AssignmentExpression {
                                    left: m.node(245, 246, "a"),
                                    operator: m.node(247, 250, AssignmentOperator::BitShiftLeft),
                                    right: m.node(251, 252, "b"),
                                }),
                                m.stmt_expr(274, 281, 282, AssignmentExpression {
                                    left: m.node(274, 275, "a"),
                                    operator: m.node(276, 279, AssignmentOperator::BitShiftRight),
                                    right: m.node(280, 281, "b"),
                                }),
                                m.stmt_expr(303, 309, 310, AssignmentExpression {
                                    left: m.node(303, 304, "a"),
                                    operator: m.node(305, 307, AssignmentOperator::BitAnd),
                                    right: m.node(308, 309, "b"),
                                }),
                                m.stmt_expr(331, 337, 338, AssignmentExpression {
                                    left: m.node(331, 332, "a"),
                                    operator: m.node(333, 335, AssignmentOperator::BitXor),
                                    right: m.node(336, 337, "b"),
                                }),
                                m.stmt_expr(359, 365, 366, AssignmentExpression {
                                    left: m.node(359, 360, "a"),
                                    operator: m.node(361, 363, AssignmentOperator::BitOr),
                                    right: m.node(364, 365, "b"),
                                }),
                            ]),
                        }),
                    }),
                ]),
            }),
        ]);
    }

    #[test]
    fn operator_precedence() {
        let m = Mock::new();

        assert_units(r#"

            contract Foo {
                function() {
                    uint a = 2 * 2 + 2;
                    uint b = 2 + 2 * 2;
                }
            }

        "#, [
            m.node(14, 169, ContractDefinition {
                name: m.node(23, 26, "Foo"),
                inherits: NodeList::empty(),
                body: m.list([
                    m.node(45, 155, FunctionDefinition {
                        name: None,
                        params: NodeList::empty(),
                        visibility: None,
                        mutability: None,
                        modifiers: NodeList::empty(),
                        returns: NodeList::empty(),
                        block: m.node(56, 155, Block {
                            body: m.list([
                                m.node(78, 97, VariableDefinitionStatement {
                                    declaration: m.node(78, 84, VariableDeclaration {
                                        type_name: m.node(78, 82, ElementaryTypeName::Uint(32)),
                                        location: None,
                                        id: m.node(83, 84, "a"),
                                    }),
                                    init: m.node(87, 96, BinaryExpression {
                                        left: m.node(87, 92, BinaryExpression {
                                            left: m.node(87, 88, Primitive::IntegerNumber("2", NumberUnit::None)),
                                            operator: m.node(89, 90, BinaryOperator::Multiplication),
                                            right: m.node(91, 92, Primitive::IntegerNumber("2", NumberUnit::None)),
                                        }),
                                        operator: m.node(93, 94, BinaryOperator::Addition),
                                        right: m.node(95, 96, Primitive::IntegerNumber("2", NumberUnit::None)),
                                    }),
                                }),
                                m.node(118, 137, VariableDefinitionStatement {
                                    declaration: m.node(118, 124, VariableDeclaration {
                                        type_name: m.node(118, 122, ElementaryTypeName::Uint(32)),
                                        location: None,
                                        id: m.node(123, 124, "b"),
                                    }),
                                    init: m.node(127, 136, BinaryExpression {
                                        left: m.node(127, 128, Primitive::IntegerNumber("2", NumberUnit::None)),
                                        operator: m.node(129, 130, BinaryOperator::Addition),
                                        right: m.node(131, 136, BinaryExpression {
                                            left: m.node(131, 132, Primitive::IntegerNumber("2", NumberUnit::None)),
                                            operator: m.node(133, 134, BinaryOperator::Multiplication),
                                            right: m.node(135, 136, Primitive::IntegerNumber("2", NumberUnit::None)),
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
}
