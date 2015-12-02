use assembly_helper;
use ast::StructDefinition;
use ast::VarType;

use assembly::MachineType;

use assembly_helper::WORD_SIZE;

use std::collections::HashMap;

pub struct FieldInfo {
    pub typ: VarType,
    pub offset: i32,
}

struct StructRepresentation {
    field_to_info: HashMap<String, FieldInfo>,
    size: i32,
}

pub struct RepresentationManager {
    struct_to_representation: HashMap<String, StructRepresentation>
}

fn get_type_size_with_map(typ: &VarType,
                          representations:
                          &HashMap<String,
                                   StructRepresentation>) -> i32 {
    match *typ {
        VarType::Pointer(_) => WORD_SIZE,
        VarType::Int => WORD_SIZE,
        VarType::Char => 1,
        VarType::Struct(ref name) => {
            representations.get(name)
                .expect("struct definitions in wrong order")
                .size
        }
    }
}

fn get_struct_representation(defn: &StructDefinition,
                             other_represenations:
                             &HashMap<String, StructRepresentation>)
                             -> StructRepresentation {
    let mut offset = 0;
    let mut field_to_info = HashMap::new();

    for (field, typ) in defn.fields.iter() {
        let info = FieldInfo {
            offset: offset,
            typ: typ.clone()
        };

        field_to_info.insert(field.clone(), info);
        
        // TODO:
        // Here we might want to do some sort alignment.
        offset += get_type_size_with_map(typ, other_represenations);
    }

    StructRepresentation{ field_to_info: field_to_info,
                          size: offset,
    }
}

impl RepresentationManager {
    pub fn new() -> RepresentationManager {
        RepresentationManager {
            struct_to_representation: HashMap::new()
        }
    }

    pub fn init(&mut self,
                struct_definitions: &Vec<StructDefinition>) {
        
        // Build a representation for each struct
        let mut struct_to_representation = HashMap::new();
        for defn in struct_definitions {
            let repr = get_struct_representation(defn,
                                                 &struct_to_representation);
            struct_to_representation.insert(defn.name.clone(), repr);
        }
        self.struct_to_representation = struct_to_representation;
    }

    pub fn get_machine_type(&self, typ: &VarType) -> MachineType {
        match *typ {
            VarType::Pointer(_) => MachineType::Long,
            VarType::Int => MachineType::Long,
            VarType::Char => MachineType::Byte,
            VarType::Struct(ref name) => {
                let repr = self.struct_to_representation.get(name);
                MachineType::Chunk(repr.unwrap().size)
            }
        }
    }

    pub fn get_type_size(&self, typ: &VarType) -> i32 {
        get_type_size_with_map(typ, &self.struct_to_representation)
    }

    pub fn get_field_info(&self,
                          struct_name: &str, field_name: &str) -> &FieldInfo {
        self.struct_to_representation
            .get(struct_name)
            .expect("Unkown struct being used")
            .field_to_info
            .get(field_name)
            .expect("Unkown field being used")
    }

    pub fn get_field_offset(&self,
                            struct_name: &str, field_name: &str) -> i32 {
        self.get_field_info(struct_name, field_name).offset
    }

    pub fn get_field_var_type(&self,
                          struct_name: &str, field_name: &str) -> VarType {
        self.get_field_info(struct_name, field_name).typ.clone()
    }

    // todo: get_field_offset
    // get_field_type
}
