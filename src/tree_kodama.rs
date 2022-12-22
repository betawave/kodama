use std::vec::Vec;
use crate::vec_tree::{MarkedTree};

pub struct AbstractASTBuffer {
    marked_tree: MarkedTree
}

impl AbstractASTBuffer {
    pub fn new() -> Self {
	AbstractASTBuffer {
	    marked_tree: MarkedTree::new()
	}
    }

    pub fn execute(&mut self, action: AbstractASTAction, object: AbstractASTObject)
		   -> Result<(), String> {
	match action {
	    AbstractASTAction::GoTo => {
		match object {
		    AbstractASTObject::This => Ok(()),
		    AbstractASTObject::Sibling => {
			    self.marked_tree.select_sibling();
                Ok(())
            }
		    AbstractASTObject::Child => {
		    	self.marked_tree.select_child();
                Ok(())
            }
		    _ => Err("GoTo not yet implemented for given object.".to_string()),
		}
	    },
	    _ => Err("Procedure not implemented!".to_string()),
	}
    }
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AbstractASTAction {
    GoTo,
    Move,
    Change,
    Delete,
    Insert,
}

impl std::fmt::Display for AbstractASTAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    AbstractASTAction::GoTo => write!(f, "GoTo"),
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
