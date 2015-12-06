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

use lexeme::OperatorType;
use token_stream::TokenStream;

use std::collections::HashMap;
use std::collections::HashSet;

struct Parser {
    // Table for recognizing struct we have
    struct_table: HashSet<String>,
}

impl Parser {

    pub fn new() -> Parser {
        Parser {
            struct_table: HashSet::new(),
        }
    }
    /// Convert OperatorType to BinaryOp
    /// OperatorType is for lexeme while BinaryOp is for ast
    /// ```
    /// self.optype_to_op(OperatorType::Plus) = BinaryOp::Plus
    /// ```
    fn optype_to_op(&mut self, op: &OperatorType) -> BinaryOp {
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
    fn lexeme_var_type_to_ast(&mut self, t: lexeme::VarType) -> ast::VarType {
        match t {
            lexeme::VarType::Int => ast::VarType::Int,
            lexeme::VarType::Char => ast::VarType::Char,
        }
    }

    /// Return the precedence of the OperatorType
    /// ```
    /// self.get_precedence(OperatorType::Plus) = 0
    /// ```
    fn get_precedence(&mut self, op: &OperatorType) -> i32 {
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
    /// self.expect_identifier(Lexeme::identifier("foo")) = "foo"
    /// ```
    fn expect_identifier(&mut self, t: Lexeme) -> String {
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
    fn evaluate_bin_op(&mut self, op: &OperatorType,
        current_stack: &mut Vec<Expression>)
        -> Expression {
            let stack_empty_err = format!(
                "Stack is empty when it shouldn't be on op {:?}",
                op);
                let r = current_stack.pop().expect(&stack_empty_err);
                let l = current_stack.pop().expect(&stack_empty_err);
                Expression::BinaryOp(self.optype_to_op(&op),
                Box::new(AstExpressionNode::new(l)),
                Box::new(AstExpressionNode::new(r)))

            }

    // A "factor" is something that isn't a binary/arithmetic operation
    // f(x) is a factor
    // f(x + 2) is also a factor, (it contains a binary operation but isn't part
    // of one
    // x + 5 is not a factor
    fn parse_factor(&mut self, tokens: &mut TokenStream) -> Expression {
        let tok = tokens.consume();

        let mut factor =
        match tok {
            Lexeme::Identifier(_) if tokens.peek() == Lexeme::LParen => {
                tokens.push(tok);
                Expression::Call(self.parse_call(tokens))
            },
            Lexeme::Identifier(name) => Expression::Variable(name),
            Lexeme::IntConstant(v) => Expression::Value(v),
            Lexeme::CharConstant(v) => Expression::Value(v),
            Lexeme::StringConstant(s) => Expression::StringValue(s),
            Lexeme::Reference => {
                // Next token should be the thing we want to reference
                let factor = self.parse_factor(tokens);
                Expression::Reference(Box::new(AstExpressionNode::new(factor)))
            }
            Lexeme::Operator(OperatorType::Star) => {
                let identifier = tokens.consume();
                if let Lexeme::Identifier(name) = identifier {
                    Expression::Dereference(name)
                } else {
                    panic!("Expected token after * to be identifier");
                }
            }
            _ => panic!("Unexpected lexeme {:?}. A factor can't contain self",
            tok)
        };

        // Now parse all the field accesses. self is for cases like
        // (*p).x.y.z
        let mut next_tok = tokens.consume();
        while let Lexeme::Dot = next_tok {
            let field_name = self.expect_identifier(tokens.consume());
            let object_factor = AstExpressionNode::new(factor);
            factor = Expression::FieldAccess(Box::new(object_factor),
            field_name);

            next_tok = tokens.consume();
        }
        // The last token we took was not a Dot, so we put it back
        tokens.push(next_tok);

        factor
    }

    /// Executing the two stack algorithm
    fn two_stack_algo(&mut self, tokens: &mut TokenStream) -> Expression {
        let mut operator_stack = Vec::new();
        let mut output = Vec::new();

        // Keep track of number of parens. Sometimes we will have an expression
        // followed by a parenthese like call((abc + bcd)), and the last paren is
        // not part of the expression.
        let mut num_left_parens = 0;
        let mut num_right_parens = 0;

        // We toggle self each time we see a factor
        // In general you expect to see a factor, then operator, then factor,
        // We never see operators next to each other, and never see factors next
        // to each other
        let mut is_expecting_factor = true;
        while !tokens.is_empty() {
            let tok = tokens.consume();

            match tok {
                Lexeme::Identifier(_) | Lexeme::IntConstant(_) | Lexeme::CharConstant(_)
                | Lexeme::StringConstant(_)
                | Lexeme::Reference | Lexeme::Operator(OperatorType::Star)
                if is_expecting_factor => {
                    tokens.push(tok);
                    output.push(self.parse_factor(tokens));
                    is_expecting_factor = false;
                }
                Lexeme::Dot => {
                    // We want to access the stuff we just parsed as a struct
                    // An example of self happening is when we do (*p).somefield
                    // TODO: We may want to eliminate self case, and only
                    // allow p->somefield
                    let prev_expr = AstExpressionNode::new(
                        output.pop()
                        .expect("Cannot start an expression with a Dot"));

                        let field_name = self.expect_identifier(tokens.consume());
                        let new_expr = Expression::FieldAccess(Box::new(prev_expr),
                        field_name);
                        output.push(new_expr);

                        is_expecting_factor = false;
                    }
                    Lexeme::Operator(o1) => {
                        while let Some(Lexeme::Operator(o2)) = operator_stack.pop() {
                            if self.get_precedence(&o1) <= self.get_precedence(&o2) {
                                let bin_expr = self.evaluate_bin_op(&o2, &mut output);
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
                        // self right paren.
                        if num_left_parens == num_right_parens {
                            tokens.push(tok);
                            break;
                        }

                        num_right_parens += 1;
                        while let Some(lex) = operator_stack.pop() {
                            if let Lexeme::Operator(op) = lex {
                                let bin_expr = self.evaluate_bin_op(&op, &mut output);
                                output.push(bin_expr);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    _ => {
                        // We don't know what self token is, so we give it back, and
                        // assume the expression ends here
                        tokens.push(tok);
                        break;
                    },
                }
            }

            while let Some(op) = operator_stack.pop() {
                if let Lexeme::Operator(o) = op {
                    let bin_expr = self.evaluate_bin_op(&o, &mut output);
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
    fn parse_expression(&mut self, tokens: &mut TokenStream) -> AstExpressionNode {
        let expr = self.two_stack_algo(tokens);
        AstExpressionNode::new(expr)
    }

    /// Parse the type
    fn parse_type(&mut self, tokens: &mut TokenStream) -> ast::VarType {
        let tok = tokens.consume();
        if let Lexeme::Type(t) = tok {
            let base_type = self.lexeme_var_type_to_ast(t);
            self.parse_pointer(tokens, base_type)
        } else if let Lexeme::Identifier(struct_name) = tok {
            self.parse_pointer(tokens, ast::VarType::Struct(struct_name))
        } else {
            panic!("Unexpected token! {:?}", tok);
        }
    }

    /// Parse pointer if needed
    fn parse_pointer(&mut self, tokens: &mut TokenStream, base_type: ast::VarType) -> ast::VarType {
        let mut res = base_type;
        while tokens.peek() == Lexeme::Operator(OperatorType::Star) {
            res = ast::VarType::Pointer(Box::new(res.clone()));
            tokens.consume();
        }
        res
    }

    /// Parse a return statement
    fn parse_return(&mut self, tokens: &mut TokenStream) -> Statement {
        assert_eq!(tokens.consume(), Lexeme::Return);
        assert!(!tokens.is_empty());

        let expr = self.parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
        return Statement::Return(expr);
    }

    /// Parse a print statement
    fn parse_print(&mut self, tokens: &mut TokenStream) -> Statement {
        assert_eq!(tokens.consume(), Lexeme::Print);
        assert!(!tokens.is_empty());
        let out = Statement::Print(self.parse_expression(tokens));

        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
        out
    }

    /// Parse a if statement
    fn parse_if(&mut self, tokens: &mut TokenStream) -> Statement {
        assert_eq!(tokens.consume(), Lexeme::If);
        assert!(!tokens.is_empty());

        let condition = self.parse_expression(tokens);
        let block = self.parse_block(tokens);
        let mut else_block = None;
        if tokens.peek() == Lexeme::Else {
            assert_eq!(tokens.consume(), Lexeme::Else);
            else_block = Some(self.parse_block(tokens));
        }

        Statement::If(condition, block, else_block)
    }

    /// Parse a while statement
    fn parse_while(&mut self, tokens: &mut TokenStream) -> Statement {
        assert_eq!(tokens.consume(), Lexeme::While);
        assert!(!tokens.is_empty());

        let condition = self.parse_expression(tokens);
        let block = self.parse_block(tokens);

        Statement::While(condition, block)
    }

    /// Parse a let statement (declaration with/without assignment)
    fn parse_declaration(&mut self, tokens: &mut TokenStream) -> Statement {
        assert!(!tokens.is_empty());

        let var_type = self.parse_type(tokens);

        let name = self.expect_identifier(tokens.consume());

        let mut tok = tokens.consume();
        let mut expr = None;
        if tok == Lexeme::Assign {
            expr = Some(self.parse_expression(tokens));
            tok = tokens.consume();
        }

        assert_eq!(tok, Lexeme::EndOfStatement);

        Statement::Let(name, var_type, expr)
    }

    /// Parse an assign statement
    fn parse_assignment(&mut self, tokens: &mut TokenStream) -> Statement{
        // The type checker will make sure that the left expression
        // is "assignable"
        let left = self.parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::Assign);
        let right = self.parse_expression(tokens);
        assert_eq!(tokens.consume(), Lexeme::EndOfStatement);

        Statement::Assign(left, right)
    }

    /// Parse a function call statement
    fn parse_call(&mut self, tokens: &mut TokenStream) -> FunctionCall {
        let tok = tokens.consume();
        if let Lexeme::Identifier(fn_name) = tok {
            assert_eq!(tokens.consume(), Lexeme::LParen);
            let mut args_exprs = Vec::new();
            loop {
                let arg_expr = self.parse_expression(tokens);
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
    fn parse_function(&mut self, tokens: &mut TokenStream) -> Function {
        if let Lexeme::Type(_) = tokens.peek() {
            let return_type = self.parse_type(tokens);

            let fn_name = self.expect_identifier(tokens.consume());
            assert_eq!(tokens.consume(), Lexeme::LParen);

            let mut args = Vec::new();
            let mut arg_types = Vec::new();
            loop {
                let arg_type = self.parse_type(tokens);
                arg_types.push(arg_type);
                let fn_arg = self.expect_identifier(tokens.consume());
                args.push(fn_arg);
                if tokens.peek() == Lexeme::RParen { break; }
                assert_eq!(tokens.consume(), Lexeme::Comma);
            }

            assert_eq!(tokens.consume(), Lexeme::RParen);

            let statements = self.parse_block(tokens);
            return Function {name: fn_name,
                statements: statements,
                args: args,
                fn_type: ast::FunctionType {
                    arg_types: arg_types,
                    return_type: return_type,
                    is_var_args: false,
                }
            }
        } else {
            panic!("The function declaration starts without type");
        }
    }

    /// Parse a struct definition
    fn parse_struct(&mut self, tokens: &mut TokenStream) -> StructDefinition {
        assert_eq!(tokens.consume(), Lexeme::Struct);
        let name = self.expect_identifier(tokens.consume());
        assert_eq!(tokens.consume(), Lexeme::StartBlock);

        let mut field_to_type = HashMap::new();
        while tokens.peek() != Lexeme::EndBlock {
            let typ = self.parse_type(tokens);
            let field_name = self.expect_identifier(tokens.consume());

            field_to_type.insert(field_name, typ);

            assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
        }
        assert_eq!(tokens.consume(), Lexeme::EndBlock);

        self.struct_table.insert(name.clone());
        StructDefinition {
            name: name,
            fields: field_to_type,
        }
    }

    /// Parse a statement
    fn parse_statement(&mut self, tokens: &mut TokenStream) -> Statement {
        let token = tokens.peek();

        match token {
            Lexeme::Return => self.parse_return(tokens),
            Lexeme::Print => self.parse_print(tokens),
            Lexeme::If => self.parse_if(tokens),
            Lexeme::While => self.parse_while(tokens),
            Lexeme::Type(_) => self.parse_declaration(tokens),
            Lexeme::Identifier(ref struct_name) if self.struct_table.contains(struct_name) =>
                self.parse_declaration(tokens),
            Lexeme::Identifier(_) if tokens.peek_n(2) == Lexeme::LParen => {
                let fn_call = self.parse_call(tokens);
                assert_eq!(tokens.consume(), Lexeme::EndOfStatement);
                Statement::Call(fn_call)
            },
            Lexeme::Identifier(_) |
            Lexeme::Operator(OperatorType::Star) |
            Lexeme::LParen => {
                self.parse_assignment(tokens)
            }
            _ => panic!("Unexpected lexeme {:?}", token),
        }
    }

    /// Parse a block which is simply formed by a bunch of statements
    fn parse_block(&mut self, tokens: &mut TokenStream) -> Vec<Statement> {
        let mut out = Vec::new();
        assert_eq!(tokens.consume(), Lexeme::StartBlock);

        while !tokens.is_empty() {
            if tokens.peek() == Lexeme::EndBlock {
                tokens.consume();

                return out;
            }

            out.push(self.parse_statement(tokens));
        }

        panic!("Block did not end with a EndBlock lexeme");
    }

    /// Parsing the program which is simly formed by a bunch of
    /// function and struct definition
    fn parse_program(&mut self, tokens: &mut TokenStream) -> ast::Program {
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        while !tokens.is_empty() {
            let t = tokens.peek();
            match t {
                Lexeme::Type(_) => functions.push(self.parse_function(tokens)),
                Lexeme::Struct => structs.push(self.parse_struct(tokens)),
                _ => panic!("Illegal token {:?}", t),
            }
        }
        ast::Program{functions: functions,
            structs: structs}
    }
}

/// The starter of the parser
pub fn parse(tokens: &mut TokenStream) -> ast::Program {
    let mut p = Parser::new();
    p.parse_program(tokens)
}
