use ast::VarType;
use ast::VarType::*;


/// Return true if the type is Pointer
pub fn is_pointer(typ: &VarType) -> bool {
    if let Pointer(_, _) = *typ {
        true
    } else {
        false
    }
}
