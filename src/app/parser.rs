mod add_parser;
mod root_parser;
use crate::app::parser::root_parser::RootParser;

pub trait Parseable {
    fn print_help() -> ();
    fn parse(args: Vec<String>) -> ();
}

pub struct Parser();

impl Parseable for Parser {
    fn print_help() -> () {
        RootParser::print_help();
    }

    fn parse(args: Vec<String>) -> () {
        RootParser::parse(args);
    }
}
