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
    Code(String),
    /// A code block containing a bunch of sub-items.
    CodeBlock(Vec<Item>),
}

impl From<Vec<Item>> for Ast
{
    fn from(items: Vec<Item>) -> Ast {
        Ast { items: items }
    }
}

