use ast::Function;

pub trait GeneratesCode {
    fn generate_code(&mut self, &Vec<Function>) -> String;
}
