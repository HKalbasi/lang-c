use ast::*;
use span::{Node, Span};

#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub enum Operation {
    Member(Node<MemberOperator>, Node<Identifier>),
    Unary(Node<UnaryOperator>),
    Binary(Node<BinaryOperator>, Node<Expression>),
    Call(Vec<Node<Expression>>),
}

impl Node<Operation> {
    pub fn apply(self, a: Node<Expression>) -> Node<Expression> {
        let span = Span::span(a.span.start, self.span.end);
        let expr = match self.node {
            Operation::Member(op, id) => Expression::Member {
                operator: op,
                expression: Box::new(a),
                identifier: id,
            },
            Operation::Unary(op) => Expression::UnaryOperator {
                operator: op,
                operand: Box::new(a),
            },
            Operation::Binary(op, b) => Expression::BinaryOperator {
                operator: op,
                lhs: Box::new(a),
                rhs: Box::new(b),
            },
            Operation::Call(args) => Expression::Call {
                callee: Box::new(a),
                arguments: args,
            },
        };

        Node::new(expr, span)
    }
}

pub fn apply_ops(ops: Vec<Node<Operation>>, expr: Node<Expression>) -> Node<Expression> {
    ops.into_iter().fold(expr, |a, op| op.apply(a))
}

pub fn concat<T>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.extend(b);
    a
}

pub fn infix(node: Node<()>, op: BinaryOperator, lhs: Node<Expression>, rhs: Node<Expression>) -> Node<Expression> {
    let span = Span::span(lhs.span.start, rhs.span.end);
    Node::new(
        Expression::BinaryOperator {
            operator: Node::new(op, node.span),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        span,
    )
}
