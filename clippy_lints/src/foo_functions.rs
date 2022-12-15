use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::ast::*;
use rustc_ast::visit::FnKind;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;

declare_clippy_lint! {
    /// ### What it does
    ///  Checks for functions named `foo`
    /// ### Why is this bad?
    ///   `foo` is an undescriptive function name.
    /// ### Example
    /// ```rust
    /// foo() {
    ///     println!("hello");
    ///  }
    /// ```
    /// Use instead:
    /// ```rust
    /// say_hello() {
    ///     println!("hello");
    ///  }
    /// ```
    #[clippy::version = "1.67.0"]
    pub FOO_FUNCTIONS,
    pedantic,
    "function named `foo`, which is not a descriptive name"
}
declare_lint_pass!(FooFunctions => [FOO_FUNCTIONS]);

impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if is_foo_fn(fn_kind) {
            span_lint_and_help(
                cx,
                FOO_FUNCTIONS,
                span,
                "function named `foo`",
                None,
                "consider using a more meaningful name",
            );
        }
    }
}

fn is_foo_fn(fn_kind: FnKind<'_>) -> bool {
    match fn_kind {
        FnKind::Fn(_, ident, ..) => ident.name.as_str() == "foo",
        // ignore closures
        FnKind::Closure(..) => false,
    }
}
