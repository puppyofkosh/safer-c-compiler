use ast::Function;

pub trait GeneratesCode {

    // FIXME: Change this to return a String
    fn generate_code(&mut self, &Vec<Function>);
}
