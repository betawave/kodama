mod tree_kodama;
use tree_kodama::{AbstractASTBuffer, AbstractASTAction, AbstractASTObject};
use console::Term;

struct ActionBinding {
    key: char,
    action: AbstractASTAction,
}

struct ObjectBinding {
    key: char,
    object: AbstractASTObject,
}

/* 
Key Mapping:

Actions:       Objects:
m -> Move      R -> Root
c -> Change    P -> Parent
d -> Delete    C -> Child
i -> Insert    S -> Sibling

Entering an Object without specifiying an action first implies 'Move'.
*/

fn main() {
    let action_bindings = [ActionBinding {key: 'm', action: AbstractASTAction::Move},
                           ActionBinding {key: 'c', action: AbstractASTAction::Change},
			   ActionBinding {key: 'd', action: AbstractASTAction::Delete},
			   ActionBinding {key: 'i', action: AbstractASTAction::Insert}];
    let object_bindings = [ObjectBinding {key: 'R', object: AbstractASTObject::Root},
			   ObjectBinding {key: 'T', object: AbstractASTObject::This},
			   ObjectBinding {key: 'P', object: AbstractASTObject::Parent},
			   ObjectBinding {key: 'C', object: AbstractASTObject::Child},
			   ObjectBinding {key: 'S', object: AbstractASTObject::Sibling},
			   ObjectBinding {key: 'L', object: AbstractASTObject::Leaf}];

    let term = Term::stdout();
    term.clear_screen().expect("Could not clear screen!");

    let mut action: Option<AbstractASTAction> = None;
    let mut object: Option<AbstractASTObject> = None;

    loop {
	let input = term.read_key().unwrap();
	match input {
	    console::Key::Char(in_char) => {
		let mut matched = false;
		if action == None {
		    for i in 0..action_bindings.len() {
			if action_bindings[i].key == in_char {
			    action = Some(action_bindings[i].action);
			    matched = true;
			    break;
			}
		    }
		    if !matched {
			for i in 0..object_bindings.len() {
			    if object_bindings[i].key == in_char {
				object = Some(object_bindings[i].object);
				action = Some(AbstractASTAction::Move);
				matched = true;
				break;
			    }
			}
		    }
		} else {
		    for i in 0..object_bindings.len() {
			if object_bindings[i].key == in_char {
			    object = Some(object_bindings[i].object);
			    matched = true;
			    break;
			}
		    }
		}
		term.clear_screen().expect("Could not clear screen!");
		if !matched {
		    println!("Could not parse '{}'", in_char);
		} else if action != None && object != None {
		    println!("Executing {} {}", action.unwrap(), object.unwrap());
		    action = None;
		    object = None;
		}
	    },
	    console::Key::Escape => break,
	    _ => continue,
	}
    }
}
