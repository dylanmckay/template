extern crate tempo;
extern crate clap;

use clap::{Arg, App};

use std::io::prelude::*;
use std::io::stderr;
use std::{fs, process};

struct Config<'a>
{
    /// The file path of the `.trs` file.
    input_file_path: &'a str,
    /// Whether a standalone program should be generated.
    standalone: bool,
}

fn main() {
    let matches = App::new("Tempo template engine")
                          .version(tempo::VERSION)
                          .author("Dylan McKay <me@dylanmckay.io>")
                          .about("A better templating engine for Rust")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("standalone")
                               .short("s")
                               .help("Output source code with a main() function"))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    let config = Config {
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

    let trans_config = tempo::trans::Config {
        include_entry_point: config.standalone,
    };

    let ast = tempo::parse::parse(&source).unwrap();
    let rust_code = tempo::trans::trans(ast, &trans_config).unwrap();
    println!("{}", rust_code);

    Ok(())
}


