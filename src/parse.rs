use Error;
use ast;

use regex::Regex;

pub const CODE_BLOCK_REGEX: &'static str = "<%.*?%>";

/// A range of characters in the text.
struct Span {
    pub low_index: usize,
    pub high_index: usize,
}

#[derive(Debug)]
enum FragmentKind {
    Code,
    Text,
}

/// A fragment of the text.
struct Fragment {
    kind: FragmentKind,
    span: Span,
}

pub fn parse(input: &str) -> Result<ast::Ast, Error> {
    let code_block_regex = Regex::new(CODE_BLOCK_REGEX).unwrap();

    let code_spans: Vec<_> = code_block_regex.find_iter(input).map(|m| {
        Span {
            // add/subtract the '<%' and '%>' parts.
            low_index: m.start(),
            high_index: m.end(),
        }
    }).collect();

    verify_no_overlapping_spans(&code_spans);

    let code_fragments: Vec<_> = code_spans.into_iter().map(|span| {
        Fragment { span: span, kind: FragmentKind::Code }
    }).collect();

    let fragments = if !code_fragments.is_empty() {
        // If we have code fragments, we can interpolate the text fragments between them.
        fill_in_text_fragments(code_fragments)
    } else {
        vec![Fragment {
            kind: FragmentKind::Text,
            span: Span { low_index: 0, high_index: input.len() },
        }]
    };

    let mut fragments = remove_empty_fragments(fragments);
    trim_delimiters_from_code_frags(&mut fragments);

    let items = fragments.into_iter().map(|frag| {
        let frag_text = input[frag.span.low_index..frag.span.high_index].to_string();

        // TODO: temp
        // if let FragmentKind::Code = frag.kind {
        //     let parse_sess = syntax::parse::ParseSess::new(syntax::codemap::FilePathMapping::empty());
        //     let stmt = syntax::parse::parse_stmt_from_source_str("source name".to_owned(), frag_text.clone(), &parse_sess).unwrap();
        //     println!("stmt: {:?}", stmt);
        // }

        let item_kind = match frag.kind {
            FragmentKind::Text => ast::ItemKind::Text(frag_text),
            FragmentKind::Code => ast::ItemKind::Code(frag_text),
        };

        ast::Item { kind: item_kind }
    }).collect();

    Ok(ast::Ast { items: items })
}

fn verify_no_overlapping_spans(_spans: &[Span]) {
    // FIXME: verify that no code spans overlap.
}

fn fill_in_text_fragments(code_fragments: Vec<Fragment>) -> Vec<Fragment> {
    let mut current_index = 0;

    code_fragments.into_iter().flat_map(|code_fragment| {
        let high_index = code_fragment.span.high_index;
        assert!(code_fragment.span.low_index >= current_index);

        // Check if we have a perfectly contiguous code fragment.
        let fragments = if code_fragment.span.low_index == current_index {
            vec![code_fragment].into_iter()
        } else { // otherwise we have a gap with text data.
            let text_fragment = Fragment {
                kind: FragmentKind::Text,
                span: Span { low_index: current_index, high_index: code_fragment.span.low_index },
            };

            vec![text_fragment, code_fragment].into_iter()
        };

        current_index = high_index;
        fragments
    }).collect()
}

fn remove_empty_fragments(fragments: Vec<Fragment>) -> Vec<Fragment> {
    fragments.into_iter().filter(|frag| frag.span.low_index != frag.span.high_index).collect()
}

/// Trim `<%` and '%>' from code fragments.
fn trim_delimiters_from_code_frags(fragments: &mut Vec<Fragment>) {
    for frag in fragments.iter_mut() {
        if let FragmentKind::Code = frag.kind {
            // Trim the '<%' and '%>'.
            frag.span.low_index += 2;
            frag.span.high_index -= 2;
        }
    }
}

#[cfg(test)]
mod test {
    use ast::*;
    use super::*;

    #[test]
    fn parses_empty_string() {
        assert_eq!(parse("").unwrap(), vec![].into());
    }

    #[test]
    fn parses_standalone_new_lines() {
        assert_eq!(parse("\n\n\n").unwrap(), vec![
            Item { kind: ItemKind::Text("\n\n\n".to_owned()) },
        ].into());
    }

    #[test]
    fn parses_standalone_text() {
        assert_eq!(parse("hello world").unwrap(), vec![
            Item { kind: ItemKind::Text("hello world".to_owned()) },
        ].into());
    }

    #[test]
    fn parses_standalone_code() {
        assert_eq!(parse("<% hello %>").unwrap(), vec![
            Item { kind: ItemKind::Code(" hello ".to_owned()) },
        ].into());
    }
}
