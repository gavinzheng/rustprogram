extern crate structopt;
extern crate colored;
extern crate clap; 
use clap::{Arg, App, SubCommand}; 
use colored::*;
use structopt::StructOpt;
use std::env; 

fn main() {
    // 迭代遍历所有的命令行参数，如果在其中找到--help参数，则打印帮助信息
    for argument in env::args() { 
        if argument == "--help" {   // 如果要处理-h，必须增加代码，虽然-h和--help等同
            println!("You passed --help as one of the arguments!"); 
        } 
    }
    
    // 使用collect方法将所有命令行参数存入一个Vector（矢量）变量 
    let arguments: Vec<String> = env::args().collect(); 
    println!("{} arguments passed", arguments.len()); 
    
    // 获取特定位置（此处为1）的命令行参数值
    let message = std::env::args().nth(1) 
                    .expect("Missing the message. Usage: commandline < message>"); 
    
    let name = env::args().skip(1).next(); 
    match name { 
        Some(n) => println!("Hi there ! {}", n), 
        None => panic!("Didn't receive any name ?") 
    }

    ///////////////////////////////////////////////////////////////////////////
    /// StructOpt 通过derive能很方便的处理命令行参数。structopt宏可以设置默认值以及short/long
    ///  缺点是太死板，比如增加一个命令行参数，就需要修改下面的Parameter结构。不利于处理不定长的命令行参数
    #[derive(StructOpt)]
    struct Parameter {
        #[structopt(default_value = "ARGS")]
        message: String, // [1]
    
        #[structopt(short = "h", long = "help")]
        help: bool,

        #[structopt(short = "f", long = "file",  parse(from_os_str))]
        file: Option<std::path::PathBuf>,
    }

    let parameters = Parameter::from_args();
    let message = parameters.message;
    // let file = if parameters.file { 
    //     println!("You provide the greeting card template!"); 
    // };

    match &parameters.file {
        Some (path) => {
            // let cat_template = std::fs::read_to_string(path)
            // .expect(&format!("could not read file {:?}", path));
            // let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", path);
        },
        None => {
            // ... print as before
        } 
    }

    // 使用Colord在控制台上答应彩色字符
    println!("{}", message.bright_yellow().underline().on_purple());

    // let _bufferblock = [0_u8;1024];
    let a = [0, 1, 2, 3, 4]; 
    let complete = &a[..]; // 包含数组a的所有元素的片段
    
    // 包含数组a索引位置1到4（不包含4）的元素的切片，内容为`1`, `2`, and `3`. 
    let middle = &a[1..=4];
    
    ///////////////////////////////////////////////////////////////////////////
    /// Clap
    let app = App::new("commandline");
    // .version("1.0")
    // .about("Says hello")
    // .author("Michael Snoyman");

    // Define the name command line option
    // let name_option = Arg::with_name("help")
    //     .long("help") // allow --name
    //     .takes_value(false)
    //     .help("Who to say hello to")
    //     .required(true);

    // // now add in the argument we want to parse
    // let app = app.arg(name_option);
    let app = app.arg(Arg::with_name("help")
                    .short("h")
                    .long("help")
                    .takes_value(false)
                    .help("Help information "))
                .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("Greeting Card Template file"))
                .arg(Arg::with_name("message")
                    .short("m")
                    .long("message")
                    .takes_value(true)
                    .help("Greeting message"));


    // extract the matches
    let matches = app.get_matches();

    // Extract the actual name
    let name = matches.value_of("help")
        .expect("This can't be None, we said it was required");

    println!("Hello, {}!", name);
}

