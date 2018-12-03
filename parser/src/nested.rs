use Parser;
use lexer::{Token, Logos, lookup};
use ast::*;

type NestedHandler = Option<for<'ast> fn(&mut Parser<'ast>, ExpressionNode<'ast>) -> Option<ExpressionNode<'ast>>>;

pub trait Precedence {
    const LUT: [NestedHandler; Token::SIZE];
}

pub struct TopPrecedence;
pub struct Precedence14;
pub struct Precedence13;
pub struct Precedence12;
pub struct Precedence11;
pub struct Precedence10;
pub struct Precedence9;
pub struct Precedence8;
pub struct Precedence7;
pub struct Precedence6;
pub struct Precedence5;
pub struct Precedence4;
pub struct Precedence3;
pub struct Precedence2;

impl Precedence for TopPrecedence {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorEquality => EQ,
        Token::OperatorInequality => INEQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        Token::OperatorLogicalAnd => AND,
        Token::OperatorLogicalOr => OR,
        Token::OperatorConditional => COND,
        Token::Assign => ASGN,
        Token::AssignAddition => A_ADD,
        Token::AssignSubtraction => A_SUB,
        Token::AssignMultiplication => A_MUL,
        Token::AssignDivision => A_DIV,
        Token::AssignRemainder => A_REM,
        Token::AssignBitShiftLeft => A_BSL,
        Token::AssignBitShiftRight => A_BSR,
        Token::AssignBitAnd => A_BAN,
        Token::AssignBitXor => A_XOR,
        Token::AssignBitOr => A_BOR,
        _ => None,
    };
}

impl Precedence for Precedence14 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorEquality => EQ,
        Token::OperatorInequality => INEQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        Token::OperatorLogicalAnd => AND,
        Token::OperatorLogicalOr => OR,
        Token::OperatorConditional => COND,
        _ => None,
    };
}

impl Precedence for Precedence13 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorEquality => EQ,
        Token::OperatorInequality => INEQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        Token::OperatorLogicalAnd => AND,
        Token::OperatorLogicalOr => OR,
        _ => None,
    };
}

impl Precedence for Precedence12 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorEquality => EQ,
        Token::OperatorInequality => INEQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        Token::OperatorLogicalAnd => AND,
        _ => None,
    };
}

impl Precedence for Precedence11 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorEquality => EQ,
        Token::OperatorInequality => INEQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        _ => None,
    };
}

impl Precedence for Precedence10 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorLesser => LESS,
        Token::OperatorLesserEquals => LSEQ,
        Token::OperatorGreater => GRTR,
        Token::OperatorGreaterEquals => GREQ,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        _ => None,
    };
}

impl Precedence for Precedence9 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        Token::OperatorBitOr => B_OR,
        _ => None,
    };
}

impl Precedence for Precedence8 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorBitAnd => B_AND,
        Token::OperatorBitXor => B_XOR,
        _ => None,
    };
}

impl Precedence for Precedence7 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        Token::OperatorBitAnd => B_AND,
        _ => None,
    };
}

impl Precedence for Precedence6 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        Token::OperatorBitShiftLeft => BSL,
        Token::OperatorBitShiftRight => BSR,
        _ => None,
    };
}

impl Precedence for Precedence5 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        Token::OperatorAddition => ADD,
        Token::OperatorSubtraction => SUB,
        _ => None,
    };
}

impl Precedence for Precedence4 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorMultiplication => MUL,
        Token::OperatorDivision => DIV,
        Token::OperatorRemainder => REM,
        Token::OperatorExponent => EXPN,
        _ => None,
    };
}

impl Precedence for Precedence3 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        Token::OperatorExponent => EXPN,
        _ => None,
    };
}

impl Precedence for Precedence2 {
    const LUT: [NestedHandler; Token::SIZE] = lookup! {
        Token::Accessor => MEMBR,
        Token::ParenOpen => CALL,
        Token::BracketOpen => INDEX,
        Token::OperatorIncrement => INC,
        Token::OperatorDecrement => DEC,
        _ => None,
    };
}

const CALL: NestedHandler = Some(|par, callee| {
    par.lexer.advance();

    let arguments = par.expression_list();
    let end       = par.expect_end(Token::ParenClose);

    par.node_at(callee.start, end, CallExpression {
        callee,
        arguments,
    })
});

