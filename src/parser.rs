use ast;
use ast::AstExpressionNode;
use ast::FunctionCall;
use ast::StructDefinition;
use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use ast::Function;

use lexeme;
use lexeme::Lexeme;
use lexeme::Lexeme::Identifier;

use lexeme::OperatorType;
use token_stream::TokenStream;

use std::collections::HashMap;

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
    }
}

fn lexeme_var_type_to_ast(t: lexeme::VarType) -> ast::VarType {
    match t {
        lexeme::VarType::Int => ast::VarType::Int,
        lexeme::VarType::Char => ast::VarType::Char,
        lexeme::VarType::Pointer => panic!("Use parse_type function!")
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
    }
}

fn expect_identifier(t: Lexeme) -> String {
    if let Lexeme::Identifier(s) = t {
        s
    } else {
        panic!("Expected an identifier, instead got {:?}", t);
    }
}

fn evaluate_bin_op(op: &OperatorType,
                   current_stack: &mut Vec<Expression>)
                   -> Expression {
    let r = current_stack.pop().unwrap();
    let l = current_stack.pop().unwrap();
    Expression::BinaryOp(optype_to_op(&op),
                         Box::new(AstExpressionNode::new(l)),
                         Box::new(AstExpressionNode::new(r)))
    
}

// Return true if when the operator * follows this token,
// it should be treated as a dereference, false if it should not
fn deref_follows(tok: &Option<Lexeme>) -> bool {
    match tok.as_ref() {
        None => true,
        Some(lex) => {
            match *lex {
                Lexeme::Operator(_) => true,
                Lexeme::LParen => true,
                _ => false
            }
        }
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

    let mut prev_tok = None;
    while !tokens.is_empty() {
        let tok = tokens.consume();
        let tok_copy = tok.clone();
        
        match tok {
            Identifier(name) => output.push(Expression::Variable(name)),
            Lexeme::IntConstant(v) => output.push(Expression::Value(v)),
            Lexeme::StringConstant(s) => {
                output.push(Expression::StringValue(s));
            },
            Lexeme::Call => {
                tokens.push(tok);
                output.push(Expression::Call(parse_call(tokens)));
            }
            Lexeme::Reference => {
                // next token should be an identifier
                let identifier = tokens.consume();
                if let Identifier(name) = identifier {
                    output.push(Expression::Reference(name));
                } else {
                    panic!("Expected token after & to be identifier");
                }
            }
            Lexeme::Operator(OperatorType::Star) if deref_follows(&prev_tok) => {
                let identifier = tokens.consume();
                if let Identifier(name) = identifier {
                    output.push(Expression::Dereference(name));
                } else {
                    panic!("Expected token after * to be identifier");
                }
            }
            Lexeme::Operator(o1) => {
                while let Some(Lexeme::Operator(o2)) = operator_stack.pop() {
                    if get_precedence(&o1) <= get_precedence(&o2) {
                        let bin_expr = evaluate_bin_op(&o2, &mut output);
                        output.push(bin_expr);
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
                        let bin_expr = evaluate_bin_op(&op, &mut output);
                        output.push(bin_expr);
                    }
                    else {
                        break;
                    }
                }
            }
            _ => {
                // We don't know what this token is, so we give it back, and
                // assume the expression ends here
                tokens.push(tok);
                break;
            },
        }
        prev_tok = Some(tok_copy);
    }

    while let Some(op) = operator_stack.pop() {
        if let Lexeme::Operator(o) = op {
            let bin_expr = evaluate_bin_op(&o, &mut output);
            output.push(bin_expr);
        }
        else {
            panic!("Mismatched parens");
        }
    }
    
    let res = output.pop().unwrap();
    assert!(output.is_empty(), "Tokens remaining on the stack! Invalid input");
    res
}

fn parse_expression(tokens: &mut TokenStream) -> AstExpressionNode {
    let expr = two_stack_algo(tokens);
    AstExpressionNode::new(expr)
}

fn parse_type(tokens: &mut TokenStream) -> ast::VarType {
    let tok = tokens.consume();
    if let Lexeme::Type(t) = tok {
        if t == lexeme::VarType::Pointer {
            assert_eq!(tokens.consume(), Lexeme::LParen);
            let res = ast::VarType::Pointer(Box::new(parse_type(tokens)));
            assert_eq!(tokens.consume(), Lexeme::RParen);
            return res;
        } else {
            return lexeme_var_type_to_ast(t);
        }
    } else {
        panic!("Unexpected token! {:?}", tok);
    }
}

fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    return Statement::Return(expr);
}


fn parse_print(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Print);
    assert!(!tokens.is_empty());
    let out = Statement::Print(parse_expression(tokens));

    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    out
}


fn parse_if(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::If);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);

    Statement::If(condition, block)
}

fn parse_while(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::While);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);

    Statement::While(condition, block)
}

fn parse_declaration(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Let);
    assert!(!tokens.is_empty());
    
    let var_type = parse_type(tokens);

    let name = expect_identifier(tokens.consume());
    assert_eq!(tokens.consume(), Lexeme::Assign);

    let expr = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);

    Statement::Let(name, var_type, expr)
}

fn parse_assignment(tokens: &mut TokenStream) -> Statement{
    // The type checker will make sure that the left expression
    // is "assignable"
    let left = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::Assign);
    let right = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);

    Statement::Assign(left, right)
}

fn parse_call(tokens: &mut TokenStream) -> FunctionCall {
    assert_eq!(tokens.consume(), Lexeme::Call);
    assert_eq!(tokens.consume(), Lexeme::LParen);
    assert!(!tokens.is_empty());

    let tok = tokens.consume();
    if let Identifier(fn_name) = tok {
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

    let return_type = parse_type(tokens);
    
    let fn_name = expect_identifier(tokens.consume());
    assert_eq!(tokens.consume(), Lexeme::LParen);
    
    let arg_type = parse_type(tokens);

    let fn_arg = expect_identifier(tokens.consume());
    assert_eq!(tokens.consume(), Lexeme::RParen);
    
    let statements = parse_block(tokens);
    return Function {name: fn_name,
                     statements: statements,
                     arg: fn_arg,
                     fn_type: ast::FunctionType {
                         arg_types: vec![arg_type],
                         return_type: return_type,
                     }
    }
}

fn parse_struct(tokens: &mut TokenStream) -> StructDefinition {
    assert_eq!(tokens.consume(), Lexeme::Struct);
    let name = expect_identifier(tokens.consume());
    assert_eq!(tokens.consume(), Lexeme::StartBlock);

    let mut field_to_type = HashMap::new();
    while tokens.peek() != Lexeme::EndBlock {
        let typ = parse_type(tokens);
        let field_name = expect_identifier(tokens.consume());

        field_to_type.insert(field_name, typ);
        
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    }
    assert_eq!(tokens.consume(), Lexeme::EndBlock);

    StructDefinition {
        name: name,
        fields: field_to_type,
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
        Lexeme::Call => {
            let fn_call = parse_call(tokens);
            assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
            Statement::Call(fn_call)
        },
        Identifier(_) | Lexeme::Operator(OperatorType::Star) => {
            parse_assignment(tokens)
        }
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

pub fn parse_program(tokens: &mut TokenStream) -> ast::Program {
    let mut functions = Vec::new();
    let mut structs = Vec::new();
    while !tokens.is_empty() {
        let t = tokens.peek();
        match t {
            Lexeme::Function => functions.push(parse_function(tokens)),
            Lexeme::Struct => structs.push(parse_struct(tokens)),
            _ => panic!("Illegal token {:?}", t),
        }
    }
    ast::Program{functions: functions,
                 structs: structs}
}
