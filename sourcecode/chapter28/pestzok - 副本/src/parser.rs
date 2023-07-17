#![allow(clippy::clone_on_copy)]

extern crate pest;
// #[macro_use] 
// extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "zokrates.pest"]
struct ZoKratesParser;

#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    ZoKratesParser::parse(Rule::file, input)
}

