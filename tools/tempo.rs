extern crate tempo;

fn main() {
    let ast = tempo::parse::parse("<% if true {%> hello world\n<% } %>").unwrap();

    let rust_code = tempo::trans::trans(ast).unwrap();

    println!("{}", rust_code);
}

