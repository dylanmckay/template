#![feature(plugin)]
#![plugin(tempo_include)]

mod basic_html {
    tempo_include!("../examples/basic_html.trs");
}

fn main() {
    basic_html::render(&mut ::std::io::stdout()).unwrap();
}

