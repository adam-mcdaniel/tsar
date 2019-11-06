use comment::c::strip;

pub mod lower;
pub mod parser;
pub mod tokens;

use lalrpop_util::ParseError;
type Error<T> = ParseError<usize, T, String>;

/// This function is used to convert a string of Tsar code directly
/// to the Xasm intermediate representation.
///
/// It strips comments, then attempts to parse.
pub fn assemble(script: impl ToString) -> Result<String, String> {
    use lower::Lower;
    // We pass the code stripped with comments into the parser
    match parser::ProgramParser::new().parse(&strip(&script.to_string())?) {
        // if the parser succeeds, build will succeed
        Ok(parsed) => Ok(parsed.lower()),
        // if the parser succeeds, annotate code with comments
        Err(e) => Err(format_error(&script.to_string(), e)),
    }
}

/// This formats an error properly given the line, the `unexpected` token as a string,
/// the line number, and the column number of the unexpected token.
pub fn make_error(
    line: &str,
    unexpected: &str,
    line_number: usize,
    column_number: usize,
) -> String {
    // The string used to underline the unexpected token
    let underline = format!(
        "{}^{}",
        " ".repeat(column_number),
        "-".repeat(unexpected.len() - 1)
    );

    // Format string properly and return
    format!(
        "{WS} |
{line_number} | {line}
{WS} | {underline}
{WS} |
{WS} = unexpected `{unexpected}`",
        WS = " ".repeat(line_number.to_string().len()),
        line_number = line_number,
        line = line,
        underline = underline,
        unexpected = unexpected
    )
}

// Gets the line number, the line, and the column number of the error
fn get_line(script: &str, location: usize) -> (usize, String, usize) {
    // Get the line number from the character location
    let line_number = script[..location + 1].lines().count();
    // Get the line from the line number
    let line = match script.lines().nth(line_number - 1) {
        Some(line) => line,
        None => {
            let lines = script.lines().collect::<Vec<&str>>();
            lines[lines.len() - 1]
        }
    }
    .trim();

    // Get the column number from the location
    let column = {
        let mut current_column = 0;
        // For every character in the script until the location of the error,
        // keep track of the column location
        for ch in script[..location].chars() {
            if ch == '\n' {
                current_column = 0;
            } else if ch != '\t' {
                current_column += 1;
            }
        }
        current_column
    };

    (line_number, String::from(line), column as usize)
}

/// This is used to take an LALRPOP error and convert
/// it into a nicely formatted error message
pub fn format_error<T: core::fmt::Debug>(script: &str, err: Error<T>) -> String {
    match err {
        Error::InvalidToken { location } => {
            let (line_number, line, column) = get_line(script, location);
            make_error(&line, &line, line_number, column)
        }
        Error::UnrecognizedEOF { location, .. } => {
            let (line_number, line, _) = get_line(script, location);
            make_error(&line, "EOF", line_number, line.len())
        }
        Error::UnrecognizedToken { token, .. } => {
            let start = token.0;
            let end = token.2;

            let (line_number, line, column) = get_line(script, start);
            let unexpected = &script[start..end];
            make_error(&line, unexpected, line_number, column)
        }
        Error::ExtraToken { token } => {
            let start = token.0;
            let end = token.2;

            let (line_number, line, column) = get_line(script, start);
            let unexpected = &script[start..end];

            make_error(&line, unexpected, line_number, column)
        }
        Error::User { error } => format!(
            "  |\n? | {}\n  | {}\n  |\n  = unexpected compiling error",
            error,
            format!("^{}", "-".repeat(error.len() - 1))
        ),
    }
}
