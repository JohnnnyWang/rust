use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: private-field
//
// This diagnostic is triggered if created structure does not have field provided in record.
pub(crate) fn private_field(ctx: &DiagnosticsContext<'_>, d: &hir::PrivateField) -> Diagnostic {
    // FIXME: add quickfix
    Diagnostic::new(
        "private-field",
        format!(
            "field `{}` of `{}` is private",
            d.field.name(ctx.sema.db),
            d.field.parent_def(ctx.sema.db).name(ctx.sema.db)
        ),
        ctx.sema.diagnostics_display_range(d.expr.clone().map(|it| it.into())).range,
    )
}

#[cfg(test)]
mod tests {
    use crate::tests::check_diagnostics;

    #[test]
    fn private_field() {
        check_diagnostics(
            r#"
mod module { pub struct Struct { field: u32 } }
fn main(s: module::Struct) {
    s.field;
  //^^^^^^^ error: field `field` of `Struct` is private
}
"#,
        );
    }

    #[test]
    fn private_but_shadowed_in_deref() {
        check_diagnostics(
            r#"
//- minicore: deref
mod module {
    pub struct Struct { field: Inner }
    pub struct Inner { pub field: u32 }
    impl core::ops::Deref for Struct {
        type Target = Inner;
        fn deref(&self) -> &Inner { &self.field }
    }
}
fn main(s: module::Struct) {
    s.field;
}
"#,
        );
    }
}
