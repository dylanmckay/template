use Error;
use ast;

use std::io::prelude::*;
use std::io::Cursor;
use std::io;

const INTERNAL_WRITER_NAME: &'static str = "_writer";

/// Configuration options for translation.
#[derive(Clone, Debug)]
pub struct Config
{
    /// Whether an entry point should be included.
    pub include_entry_point: bool,
}

/// Translate an AST into source code.
pub fn trans(ast: ast::Ast, config: &Config) -> Result<String, Error> {
    let mut write = Cursor::new(Vec::new());

    if config.include_entry_point {
        emit_entry_point(&mut write)?;
    }

    emit_main_function_start(&mut write)?;

    for item in ast.items {
        match item.kind {
            ast::ItemKind::Code(code) => {
                emit_code(&code, &mut write)?;
            },
            ast::ItemKind::Text(text) => {
                emit_text(&text, &mut write)?;
            },
        }
    }

    emit_main_function_end(&mut write)?;

    Ok(String::from_utf8(write.into_inner()).unwrap())
}

fn emit_main_function_start(write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "fn render({}: &mut ::std::io::Write) -> Result<(), ::std::io::Error> {{",
        INTERNAL_WRITER_NAME)
}

fn emit_main_function_end(write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "\n    ;Ok(())\n }}")
}

fn emit_code(code: &str, write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "    {}", code.trim())
}

fn emit_text(text: &str, write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "    {}.write_all(b\"{}\")?;", INTERNAL_WRITER_NAME,
        escape_string(text))
}

fn emit_entry_point(write: &mut Write) -> Result<(), io::Error> {
    writeln!(write, "fn main() {{")?;
    writeln!(write, "    render(&mut ::std::io::stdout()).unwrap()")?;
    writeln!(write, "}}\n")?;

    Ok(())
}

fn escape_string(text: &str) -> String {
    text.replace("\\", "\\\\")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\"", "\\\"")
        .replace("'", "\\'")
}

impl Default for Config
{
    fn default() -> Self {
        Config {
            include_entry_point: false,
        }
    }
}


