use std::vec::Vec;

pub struct AbstractAST {
    children: Vec<AbstractAST>
}

pub enum AbstractASTContext<'a> {
    Top,
    Path {
	elders: Vec<AbstractAST>,
	ancestors: &'a AbstractASTContext<'a>,
	younglings: Vec<AbstractAST>,
    },
}

pub struct AbstractASTBuffer<'a> {
    cursor: AbstractAST,
    context: AbstractASTContext<'a>,
}

impl AbstractASTBuffer<'_> {
    fn execute(&mut self, action: AbstractASTAction, object: AbstractASTObject)
	       -> Result<(), String> {
	Err("Procedure not implemented!".to_string())
    }
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AbstractASTAction {
    Move,
    Change,
    Delete,
    Insert,
}

impl std::fmt::Display for AbstractASTAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    AbstractASTAction::Move => write!(f, "Move"),
	    AbstractASTAction::Change => write!(f, "Change"),
	    AbstractASTAction::Delete => write!(f, "Delete"),
	    AbstractASTAction::Insert => write!(f, "Insert"),
	}
    }
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AbstractASTObject {
    Root,
    Parent,
    This,
    Child,
    Sibling,
    Leaf,
}

impl std::fmt::Display for AbstractASTObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    AbstractASTObject::Root => write!(f, "Root"),
	    AbstractASTObject::Parent => write!(f, "Parent"),
	    AbstractASTObject::This => write!(f, "This"),
	    AbstractASTObject::Child => write!(f, "Child"),
	    AbstractASTObject::Sibling => write!(f, "Sibling"),
	    AbstractASTObject::Leaf => write!(f, "Leaf"),
	}
    }
}
