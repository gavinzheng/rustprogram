use inkwell::{
     builder::Builder, context::Context, execution_engine::JitFunction, types::IntType,
     values::AnyValue, values::IntValue, OptimizationLevel,
};

use crate::{Compile,  Result};
use crate::ast::*;

type JitFunc = unsafe extern "C" fn() -> i32;

// ANCHOR: jit_ast
pub struct Jit;

impl Compile for Jit {
    type Output = Result<i32>;

    fn from_ast(ast: File) -> Self::Output {

       let context = Context::create();
       let module = context.create_module("zokprogram");
       let builder = context.create_builder();
       let execution_engine = module
                              .create_jit_execution_engine(OptimizationLevel::None)
                              .unwrap();

       for symdeclare in ast.declarations {
            match symdeclare {
                SymbolDeclaration::Function(x) => {
                    for statement in x.statements {
                        match statement {
                            Statement::Return(y) => {
                                match y.expression.unwrap() {
                                    // Identifier(z),
                                    Expression::Literal(w) =>{
                                        match w{
                                            LiteralExpression::DecimalLiteral(u)=>{
                                                let ret = u.value.span.as_str().to_string().parse::<i32>().unwrap();
                                                // println!("return value :{} ret={}", u.value.span.as_str().to_string(),ret) ;
                                        
                                                let i32_type = context.i32_type();
                                                let fn_type = i32_type.fn_type(&[], false);
                                        
                                                let function = module.add_function(&x.id.value, fn_type, None);
                                                //let function = module.add_function("jit", fn_type, None);
                                                let basic_block = context.append_basic_block(function, "entry");
                                        
                                                builder.position_at_end(basic_block);
                                                // builder.build_return(Some(&u.value.span.as_str().to_string()));
                                                let i32_type = context.i32_type();
                                                let i32_val =  i32_type.const_int(ret as u64, false);
                                                builder.build_return(Some(&i32_val));
                                                // println!("ret value = {}",i32_val);
                                                // for node in ast {
                                                //     let recursive_builder = RecursiveBuilder::new(i32_type, &builder);
                                                //     let return_value = recursive_builder.build(&node);
                                                //     builder.build_return(Some(&return_value));
                                                // }
                                                println!(
                                                    "Generated LLVM IR: {}",
                                                    function.print_to_string().to_string()
                                                );
                                        
                                                //unsafe {
                                                //    let jit_function: JitFunction<JitFunc> = execution_engine.get_function("jit").unwrap();
                                        
                                                    //Ok(jit_function.call());
                                                //    Ok(jit_function.call())
                                               // }              
                                            }
                                            _ => unreachable!(),   
                                        }
                                    }
                                    _ => unreachable!(),
                                }    
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }

        }

        unsafe {
          let jit_function: JitFunction<JitFunc> = execution_engine.get_function("main").unwrap();
          Ok(jit_function.call())
        }
        /*let context = Context::create();
        let module = context.create_module("zokprogram");

        let builder = context.create_builder();

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);

        let function = module.add_function("jit", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        // for node in ast {
        //     let recursive_builder = RecursiveBuilder::new(i32_type, &builder);
        //     let return_value = recursive_builder.build(&node);
        //     builder.build_return(Some(&return_value));
        // }
        println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );

        unsafe {
            let jit_function: JitFunction<JitFunc> = execution_engine.get_function("jit").unwrap();

            Ok(jit_function.call())
        }*/
        // Ok(1)
    }
}
// ANCHOR_END: jit_ast

// ANCHOR: jit_recursive_builder
// struct RecursiveBuilder<'a> {
//     i32_type: IntType<'a>,
//     builder: &'a Builder<'a>,
// }

// impl<'a> RecursiveBuilder<'a> {
//     pub fn new(i32_type: IntType<'a>, builder: &'a Builder) -> Self {
//         Self { i32_type, builder }
//     }
//     pub fn build(&self, ast: &Node) -> IntValue {
//         match ast {
//             Node::Int(n) => self.i32_type.const_int(*n as u64, true),
//             Node::UnaryExpr { op, child } => {
//                 let child = self.build(child);
//                 match op {
//                     Operator::Minus => child.const_neg(),
//                     Operator::Plus => child,
//                 }
//             }
//             Node::BinaryExpr { op, lhs, rhs } => {
//                 let left = self.build(lhs);
//                 let right = self.build(rhs);

//                 match op {
//                     Operator::Plus => self.builder.build_int_add(left, right, "plus_temp"),
//                     Operator::Minus => self.builder.build_int_sub(left, right, "minus_temp"),
//                 }
//             }
//         }
//     }
// }
// ANCHOR_END: jit_recursive_builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Jit::from_source("1 + 2").unwrap(), 3);
        assert_eq!(Jit::from_source("2 + (2 - 1)").unwrap(), 3);
        assert_eq!(Jit::from_source("(2 + 3) - 1").unwrap(), 4);
        assert_eq!(Jit::from_source("1 + ((2 + 3) - (2 + 3))").unwrap(), 1);
        assert_eq!(Jit::from_source("(1 + 2)").unwrap(), 3);
        // parser fails
        // assert_eq!(Jit::from_source("2 + 3 - 1").unwrap(), 4);
    }
}
