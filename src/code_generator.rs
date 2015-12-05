use ast::Program;

/// This is an interface
pub trait GeneratesCode {
    fn generate_code(&mut self, &Program) -> String;
}
