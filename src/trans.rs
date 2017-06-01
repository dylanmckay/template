use Error;
use ast;

use std::io::prelude::*;
use std::io::Cursor;
use std::io;

const INTERNAL_WRITER_NAME: &'static str = "_writer";

pub fn trans(ast: ast::Ast) -> Result<String, Error> {
    let mut write = Cursor::new(Vec::new());

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
    writeln!(write, "    {}.write_all(b\"{}\")?;", INTERNAL_WRITER_NAME, text)
}

