extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "zokrates.pest"]
struct ZokParser;

fn main() {
    let pairs = ZokParser::parse(Rule::function_definition, "def main(field a) -> u8 { return 56;}").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::identifier => println!("identifier:  {}", inner_pair.as_str()),
                Rule::ty => println!("ty:   {}", inner_pair.as_str()),
                Rule::parameter_list => println!("parameter_list:  {}", inner_pair.as_str()),
                Rule::block_statement => println!("block_statement:   {}", inner_pair.as_str()),
                _ => {println!("Otherwise:   {}", inner_pair.as_str());}//unreachable!()}
            };
        }
    }
}