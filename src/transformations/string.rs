use full_moon::visitors::VisitorMut;

use super::decorator::DecoratorStatementVisitor;

pub fn transform_string(code: &str) -> String {
    let mut ast = full_moon::parse(code).expect("Error parsing code");

    let mut decorator_statement_visitor = DecoratorStatementVisitor::default();
    ast = decorator_statement_visitor.visit_ast(ast);

    full_moon::print(&ast)
}