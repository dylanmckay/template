extern crate tempo;
extern crate clap;

use clap::{Arg, App};

use std::io::prelude::*;
use std::io::stderr;
use std::{fs, process};

struct Config<'a>
{
    /// What we should generate.
    output_kind: OutputKind,
    /// The file path of the `.trs` file.
    input_file_path: &'a str,
    /// Whether a standalone program should be generated.
    standalone: bool,
}

enum OutputKind {
    Code,
    Ast,
}

fn main() {
    let matches = App::new("Tempo template engine")
                          .version(tempo::VERSION)
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about(env!("CARGO_PKG_DESCRIPTION"))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("standalone")
                               .short("s")
                               .help("Output source code with a main() function"))
                          .arg(Arg::with_name("print")
                               .long("print")
                               .takes_value(true)
                               .possible_values(&["ast"])
                               .help("Print specific information instead of source code"))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    let output_kind = match matches.value_of("print") {
        None => OutputKind::Code,
        Some("ast") => OutputKind::Ast,
        _ => unreachable!(),
    };

    let config = Config {
        output_kind: output_kind,
        input_file_path: matches.value_of("INPUT").unwrap(),
        standalone: matches.is_present("standalone"),
    };

    if let Err(e) = run(&config) {
        writeln!(stderr(), "error: {}", e).unwrap();
        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), tempo::Error> {
    let mut file = fs::File::open(config.input_file_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let ast = tempo::parse::parse_str(&source).unwrap();

    match config.output_kind {
        OutputKind::Code => print_code(ast, config)?,
        OutputKind::Ast => print_ast(ast)?,
    }

    Ok(())
}

fn print_code(ast: tempo::Ast, config: &Config) -> Result<(), tempo::Error> {
    let trans_config = tempo::trans::Config {
        include_entry_point: config.standalone,
    };

    let rust_code = tempo::trans::rust_code(ast, &trans_config).unwrap();
    println!("{}", rust_code);

    Ok(())
}

fn print_ast(ast: tempo::Ast) -> Result<(), tempo::Error> {
    println!("{:#?}", ast);
    Ok(())
}


