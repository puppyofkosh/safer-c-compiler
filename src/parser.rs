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


/// Convert OperatorType to BinaryOp
/// OperatorType is for lexeme while BinaryOp is for ast
/// ```
/// optype_to_op(OperatorType::Plus) = BinaryOp::Plus
/// ```
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

/// Convert lexeme VarType to ast's VarType
fn lexeme_var_type_to_ast(t: lexeme::VarType) -> ast::VarType {
    match t {
        lexeme::VarType::Int => ast::VarType::Int,
        lexeme::VarType::Char => ast::VarType::Char,
        lexeme::VarType::Pointer => panic!("Use parse_type function!")
    }
}

/// Return the precedence of the OperatorType
/// ```
/// get_precedence(OperatorType::Plus) = 0
/// ```
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

/// Check if the input is a identifier and return the 'name' (string) of the identifier
/// ```
/// expect_identifier(Lexeme::identifier("foo")) = "foo"
/// ```
fn expect_identifier(t: Lexeme) -> String {
    if let Lexeme::Identifier(s) = t {
        s
    } else {
        panic!("Expected an identifier, instead got {:?}", t);
    }
}

/// Helper methods for two-stack algorithm:
/// pop two stuff from the stack and evaluate
/// them with the current operator
/// Return an expression which is a binary operation
fn evaluate_bin_op(op: &OperatorType,
                   current_stack: &mut Vec<Expression>)
                   -> Expression {
    let stack_empty_err = format!(
        "Stack is empty when it shouldn't be on op {:?}",
        op);
    let r = current_stack.pop().expect(&stack_empty_err);
    let l = current_stack.pop().expect(&stack_empty_err);
    Expression::BinaryOp(optype_to_op(&op),
                         Box::new(AstExpressionNode::new(l)),
                         Box::new(AstExpressionNode::new(r)))

}

// A "factor" is something that isn't a binary/arithmetic operation
// f(x) is a factor
// f(x + 2) is also a factor, (it contains a binary operation but isn't part
// of one
// x + 5 is not a factor
fn parse_factor(tokens: &mut TokenStream) -> Expression {
    let tok = tokens.consume();

    let mut factor =
    match tok {
        Identifier(_) if tokens.peek() == Lexeme::LParen => {
            tokens.push(tok);
            Expression::Call(parse_call(tokens))
        },
        Identifier(name) => Expression::Variable(name),
        Lexeme::IntConstant(v) => Expression::Value(v),
        Lexeme::StringConstant(s) => Expression::StringValue(s),
        Lexeme::Reference => {
            // Next token should be the thing we want to reference
            let factor = parse_factor(tokens);
            Expression::Reference(Box::new(AstExpressionNode::new(factor)))
        }
        Lexeme::Operator(OperatorType::Star) => {
            let identifier = tokens.consume();
            if let Identifier(name) = identifier {
                Expression::Dereference(name)
            } else {
                panic!("Expected token after * to be identifier");
            }
        }
        _ => panic!("Unexpected lexeme {:?}. A factor can't contain this",
                    tok)
    };

    // Now parse all the field accesses. This is for cases like
    // (*p).x.y.z
    let mut next_tok = tokens.consume();
    while let Lexeme::Dot = next_tok {
        let field_name = expect_identifier(tokens.consume());
        let object_factor = AstExpressionNode::new(factor);
        factor = Expression::FieldAccess(Box::new(object_factor),
                                         field_name);

        next_tok = tokens.consume();
    }
    // The last token we took was not a Dot, so we put it back
    tokens.push(next_tok);

    factor
}

fn two_stack_algo(tokens: &mut TokenStream) -> Expression {
    let mut operator_stack = Vec::new();
    let mut output = Vec::new();

    // Keep track of number of parens. Sometimes we will have an expression
    // followed by a parenthese like call((abc + bcd)), and the last paren is
    // not part of the expression.
    let mut num_left_parens = 0;
    let mut num_right_parens = 0;

    // We toggle this each time we see a factor
    // In general you expect to see a factor, then operator, then factor,
    // We never see operators next to each other, and never see factors next
    // to each other
    let mut is_expecting_factor = true;
    while !tokens.is_empty() {
        let tok = tokens.consume();

        match tok {
            Identifier(_) | Lexeme::IntConstant(_) | Lexeme::StringConstant(_)
                | Lexeme::Reference | Lexeme::Operator(OperatorType::Star)
                if is_expecting_factor => {
                    tokens.push(tok);
                    output.push(parse_factor(tokens));
                    is_expecting_factor = false;
                }
            Lexeme::Dot => {
                // We want to access the stuff we just parsed as a struct
                // An example of this happening is when we do (*p).somefield
                // TODO: We may want to eliminate this case, and only
                // allow p->somefield
                let prev_expr = AstExpressionNode::new(
                    output.pop()
                        .expect("Cannot start an expression with a Dot"));

                let field_name = expect_identifier(tokens.consume());
                let new_expr = Expression::FieldAccess(Box::new(prev_expr),
                                                       field_name);
                output.push(new_expr);

                is_expecting_factor = false;
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
                is_expecting_factor = true;
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

    let res = output.pop().expect("Error: output is empty!");
    assert!(output.is_empty(), "Tokens remaining on the stack! Invalid input");
    res
}

/// Parse a expression using two-stack algorithm
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
    } else if let Lexeme::Identifier(struct_name) = tok {
        ast::VarType::Struct(struct_name)
    } else {
        panic!("Unexpected token! {:?}", tok);
    }
}

/// Parse a return statement
fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    return Statement::Return(expr);
}

