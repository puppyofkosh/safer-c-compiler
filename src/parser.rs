use ast;

pub fn parse_return(tokens: &[String]) -> ast::Statement {
    assert_eq!(tokens[0], "return");
    let val = tokens[1].parse::<i32>().ok().expect("Not a number!?");
    return ast::Statement::Return{val: val};
}
