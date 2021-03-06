//! Acts like an interface to the actual Pest routines.
//!
//! It also prevents the need for importing Pest stuff everywhere.

use pest::error::{Error as PestErr, ErrorVariant};
use pest::Span;

pub use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct SDLP;

/// The parsing error type, generated by Pest.
pub type Error = PestErr<Rule>;
/// The parsing result type, based on `Error`.
pub type ParseRes<T> = Result<T, Error>;
/// Pest's parse tree type.
pub type ParseTree<'input> = Pair<'input, Rule>;

/// Allows creating custom error messages under Pest's errors.
pub fn parse_err(message: String, span: Span) -> Error {
    Error::new_from_span(ErrorVariant::CustomError { message }, span)
}

/// Parses the given text into a tag tree.
pub fn parse(rule: Rule, input: &str) -> ParseRes<ParseTree> {
    SDLP::parse(rule, input).map(|mut res| res.next().unwrap())
}
