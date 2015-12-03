use ast::BinaryOp;
use ast::VarType;
use ast::VarType::*;

use ast::Expression;
use ast::AstExpressionNode;

pub fn type_contains(parent: &VarType, child: &VarType) -> bool {
    parent == child || (*parent == Int && *child == Char)
}

/// Return true if the type is Pointer
pub fn is_pointer(typ: &VarType) -> bool {
    if let Pointer(_) = *typ {
        true
    } else {
        false
    }
}

pub fn is_pointer_arithmetic(l: &VarType,
                         r: &VarType, op: BinaryOp) -> bool {
    if op != BinaryOp::Plus && op != BinaryOp::Minus {
        return false;
    }

    return (is_pointer(&l) && type_contains(&Int, &r)) ||
        (type_contains(&Int, &l) && is_pointer(&r));
}

pub fn is_expression_assignable(expr_node: &AstExpressionNode) -> bool {
    match expr_node.expr {
        Expression::Variable(_) => true,
        Expression::Dereference(_) => true,
        _ => false
    }
}
