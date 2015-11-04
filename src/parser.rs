use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;


static OPERATOR_ORDER: &'static [OperatorType] = &[
    OperatorType::Plus, OperatorType::Minus,
    OperatorType::CompareGreater, OperatorType::CompareLess,
    OperatorType::CompareGreaterOrEqual, OperatorType::CompareLessOrEqual,
    OperatorType::CompareNotEqual, OperatorType::CompareEqual,

    OperatorType::Star, OperatorType::Divide];

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

fn precedence_leq(a: &OperatorType, b: &OperatorType) -> bool {
    OPERATOR_ORDER.iter().position(|x| x == a) < 
        OPERATOR_ORDER.iter().position(|x| x == b)
}

fn rpn_to_ast(rpn_tokens: &Vec<Lexeme>) -> Expression {
    let mut stack = Vec::new();

    for tok in rpn_tokens {
        match *tok {
            Lexeme::IntConstant(ref v) => stack.push(Expression::Value(*v)),
            Lexeme::Identifier(ref name) => stack.push(Expression::Variable(name.clone())),
            Lexeme::Operator(ref op) => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(Expression::BinaryOp(optype_to_op(op),
                                                Box::new(l),
                                                Box::new(r)));
            }
            _ => panic!("Invalid lexeme"),
        }
    }
    
    stack.pop().unwrap()
}

fn two_stack_algo(tokens: &mut TokenStream) -> Vec<Lexeme> {
    let mut operator_stack = Vec::new();
    let mut output = Vec::new();

    while !tokens.is_empty() {
        let tok = tokens.peek();
        
        match tok {
            Lexeme::Identifier(_) => output.push(tok),
            Lexeme::IntConstant(_v) => output.push(tok),
            Lexeme::Operator(o1) => {
                while let Some(Lexeme::Operator(o2)) = operator_stack.pop() {
                    if precedence_leq(&o1, &o2) {
                        output.push(Lexeme::Operator(o2));
                    }
                    else {
                        // push it back on the stack
                        operator_stack.push(Lexeme::Operator(o2));
                        break;
                    }
                }
                operator_stack.push(Lexeme::Operator(o1));
            }
            Lexeme::LParen => operator_stack.push(tok),
            Lexeme::RParen => {
                while let Some(lex) = operator_stack.pop() {
                    if lex == Lexeme::LParen {
                        break;
                    }
                    
                    output.push(lex);
                }   
            }
            _ => break,
        }

        tokens.consume();
    }

    while let Some(op) = operator_stack.pop() {
        if op == Lexeme::LParen || op == Lexeme::RParen {
            panic!("Mismatched parens");
        }
        
        output.push(op);
    }
    
    output
}

fn parse_expression(tokens: &mut TokenStream) -> Expression {
    let rpn = two_stack_algo(tokens);
    return rpn_to_ast(&rpn);
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

fn parse_statement(tokens: &mut TokenStream) -> Statement {
    let token = tokens.peek();

    match token {
        Lexeme::Return => parse_return(tokens),
        Lexeme::Print => parse_print(tokens),
        Lexeme::If => parse_if(tokens),
        Lexeme::While => parse_while(tokens),
        Lexeme::Let => parse_declaration(tokens),
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
