extern crate pest_derive;
extern crate pest_ast;

use pest::Parser;
use pest_derive::Parser;
use pest_ast::FromPest;
use std::fs;

#[derive(pest_derive::Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

// mod ast {
//     use super::csv::Rule;
//     use pest::Span;

//     fn span_into_str(span: Span) -> &str {
//         span.as_str()
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::field))]
//     pub struct Field {
//         #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
//         pub value: f64,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::record))]
//     pub struct Record {
//         pub fields: Vec<Field>,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::file))]
//     pub struct File {
//         pub records: Vec<Record>,
//         eoi: EOI,
//     }

//     #[derive(Debug, FromPest)]
//     #[pest_ast(rule(Rule::EOI))]
//     struct EOI;
// }

fn main() {
    // let source = "aaabbbccc";

    // let mut parse_tree = CSVParser::parse(Rule::S, source).expect("parse success");
    // println!("parse tree = {:#?}", parse_tree);

    // let syntax_tree = S::from_pest(&mut parse_tree).expect("infallible");
    // println!("syntax tree = {:#?}", syntax_tree);

    // let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    // println!("{:?}", successful_parse);

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);

    let unparsed_file = fs::read_to_string("./numbers.csv").expect("cannot read file");

    let file = CSVParser::parse(Rule::file, &unparsed_file)
                .expect("unsuccessful parse") // unwrap the parse result
                .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
