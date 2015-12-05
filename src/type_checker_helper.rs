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
    if op != BinaryOp::Plus && op != BinaryOp::Minus 
        && op != BinaryOp::CompareEqual && op != BinaryOp::CompareNotEqual {
        return false;
    }

    return (is_pointer(&l) && type_contains(&Int, &r)) ||
        (type_contains(&Int, &l) && is_pointer(&r));
}

// Return true if the expression represents something that has an address
// in memory (is an "lvalue")
pub fn expression_has_address(expr_node: &AstExpressionNode) -> bool {
    match expr_node.expr {
        Expression::Variable(_) => true,
        Expression::Dereference(_) => true,
        Expression::FieldAccess(_,_) => true,
        _ => false
    }
}

pub fn can_assign_expr_to_type(left_t: &VarType,
                               right: &AstExpressionNode) -> bool {
    if !right.typ.is_some() {
        return false;
    }

    if let &Struct(_) = left_t {
        // Can't assign structs to one another
        return false;
    } 
    
    if !type_contains(left_t, right.typ.as_ref().unwrap()) {
        // Special case: left is a pointer and right is 0
        // it is okay to assign 0 to a pointer
        let mut res = false;
        if let &Pointer(_) = left_t {
            if let &Expression::Value(val) = &right.expr {
                res = val == 0;
            }
        }

        if !res {
            return false;
        }
    }

    true
}

pub fn is_assignment_valid(left: &AstExpressionNode,
                           right: &AstExpressionNode) -> bool {
    if !left.typ.is_some() || !right.typ.is_some() {
        return false;
    }
    
    if !expression_has_address(left) {
        return false;
    }

    can_assign_expr_to_type(left.typ.as_ref().unwrap(), right)
}
