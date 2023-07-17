
#![allow(warnings)]
use pestzok::ast::*;
use cfg_if::cfg_if;

use pestzok::Compile;

//cfg_if! {
//    if #[cfg(feature = "default")] {
//        use pestzok::Jit as Engine;
//    }
//    else {
//        use pestzok::Interpreter as Engine;
//    }
//}
use pestzok::Jit as Engine;

fn main() {
//    let args: Vec<String> = std::env::args().collect();
//    if args.len() < 2 {
//        eprintln!("No input file was provided");
//        std::process::exit(-1);
//    }
    // println!(
    //     "{:?}",
    //     Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap()).unwrap()
    // );
    println!(
        "{:?}",
        Engine::from_source("def main() -> u32 { return 56;}").unwrap()
    );

    // let pairs = generate_ast(Rule::function_definition, "def main() -> u8 { return 56;}").unwrap_or_else(|e| panic!("{}", e));
    // println!("parse_tree: {:?}", pairs);

    // for pair in pairs {
    //     if let Rule::function_definition = pair.as_rule() {
    //         // ast.push(build_ast_from_expr(pair));
    //         println!("function_definition:   {}", pair.as_str());
    //     }
    //     for inner_pair in pair.into_inner() {
    //         match inner_pair.as_rule() {
    //             Rule::identifier => println!("identifier:  {}", inner_pair.as_str()),
    //             Rule::ty => println!("ty:   {}", inner_pair.as_str()),
    //             Rule::parameter_list => println!("parameter_list:  {}", inner_pair.as_str()),
    //             Rule::block_statement => println!("block_statement:   {}", inner_pair.as_str()),
    //             Rule::statement => println!("statement:   {}", inner_pair.as_str()),
    //             Rule::function_definition => println!("function_definition:   {}", inner_pair.as_str()),
    //             Rule::symbol_declaration => println!("symbol_declaration:   {}", inner_pair.as_str()),
    //             Rule::return_statement => println!("return_statement:   {}", inner_pair.as_str()),
    //             _ => {println!("Otherwise:   {}", inner_pair.as_str());}//unreachable!()}
    //         };
    //     }
    // }

    // Because ident_list is silent, the iterator will contain idents
    // for pair in pairs {
    //     // A pair is a combination of the rule which matched and a span of input
    //     println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", pair.as_span());
    //     println!("Text:    {}", pair.as_str());

    //     // A pair can be converted to an iterator of the tokens which make it up:
    //     for inner_pair in pair.into_inner() {
    //         match inner_pair.as_rule() {
    //             Rule::identifier => println!("identifier:  {}", inner_pair.as_str()),
    //             Rule::ty => println!("ty:   {}", inner_pair.as_str()),
    //             Rule::parameter_list => println!("parameter_list:  {}", inner_pair.as_str()),
    //             Rule::block_statement => println!("block_statement:   {}", inner_pair.as_str()),
    //             Rule::function_definition => println!("function_definition:   {}", inner_pair.as_str()),
    //             Rule::symbol_declaration => println!("symbol_declaration:   {}", inner_pair.as_str()),
    //             _ => {println!("Otherwise:   {}", inner_pair.as_str());}//unreachable!()}
    //         };
    //     }
    // }


}




