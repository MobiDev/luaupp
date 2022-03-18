use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Call, DecoratorStatement, Expression, FunctionArgs, FunctionCall, LocalAssignment, Prefix,
        Stmt, Suffix, Value,
    },
    tokenizer::TokenReference,
    visitors::VisitorMut,
};

use super::util::remove_trailing_whitespace;
#[derive(Default)]

pub struct DecoratorStatementVisitor {
    names: Vec<String>,
}

impl VisitorMut for DecoratorStatementVisitor {
    fn visit_decorator_statement(
        &mut self,
        decorator_statement: DecoratorStatement,
    ) -> DecoratorStatement {
        match decorator_statement.clone().expr() {
            Some(mut decexpr) => {
                decexpr = remove_trailing_whitespace(decexpr);
                let stmt = *decorator_statement.stmt();
                match stmt {
                    Stmt::Assignment(_) => todo!(),
                    Stmt::Do(_) => todo!(),
                    Stmt::FunctionCall(_) => todo!(),
                    Stmt::FunctionDeclaration(_) => todo!(),
                    Stmt::GenericFor(_) => todo!(),
                    Stmt::If(_) => todo!(),
                    Stmt::LocalAssignment(local_assignment) => {
                        let la = local_assignment.clone();
                        let mut expr_list: Punctuated<Expression> = Punctuated::new();
                        for expr in local_assignment.expressions().to_owned().into_pairs() {
                            let mut arguments: Punctuated<Expression> = Punctuated::new();
							let exprc = expr.clone();
                            arguments.push(Pair::End(remove_trailing_whitespace(
                                expr.value().to_owned(),
                            )));
                            expr_list.push(Pair::new(
                                Expression::Value {
									value: Box::new(
										Value::FunctionCall(
											FunctionCall::new(
												Prefix::Expression(decexpr.to_owned())
											).with_suffixes(
												vec![
													Suffix::Call(
														Call::AnonymousCall(
															FunctionArgs::Parentheses {
																arguments,
																parentheses: ContainedSpan::new(
																	TokenReference::symbol("(").unwrap(),
																	TokenReference::symbol(")").unwrap(),
																),
															},
														)
													)
												]
											)
										)
									),
                                    type_assertion: match expr {
                                        Pair::End(a) => match a {
                                            Expression::Value { value, type_assertion } => type_assertion,
                                            _ => None,
                                        },
                                        Pair::Punctuated(a, _) => match a {
                                            Expression::Value { value, type_assertion } => type_assertion,
                                            _ => None,
                                        },
                                    },
                                    // value: Box::new(Value::FunctionCall(FunctionCall {
                                    //     prefix: Prefix::Expression(decexpr.to_owned()),
                                    //     suffixes: vec![Suffix::Call(Call::AnonymousCall(
                                    //         FunctionArgs::Parentheses {
                                    //             arguments,
                                    //             parentheses: ContainedSpan::new(
                                    //                 TokenReference::symbol("(").unwrap(),
                                    //                 TokenReference::symbol(")").unwrap(),
                                    //             ),
                                    //         },
                                    //     ))],
                                    // })),
									
                                },
                                match exprc.punctuation() {
                                    Some(p) => Some(p.to_owned()),
                                    None => None,
                                },
                            ));
                        }
                        DecoratorStatement::new(None, Box::new(Stmt::LocalAssignment(LocalAssignment::new(la.names().to_owned())
							.with_local_token(la.local_token().to_owned())
							.with_equal_token(match la.equal_token() {
									Some(s) => Some(s.to_owned()),
									None => None,
								})
							.with_expressions(expr_list)
					)))
                    }
                    Stmt::DecoratorStatement(_) => todo!(),
                    Stmt::LocalFunction(_) => todo!(),
                    Stmt::NumericFor(_) => todo!(),
                    Stmt::Repeat(_) => todo!(),
                    Stmt::While(_) => todo!(),
                    _ => todo!(),
				}
            }

            None => decorator_statement,
        }
    }
}