const MEMBR: NestedHandler = Some(|par, object| {
    par.lexer.advance();

    let member = par.expect_str_node(Token::Identifier);

    par.node_at(object.start, member.end, MemberAccessExpression {
        object,
        member,
    })
});

const INDEX: NestedHandler = Some(|par, array| {
    par.lexer.advance();

    let index = par.expression::<TopPrecedence>();
    let end   = par.expect_end(Token::BracketClose);

    par.node_at(array.start, end, IndexAccessExpression {
        array,
        index,
    })
});

const INC: NestedHandler = Some(|par, operand| {
    let operator: Node<_> = par.node_at_token(PostfixOperator::Increment);

    par.node_at(operand.start, operator.end, PostfixExpression {
        operator,
        operand,
    })
});

const DEC: NestedHandler = Some(|par, operand| {
    let operator: Node<_> = par.node_at_token(PostfixOperator::Decrement);

    par.node_at(operand.start, operator.end, PostfixExpression {
        operator,
        operand,
    })
});

const COND: NestedHandler = Some(|par, test| {
    par.lexer.advance();

    let consequent = expect!(par, par.expression::<Precedence14>());

    par.expect(Token::Colon);

    let alternate = expect!(par, par.expression::<Precedence14>());

    par.node_at(test.start, alternate.end, ConditionalExpression {
        test,
        consequent,
        alternate,
    })
});


macro_rules! assign {
    ($name:ident => $op:ident) => {
        const $name: NestedHandler = {
            fn handler<'ast>(par: &mut Parser<'ast>, left: ExpressionNode<'ast>) -> Option<ExpressionNode<'ast>> {
                // TODO: check if left is LValue

                let operator = par.node_at_token(AssignmentOperator::$op);
                let right    = expect!(par, par.expression::<TopPrecedence>());

                par.node_at(left.start, right.end, AssignmentExpression {
                    operator,
                    left,
                    right,
                })
            }

            Some(handler)
        };
    }
}

macro_rules! binary {
    ($name:ident, $precedence:ident => $op:ident) => {
        const $name: NestedHandler = {
            fn handler<'ast>(par: &mut Parser<'ast>, left: ExpressionNode<'ast>) -> Option<ExpressionNode<'ast>> {
                let operator = par.node_at_token(BinaryOperator::$op);
                let right    = expect!(par, par.expression::<$precedence>());

                par.node_at(left.start, right.end, BinaryExpression {
                    operator,
                    left,
                    right,
                })
            }

            Some(handler)
        };
    }
}

assign!(ASGN  => Plain);
assign!(A_ADD => Addition);
assign!(A_SUB => Subtraction);
assign!(A_MUL => Multiplication);
assign!(A_DIV => Division);
assign!(A_REM => Remainder);
assign!(A_BSL => BitShiftLeft);
assign!(A_BSR => BitShiftRight);
assign!(A_BAN => BitAnd);
assign!(A_XOR => BitXor);
assign!(A_BOR => BitOr);

binary!(OR    , Precedence13 => LogicalOr);
binary!(AND   , Precedence12 => LogicalAnd);
binary!(EQ    , Precedence11 => Equality);
binary!(INEQ  , Precedence11 => Inequality);
binary!(LESS  , Precedence10 => Lesser);
binary!(LSEQ  , Precedence10 => LesserEquals);
binary!(GRTR  , Precedence10 => Greater);
binary!(GREQ  , Precedence10 => GreaterEquals);
binary!(B_OR  , Precedence9  => BitOr);
binary!(B_XOR , Precedence8  => BitXor);
binary!(B_AND , Precedence7  => BitAnd);
binary!(BSL   , Precedence6  => BitShiftLeft);
binary!(BSR   , Precedence6  => BitShiftRight);
binary!(ADD   , Precedence5  => Addition);
binary!(SUB   , Precedence5  => Subtraction);
binary!(MUL   , Precedence4  => Multiplication);
binary!(DIV   , Precedence4  => Division);
binary!(REM   , Precedence4  => Remainder);
binary!(EXPN  , Precedence3  => Exponent);


impl<'ast> Parser<'ast> {
    #[inline]
    pub fn nested_expression<P>(&mut self, mut left: ExpressionNode<'ast>) -> ExpressionNode<'ast>
    where
        P: Precedence,
    {
        // static LUT: [NestedHandler; Token::SIZE] = P::LUT;

        while let Some(node) = P::LUT[self.lexer.token as usize].and_then(|handler| handler(self, left)) {
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
