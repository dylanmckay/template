#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate tempo;
extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;
use syntax::{parse, codemap};

use std::io::prelude::*;
use std::fs;

struct Options {
    file_path: String,
    file_path_span: codemap::Span,
}

fn expand_include(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
    let options = match build_options(cx, sp, args) {
        Ok(opts) => opts,
        Err(res) => return res,
    };

    let mut file = match fs::File::open(&options.file_path) {
        Ok(f) => f,
        Err(e) => {
            cx.span_err(options.file_path_span, &e.to_string());
            return DummyResult::any(options.file_path_span);
        }
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(..) => (),
        Err(e) => panic!("error: {}", e),
    }

    let ast = match tempo::parse::parse_str(&file_contents) {
        Ok(ast) => ast,
        Err(e) => panic!("error: {}", e),
    };

    let rust_source = tempo::trans::rust_code(ast, &Default::default()).unwrap();

    let parse_sess = parse::ParseSess::new(codemap::FilePathMapping::empty());
    let krate = match syntax::parse::parse_crate_from_source_str("crate name".to_owned(), rust_source, &parse_sess) {
        Ok(krate) => krate,
        Err(mut e) => {
            e.emit();
            return DummyResult::any(options.file_path_span);
        },
    };

    MacEager::items(krate.module.items.into_iter().collect())
}

fn build_options(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Result<Options, Box<MacResult + 'static>> {
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single string, but got {} arguments", args.len()));
        return Err(DummyResult::any(sp));
    }

    let (file_path_span, file_path) = match args[0] {
        TokenTree::Token(path_sp, token::Literal(token::Lit::Str_(s), _)) => (path_sp, s.to_string()),
        _ => {
            cx.span_err(sp, "argument should be a single string");
            return Err(DummyResult::any(sp));
        }
    };

    Ok(Options {
        file_path: file_path,
        file_path_span: file_path_span,
    })
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("tempo_include", expand_include);
}

