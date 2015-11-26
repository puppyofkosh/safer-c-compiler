use std::collections::HashSet;

pub struct CodeBlock {
    pub declared_variables: HashSet<String>,
}

impl CodeBlock {
    pub fn new() -> CodeBlock {
        CodeBlock {
            declared_variables: HashSet::new(),
        }
    }
}
