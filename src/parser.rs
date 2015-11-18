use ast::FunctionCall;
use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use ast::Function;
use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;

// FIXME: Do we really want to do this?
fn optype_to_op(op: &OperatorType) -> BinaryOp {
    match *op {
        OperatorType::Plus => BinaryOp::Plus,
        OperatorType::Minus => BinaryOp::Minus,
        OperatorType::Star => BinaryOp::Multiply,
        OperatorType::Divide => BinaryOp::Divide,
        OperatorType::CompareEqual => BinaryOp::CompareEqual,
        OperatorType::CompareGreater => BinaryOp::CompareGreater,
        OperatorType::CompareLess => BinaryOp::CompareLess,
        OperatorType::CompareGreaterOrEqual => BinaryOp::CompareGreaterOrEqual,
        OperatorType::CompareLessOrEqual => BinaryOp::CompareLessOrEqual,
        OperatorType::CompareNotEqual => BinaryOp::CompareNotEqual,
        OperatorType::Assign => panic!("Illegal operator"),
    }
}

fn get_precedence(op: &OperatorType) -> i32 {
    match *op {
        OperatorType::Plus => 0,
        OperatorType::Minus => 0,
        OperatorType::CompareEqual => 1,
        OperatorType::CompareGreater => 1,
        OperatorType::CompareLess => 1,
        OperatorType::CompareGreaterOrEqual => 1,
        OperatorType::CompareLessOrEqual => 1,
        OperatorType::CompareNotEqual => 1,
        OperatorType::Star => 2,
        OperatorType::Divide => 2,
        OperatorType::Assign => panic!("Illegal op")
    }
}

fn two_stack_algo(tokens: &mut TokenStream) -> Expression {
    let mut operator_stack = Vec::new();
    let mut output = Vec::new();

    // Keep track of number of parens. Sometimes we will have an expression
    // followed by a parenthese like call((abc + bcd)), and the last paren is
    // not part of the expression.
    let mut num_left_parens = 0;
    let mut num_right_parens = 0;

    while !tokens.is_empty() {
        let tok = tokens.consume();
        
        match tok {
            Lexeme::Identifier(name) => output.push(Expression::Variable(name.clone())),
            Lexeme::IntConstant(v) => output.push(Expression::Value(v)),
            Lexeme::StringConstant(s) => output.push(Expression::StringValue(s.clone())),
            Lexeme::Call => {
                tokens.push(tok);
                output.push(Expression::Call(parse_call(tokens)));
            }
            Lexeme::Operator(o1) => {
                while let Some(Lexeme::Operator(o2)) = operator_stack.pop() {
                    if get_precedence(&o1) <= get_precedence(&o2) {
                        let r = output.pop().unwrap();
                        let l = output.pop().unwrap();
                        output.push(Expression::BinaryOp(optype_to_op(&o2),
                                                         Box::new(l), Box::new(r)));
                    }
                    else {
                        // push it back on the stack
                        operator_stack.push(Lexeme::Operator(o2));
                        break;
                    }
                }
                operator_stack.push(Lexeme::Operator(o1));
            }
            Lexeme::LParen => {
                operator_stack.push(tok);
                num_left_parens += 1;
            }
            Lexeme::RParen => {
                // Either the parens are mismatched, or we don't want
                // this right paren. 
                if num_left_parens == num_right_parens {
                    tokens.push(tok);
                    break;
                }

                num_right_parens += 1;
                while let Some(lex) = operator_stack.pop() {
                    if let Lexeme::Operator(op) = lex {
                        let r = output.pop().unwrap();
                        let l = output.pop().unwrap();
                        output.push(Expression::BinaryOp(optype_to_op(&op),
                                                         Box::new(l), Box::new(r)));
                    }
                    else {
                        break;
                    }
                }
            }
            _ => {
                // We don't know what this token is, so we give it back.
                tokens.push(tok);
                break;
            },
        }
    }

    while let Some(op) = operator_stack.pop() {
        if let Lexeme::Operator(o) = op {
            let r = output.pop().unwrap();
            let l = output.pop().unwrap();
            output.push(Expression::BinaryOp(optype_to_op(&o),
                                             Box::new(l), Box::new(r))); 
        }
        else {
            panic!("Mismatched parens");
        }
    }
    
    output.pop().unwrap()
}

