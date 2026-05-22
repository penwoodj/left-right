use ariadne::{Color, Label, Report, ReportKind, Source};
use lr_common::Span;
use std::ops::Range;

pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
    pub message: String,
    pub source_name: String,
}

pub enum DiagnosticKind {
    Error,
    Warning,
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>, source_name: impl Into<String>) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            span,
            message: message.into(),
            source_name: source_name.into(),
        }
    }

    pub fn eprint(&self, source: &str) {
        let range: Range<usize> = self.span.into();
        let kind = match self.kind {
            DiagnosticKind::Error => ReportKind::Error,
            DiagnosticKind::Warning => ReportKind::Warning,
        };

        let source_span = (self.source_name.clone(), range.clone());
        Report::build(kind, source_span.clone())
            .with_message(&self.message)
            .with_label(
                Label::new(source_span)
                    .with_message(&self.message)
                    .with_color(Color::Red),
            )
            .finish()
            .eprint((self.source_name.clone(), Source::from(source)))
            .unwrap();
    }
}

pub fn eprint_diagnostics(diagnostics: &[Diagnostic], source: &str) {
    for d in diagnostics {
        d.eprint(source);
    }
}