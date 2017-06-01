extern crate template;

fn main() {
    let ast = template::parse::parse("<% bitch { %> hello <% } %>");

    println!("{:#?}", ast);
}

