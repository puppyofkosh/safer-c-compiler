use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;

fn parse_factor(tokens: &mut TokenStream) -> Expression {
    match tokens.consume() {
        Lexeme::IntConstant(v) => Expression::Value(v),
        Lexeme::LParen => {
            let expr = parse_expr(tokens);
            let last_token = tokens.consume();
            assert_eq!(last_token, Lexeme::RParen);
            expr
        }
        _ => panic!("Wth"),
    }
}

// FIXME: Do we really want this?
fn optype_to_op(opstr: OperatorType) -> BinaryOp {
    match opstr {
        OperatorType::Plus => BinaryOp::Plus,
        OperatorType::Minus => BinaryOp::Minus,
        OperatorType::Star => BinaryOp::Multiply,
        OperatorType::Divide => BinaryOp::Divide,
    }
}

fn parse_term(tokens: &mut TokenStream) -> Expression {
    let left = parse_factor(tokens);
    if tokens.is_empty() {
        return left;
    }

    let next = tokens.peek();
    let op = match next {
        Lexeme::Operator(optype) => optype_to_op(optype),
        _ => return left,
    };

    // Check if the next operator is a * or /, in which case it is part of
    // this term. If it's an addition operator, it's part of a expr, not a
    // term
    match op {
        BinaryOp::Multiply | BinaryOp::Divide => {
            tokens.consume();
            assert!(!tokens.is_empty(), "Last tok shouldn't be an operator");
            let right = parse_factor(tokens);
            Expression::BinaryOp(op, Box::new(left), Box::new(right))
        }
        _ => left
    }
}

fn parse_expr(tokens: &mut TokenStream) -> Expression {
    let left = parse_term(tokens);
    if tokens.is_empty() {
        return left;
    }

    // Check if the next token is something we're interested in (an operator)
    // If not, we're done parsing this expression
    let op;
    if let Lexeme::Operator(optype) = tokens.peek() {
        op = optype_to_op(optype);
        tokens.consume();
    } else {
        return left;
    }

    if tokens.is_empty() {
        panic!("Op should not be the end of the statement");
    }

    let right = parse_term(tokens);
    Expression::BinaryOp(op, Box::new(left), Box::new(right))
}

fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_expr(tokens);
    return Statement::Return(Box::new(expr));
}


fn parse_print(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Print);
    assert!(!tokens.is_empty());

    Statement::Print(Box::new(parse_expr(tokens)))
}

fn parse_statement(tokens: &mut TokenStream) -> Statement {
    match tokens.peek() {
        Lexeme::Return => parse_return(tokens),
        Lexeme::Print => parse_print(tokens),
        _ => panic!("Unexpected lexeme"),
    }
}

pub fn parse_tokens(tokens: &mut TokenStream) -> Vec<Statement> {
    let mut out = Vec::new();
    while !tokens.is_empty() {
        out.push(parse_statement(tokens));
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    }
    out
}
