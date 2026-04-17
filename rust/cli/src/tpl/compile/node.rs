//! Compiled node types.

use seedfaker_core::field::{Field, RangeSpec, Transform};

#[derive(Debug)]
pub struct CompiledTemplate {
    pub nodes: Box<[Node]>,
}

#[derive(Debug)]
pub enum Node {
    Lit(Box<str>),
    Var(u16),
    New {
        field: &'static Field,
        modifier: Box<str>,
        transform: Transform,
        range: Option<RangeSpec>,
    },
    If {
        branches: Box<[(Cond, Box<[Node]>)]>,
        else_body: Box<[Node]>,
    },
    Repeat {
        count: u16,
        body: Box<[Node]>,
    },
}

#[derive(Debug)]
pub struct Cond {
    pub var: u16,
    pub op: CmpOp,
    pub value: Box<str>,
}

#[derive(Debug)]
pub enum CmpOp {
    Eq,
    Neq,
}
