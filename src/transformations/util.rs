use full_moon::{
    ast::{Expression, Value, Var, FunctionCall, Suffix, FunctionArgs, span::ContainedSpan, Call},
    tokenizer::TokenReference,
};
//  -> Vec<Suffix> 

fn clear_all_whitespace_suffixes(suffixes: Vec<Suffix>) -> Vec<Suffix> {
	suffixes.iter().map(|suffix| match suffix {
		Suffix::Call(call) => match call {
			full_moon::ast::Call::AnonymousCall(anonymous_call) => Suffix::Call(Call::AnonymousCall(remove_trailing_whitespace_function_args(anonymous_call))),
			full_moon::ast::Call::MethodCall(method_call) => todo!(),
			_ => todo!(),
		},
		Suffix::Index(_) => todo!(),
			_ => todo!(),	
		}).collect()
}

pub fn remove_trailing_whitespace_function_args(function_args: &FunctionArgs) -> FunctionArgs {
	match function_args {
		FunctionArgs::Parentheses { parentheses, arguments } => {
			let new_end_token = TokenReference::new(
				vec![], 
				parentheses.tokens().1.token().to_owned(),  
				parentheses.tokens().1.trailing_trivia().filter(|t| match t.token_type() {
					full_moon::tokenizer::TokenType::Whitespace { characters: _ } => false,
					_ => true,
				}
			).map(|x| x.to_owned()).collect());
			FunctionArgs::Parentheses { 
				parentheses: ContainedSpan::new(parentheses.tokens().0.to_owned(), new_end_token), 
				arguments: arguments.to_owned() 
			}
		},
		FunctionArgs::String(string) => todo!(),
		FunctionArgs::TableConstructor(table) => todo!(),
		_ => todo!(),
	}
}

pub fn remove_trailing_whitespace(aexpr: Expression) -> Expression {
	let cexpr = aexpr.clone();
    match aexpr {
        Expression::BinaryOperator { lhs, binop, rhs } => todo!(),
        Expression::Parentheses {
            contained,
            expression,
        } => todo!(),
        Expression::UnaryOperator { unop, expression } => todo!(),

        Expression::Value { value, type_assertion } => match *value {
            Value::Function(_) => todo!(),
            Value::FunctionCall(fn_call) => {
				// TODO: 
				Expression::Value { 
					value: Box::new(Value::FunctionCall(
						FunctionCall::new(fn_call.prefix().to_owned()).with_suffixes(clear_all_whitespace_suffixes(fn_call.suffixes().map(|s| s.to_owned()).collect())))
					),
					type_assertion: type_assertion
				}
			},
            Value::TableConstructor(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::ParenthesesExpression(_) => todo!(),
            Value::String(token) => Expression::Value {
                value: Box::new(Value::Var(Var::Name(TokenReference::new(
                    vec![],
                    token.token().to_owned(),
                    vec![],
                )))),
				type_assertion: type_assertion
            },
            Value::Symbol(token) => Expression::Value {
                value: Box::new(Value::Var(Var::Name(TokenReference::new(
                    vec![],
                    token.token().to_owned(),
                    vec![],
                )))),
				type_assertion: type_assertion
            },
            Value::Var(var) => match var {
                Var::Expression(_) => todo!(),
                Var::Name(token) => Expression::Value {
                    value: Box::new(Value::Var(Var::Name(TokenReference::new(
                        vec![],
                        token.token().to_owned(),
                        vec![],
                    )))),
					type_assertion: type_assertion
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}
