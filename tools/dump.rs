extern crate template;

fn main() {
    let ast = template::parse::parse("<% if true {%> hello <% } %>").unwrap();

    let rust_code = template::trans::trans(ast).unwrap();

    println!("{}", rust_code);
}