/// Parse a print statement
fn parse_print(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Print);
    assert!(!tokens.is_empty());
    let out = Statement::Print(parse_expression(tokens));

    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
    out
}

/// Parse a if statement
fn parse_if(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::If);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);
    let mut else_block = None;
    if tokens.peek() == Lexeme::Else {
        assert_eq!(tokens.consume(), Lexeme::Else);
        else_block = Some(parse_block(tokens));
    }

    Statement::If(condition, block, else_block)
}

/// Parse a while statement
fn parse_while(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::While);
    assert!(!tokens.is_empty());

    let condition = parse_expression(tokens);
    let block = parse_block(tokens);

    Statement::While(condition, block)
}

/// Parse a let statement (declaration with/without assignment)
fn parse_declaration(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Let);
    assert!(!tokens.is_empty());

    let var_type = parse_type(tokens);

    let name = expect_identifier(tokens.consume());

    let mut tok = tokens.consume();
    let mut expr = None;
    if tok == Lexeme::Assign {
        expr = Some(parse_expression(tokens));
        tok = tokens.consume();
    }

    assert_eq!(tok, Lexeme::EndOfStatement);

    Statement::Let(name, var_type, expr)
}

/// Parse an assign statement
fn parse_assignment(tokens: &mut TokenStream) -> Statement{
    // The type checker will make sure that the left expression
    // is "assignable"
    let left = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::Assign);
    let right = parse_expression(tokens);
    assert_eq!(tokens.consume(), Lexeme::EndOfStatement);

    Statement::Assign(left, right)
}

/// Parse a function call statement
fn parse_call(tokens: &mut TokenStream) -> FunctionCall {
    let tok = tokens.consume();
    if let Identifier(fn_name) = tok {
        assert_eq!(tokens.consume(), Lexeme::LParen);
        let mut args_exprs = Vec::new();
        loop {
            let arg_expr = parse_expression(tokens);
            args_exprs.push(arg_expr);
            if tokens.peek() == Lexeme::RParen { break; }
            assert_eq!(tokens.consume(), Lexeme::Comma);
        }
        assert_eq!(tokens.consume(), Lexeme::RParen);
        FunctionCall {name:fn_name, args_exprs: args_exprs }
    } else {
        panic!("Expected a function name");
    }
}

/// Parse a function definition
fn parse_function(tokens: &mut TokenStream) -> Function {
    assert_eq!(tokens.consume(), Lexeme::Function);
    assert!(!tokens.is_empty());

    let return_type = parse_type(tokens);

    let fn_name = expect_identifier(tokens.consume());
    assert_eq!(tokens.consume(), Lexeme::LParen);

    let mut args = Vec::new();
    let mut arg_types = Vec::new();
    loop {
        let arg_type = parse_type(tokens);
        arg_types.push(arg_type);
        let fn_arg = expect_identifier(tokens.consume());
        args.push(fn_arg);
        if tokens.peek() == Lexeme::RParen { break; }
        assert_eq!(tokens.consume(), Lexeme::Comma);
    }

    assert_eq!(tokens.consume(), Lexeme::RParen);

    let statements = parse_block(tokens);
    return Function {name: fn_name,
                     statements: statements,
                     args: args,
                     fn_type: ast::FunctionType {
                         arg_types: arg_types,
                         return_type: return_type,
                     }
    }
}

/// Parse a struct definition
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

/// Parse a statement
fn parse_statement(tokens: &mut TokenStream) -> Statement {
    let token = tokens.peek();

    match token {
        Lexeme::Return => parse_return(tokens),
        Lexeme::Print => parse_print(tokens),
        Lexeme::If => parse_if(tokens),
        Lexeme::While => parse_while(tokens),
        Lexeme::Let => parse_declaration(tokens),
        Identifier(_) if tokens.peek_n(2) == Lexeme::LParen => {
            let fn_call = parse_call(tokens);
            assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
            Statement::Call(fn_call)
        },
        Identifier(_) |
        Lexeme::Operator(OperatorType::Star) |
        Lexeme::LParen => {
            parse_assignment(tokens)
        }
        _ => panic!("Unexpected lexeme {:?}", token),
    }
}

/// Parse a block which is simply formed by a bunch of statements
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

/// The starter of the parser
/// Parsing the program which is simly formed by a bunch of
/// function and struct definition
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
