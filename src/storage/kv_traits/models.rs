#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Key(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub String);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Tag(pub String);