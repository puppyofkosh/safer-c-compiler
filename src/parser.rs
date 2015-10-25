use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;

fn parse_factor(tokens: &mut TokenStream) -> Expression {
    match tokens.consume() {
        Lexeme::Identifier(name) => Expression::Variable(name),
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

fn parse_term(tokens: &mut TokenStream) -> Expression {
    let left = parse_factor(tokens);
    if tokens.is_empty() {
        return left;
    }

    // Check if the next operator is a * or /, in which case it is part of
    // this term. If it's an addition operator, it's part of a expr, not a
    // term
    match tokens.peek() {
        Lexeme::Operator(ref t) if (*t == OperatorType::Star) | (*t == OperatorType::Divide) => {
            tokens.consume();
            assert!(!tokens.is_empty(), "Last tok shouldn't be an operator");
            let right = parse_factor(tokens);

            let op = optype_to_op(t);
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
        op = optype_to_op(&optype);
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

fn parse_comparison(tokens: &mut TokenStream) -> Expression {
    let left = parse_expr(tokens);

    if tokens.is_empty() {
        return left;
    }

    if let Lexeme::Operator(optype) = tokens.peek() {

        if optype != OperatorType::CompareEqual || optype != OperatorType::CompareGreater || optype != OperatorType::CompareLess || optype != OperatorType::CompareNotEqual || optype != OperatorType::CompareGreaterOrEqual || optype != OperatorType::CompareLessOrEqual {
            return left;
        }
        let op = optype_to_op(&optype);
        let right = parse_expr(tokens);
        return Expression::BinaryOp(op, Box::new(left), Box::new(right));
    }

    left
}

fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_comparison(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    return Statement::Return(Box::new(expr));
}


fn parse_print(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Print);
    assert!(!tokens.is_empty());
    let out = Statement::Print(Box::new(parse_comparison(tokens)));

    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    out
}


fn parse_if(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::If);
    assert!(!tokens.is_empty());

    let condition = parse_comparison(tokens);
    let block = parse_block(tokens);

    Statement::If(Box::new(condition), block)
}

fn parse_declaration(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Let);
    assert!(!tokens.is_empty());
    
    if let Lexeme::Identifier(name) = tokens.consume() {
        assert_eq!(tokens.consume(), Lexeme::Operator(OperatorType::Assign));

        let expr = parse_comparison(tokens);
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

        let expr = parse_comparison(tokens);
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