fn parse_expression(tokens: &mut TokenStream) -> Expression {
    let rpn = two_stack_algo(tokens);
    rpn
}

fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    return Statement::Return(Box::new(expr));
}


fn parse_print(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Print);
    assert!(!tokens.is_empty());
    let out = Statement::Print(Box::new(parse_expression(tokens)));

    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    out
}


fn parse_if(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::If);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);

    Statement::If(Box::new(condition), block)
}

fn parse_while(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::While);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);

    Statement::While(Box::new(condition), block)
}

fn parse_declaration(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Let);
    assert!(!tokens.is_empty());
    
    if let Lexeme::Identifier(name) = tokens.consume() {
        assert_eq!(tokens.consume(), Lexeme::Operator(OperatorType::Assign));

        let expr = parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);

        Statement::Let(name, Box::new(expr))
    }
    else {
        panic!("Expected an identifier");
    }
}

fn parse_assignment(tokens: &mut TokenStream) -> Statement{
    let tok = tokens.consume();
    if let Lexeme::Identifier(name) = tok {
        assert_eq!(tokens.consume(), Lexeme::Operator(OperatorType::Assign));

        let expr = parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
        
        Statement::Assign(name, Box::new(expr))
    }
    else {
        panic!("Expected an identifier");
    }
}

fn parse_call(tokens: &mut TokenStream) -> FunctionCall {
    assert_eq!(tokens.consume(), Lexeme::Call);
    assert_eq!(tokens.consume(), Lexeme::LParen);
    assert!(!tokens.is_empty());

    let tok = tokens.consume();
    if let Lexeme::Identifier(fn_name) = tok {
        assert_eq!(tokens.consume(), Lexeme::Comma);
        let arg_expr = parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::RParen);

        FunctionCall {name:fn_name,
                      arg_expr: Box::new(arg_expr)} 
    } else {
        panic!("Expected a function name");
  }
}

fn parse_function(tokens: &mut TokenStream) -> Function {
    assert_eq!(tokens.consume(), Lexeme::Function);
    assert!(!tokens.is_empty());

    if let Lexeme::Identifier(fn_name) = tokens.consume() {
        assert_eq!(tokens.consume(), Lexeme::LParen);
        
        if let Lexeme::Identifier(fn_arg) = tokens.consume() {
            assert_eq!(tokens.consume(), Lexeme::RParen);
            
            let statements = parse_block(tokens);
            return Function {name: fn_name,
                             statements: statements,
                             arg: fn_arg};
        }
    }
    panic!("Expected fn <function name> (<arg>)");
}

fn parse_statement(tokens: &mut TokenStream) -> Statement {
    let token = tokens.peek();

    match token {
        Lexeme::Return => parse_return(tokens),
        Lexeme::Print => parse_print(tokens),
        Lexeme::If => parse_if(tokens),
        Lexeme::While => parse_while(tokens),
        Lexeme::Let => parse_declaration(tokens),
        Lexeme::Call => {
            let fn_call = parse_call(tokens);
            assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
            Statement::Call(fn_call)
        },
        Lexeme::Identifier(_s) => parse_assignment(tokens),
        _ => panic!("Unexpected lexeme {:?}", token),
    }
}

pub fn parse_block(tokens: &mut TokenStream) -> Vec<Statement> {
    let mut out = Vec::new();
    assert_eq!(tokens.consume(), Lexeme::StartBlock);

    while !tokens.is_empty() {
        if tokens.peek() == Lexeme::EndBlock {
            tokens.consume();

            return out;
        }

        out.push(parse_statement(tokens));
    }

    panic!("Block did not end with a EndBlock lexeme");
}

pub fn parse_program(tokens: &mut TokenStream) -> Vec<Function> {
    let mut out = Vec::new();
    while !tokens.is_empty() {
        out.push(parse_function(tokens));
    }
    out
}
