pub mod lower;
pub mod parser;
pub mod tokens;

use lalrpop_util::ParseError;
type Error<T> = ParseError<usize, T, String>;

pub fn assemble(script: impl ToString) -> Result<String, String> {
    use lower::Lower;
    match parser::ProgramParser::new().parse(&script.to_string()) {
        Ok(parsed) => Ok(parsed.lower()),
        Err(e) => Err(format_error(&script.to_string(), e))
    }
}


pub fn make_error(
    script: &str,
    unexpected: &str,
    line_number: usize,
    column_number: usize,
) -> String {
    format!(
        "{WS} |
{line_number} | {line}
{WS} | {underline}
{WS} |
{WS} = unexpected `{unexpected}`",
        WS = " ".repeat(line_number.to_string().len()),
        line_number = line_number,
        line = script.lines().nth(line_number - 1).unwrap(),
        underline = format!(
            "{}^{}",
            " ".repeat(column_number),
            "-".repeat(unexpected.len() - 1)
        ),
        unexpected = unexpected
    )
}

fn get_line(script: &str, location: usize) -> (usize, String, usize) {
    let line_number = script[..location + 1].lines().count();
    let line = match script.lines().nth(line_number - 1) {
        Some(line) => line,
        None => {
            let lines = script.lines().collect::<Vec<&str>>();
            lines[lines.len() - 1]
        }
    };

    let column = {
        let mut current_column = 0;
        let location = 0;
        for ch in script[..location].chars() {
            current_column += 1;
            if ch == '\n' {
                current_column = 0;
            }
        }
        current_column
    };
    (line_number, String::from(line), column as usize)
}

pub fn format_error<T: core::fmt::Debug>(script: &str, err: Error<T>) -> String {
    match err {
        Error::InvalidToken { location } => {
            let (line_number, line, column) = get_line(script, location);
            make_error(script, &line, line_number, column)
        }
        Error::UnrecognizedEOF { location, .. } => {
            let (line_number, line, _) = get_line(script, location);
            make_error(script, "EOF", line_number, line.len())
        }
        Error::UnrecognizedToken { token, .. } => {
            let start = token.0;
            let end = token.2;

            let (line_number, _, column) = get_line(script, start);
            let unexpected = &script[start..end];
            make_error(script, unexpected, line_number, column)
        }
        Error::ExtraToken { token } => {
            let start = token.0;
            let end = token.2;

            let (line_number, _, column) = get_line(script, start);
            let unexpected = &script[start..end];

            make_error(script, unexpected, line_number, column)
        }
        Error::User { error } => format!(
            "  |\n? | {}\n  | {}\n  |\n  = unexpected compiling error",
            error,
            format!("^{}", "-".repeat(error.len() - 1))
        ),
    }
}
