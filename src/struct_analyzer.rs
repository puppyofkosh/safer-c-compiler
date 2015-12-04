use ast::StructDefinition;
use ast::VarType;

// Make sure our structs don't have any cycles
// or any other problems

use std::collections::HashSet;

pub struct StructAnalyzer {
    // A struct does not have to do anything to be "declared"
    // All off the structs are always declared
    structs_declared: HashSet<String>,

    // For a struct to be defined, we must have already seen
    // its definition
    // This allows for things like
    // 
    // struct A {
    //     vv B has not been defined but its been declared
    //    pointer(B) b
    // }
    // 
    // struct B {
    //   pointer(A) a
    // }
    //
    //
    
    structs_defined: HashSet<String>,
    errors_found: Vec<String>
}

impl StructAnalyzer {
    pub fn new() -> StructAnalyzer {
        StructAnalyzer {
            structs_declared: HashSet::new(),
            structs_defined: HashSet::new(),
            errors_found: Vec::new(),
        }
    }

    fn is_type_declared(&self, t: &VarType) -> bool {
        match *t {
            VarType::Int | VarType::Char => true,
            VarType::Pointer(ref pointed_type) => 
                self.is_type_declared(pointed_type),
            VarType::Struct(ref struct_name) => 
                self.structs_declared.contains(struct_name),
        }
    }

    fn is_type_defined(&self, t: &VarType) -> bool {
        match *t {
            VarType::Struct(ref struct_name) => 
                self.structs_defined.contains(struct_name),
            _ => self.is_type_declared(t)
        }
    }

    pub fn check_structs(&mut self,
                         structs: &Vec<StructDefinition>) -> bool {
        self.structs_declared = structs.iter()
            .map(|s| s.name.clone())
            .collect();

        for struct_defn in structs {
            for (field, typ) in struct_defn.fields.iter() {
                
                if !self.is_type_defined(typ) {
                    let msg = format!("Unkown type {:?} for field {}",
                                      typ, field);
                    self.errors_found.push(msg);
                    return false;
                }
            }

            self.structs_defined.insert(struct_defn.name.clone());
        }
        true
    }

    pub fn get_errors(&self) -> Vec<String> {
        self.errors_found.clone()
    }
}
