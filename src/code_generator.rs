use ast::Program;

pub trait GeneratesCode {
    fn generate_code(&mut self, &Program) -> String;
}
