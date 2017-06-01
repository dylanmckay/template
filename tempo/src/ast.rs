#[derive(Clone, Debug, PartialEq)]
pub struct Ast
{
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item
{
    pub kind: ItemKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind
{
    /// A normal piece of text.
    Text(String),
    /// A block of code.
    Code {
        /// The source code.
        source: String,
        /// Whether the result should be printed.
        print_result: bool,
    }
}

impl From<Vec<Item>> for Ast
{
    fn from(items: Vec<Item>) -> Ast {
        Ast { items: items }
    }
}

