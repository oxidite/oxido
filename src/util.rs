use crate::errors::syntax;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::ops::Range;

pub struct Error<'a> {
    file: &'a str,
    line: &'a str,
    code: &'a str,
    message: &'a str,
    label: bool,
}

pub fn build_error(error: Error) -> (Diagnostic<usize>, SimpleFiles<&str, &str>) {
    let mut files = SimpleFiles::new();

    let file_id = files.add(error.file, error.line);

    let diagnostic: Diagnostic<usize> = Diagnostic::error()
        .with_message(error.message)
        .with_code(error.code);

    if error.label {
        diagnostic.with_labels(vec![Label::primary(file_id, 0..error.line.len())]);
    }

    (diagnostic, files)
}

pub fn error(error: Error) {
    let (diagnostic, files) = build_error(error);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}

pub fn syntax_error(file: &str, line: &str, expection: &str, founded: &str) {
    error(Error {
        file,
        line,
        code: "E0001",
        message: &syntax(expection, founded),
        label: true,
    });
}
