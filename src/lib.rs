mod database;
mod parser;

use crate::parser::root::RootParser;
use crate::parser::Parser;

pub trait Runnable {
    fn run(args: Vec<String>) -> Result<(), String>;
}

pub struct TodoCli();

impl Runnable for TodoCli {
    fn run(args: Vec<String>) -> Result<(), String> {
        RootParser::parse(args)
    }
}
