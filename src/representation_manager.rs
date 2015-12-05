use ast::StructDefinition;
use ast::VarType;

use assembly::MachineType;

use assembly_helper::get_mtype_size;

use std::collections::HashMap;

pub struct FieldInfo {
    pub typ: VarType,
    pub machine_type: MachineType,
    pub offset: i32,
}

struct StructRepresentation {
    field_to_info: HashMap<String, FieldInfo>,
    size: i32,
}

pub struct RepresentationManager {
    struct_to_representation: HashMap<String, StructRepresentation>
}

impl RepresentationManager {
    pub fn new() -> RepresentationManager {
        RepresentationManager {
            struct_to_representation: HashMap::new()
        }
    }

    fn get_struct_representation(&self,
                                 defn: &StructDefinition)
                                 -> StructRepresentation {
        let mut offset = 0;
        let mut field_to_info = HashMap::new();

        for (field, typ) in defn.fields.iter() {
            let info = FieldInfo {
                offset: offset,
                typ: typ.clone(),
                machine_type: self.get_machine_type(typ),
            };

            field_to_info.insert(field.clone(), info);
            
            // TODO:
            // Here we might want to do some sort alignment.
            offset += self.get_type_size(typ);
        }

        StructRepresentation{ field_to_info: field_to_info,
                              size: offset,
        }
    }

    pub fn init(&mut self,
                struct_definitions: &Vec<StructDefinition>) {
        // We should never initialize this more than once
        assert!(self.struct_to_representation.is_empty());
        
        // Build a representation for each struct
        for defn in struct_definitions {
            let r = self.get_struct_representation(defn);
            self.struct_to_representation.insert(defn.name.clone(), r);
        }
    }

    fn get_type_size(&self,
                     typ: &VarType) -> i32 {
        match *typ {
            VarType::Struct(ref name) => {
                self.struct_to_representation.get(name)
                    .expect("struct definitions in wrong order")
                    .size
            }
            _ => get_mtype_size(self.get_machine_type(typ)),
        }
    }

    pub fn get_machine_type(&self, typ: &VarType) -> MachineType {
        match *typ {
            VarType::Pointer(_, _) => MachineType::Long,
            VarType::Int => MachineType::Long,
            VarType::Char => MachineType::Byte,
            VarType::Struct(ref name) => {
                let repr = self.struct_to_representation.get(name);
                MachineType::Chunk(repr.unwrap().size)
            }
        }
    }

    pub fn get_field_info(&self,
                          struct_type: &VarType,
                          field_name: &str) -> &FieldInfo {
        let struct_name = get_struct_name(struct_type);

        self.struct_to_representation
            .get(struct_name)
            .expect("Unkown struct being used")
            .field_to_info
            .get(field_name)
            .expect("Unkown field being used")
    }


    // todo: get_field_offset
    // get_field_type
}

fn get_struct_name(typ: &VarType) -> &String {
    if let &VarType::Struct(ref n) = typ {
        return n;
    } else {
        panic!("Not a struct!");
    }
}
