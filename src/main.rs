mod zipper;
mod tree_kodama;
mod vec_tree;
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
g -> GoTo      T -> This
m -> Move      R -> Root
c -> Change    P -> Parent
d -> Delete    C -> Child
i -> Insert    S -> Sibling
               L -> Leaf

Entering an Object without specifiying an action first implies 'Move'.
*/

fn main() {
    let action_bindings = [ActionBinding {key: 'g', action: AbstractASTAction::GoTo},
			   ActionBinding {key: 'm', action: AbstractASTAction::Move},
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

    let mut buffer = AbstractASTBuffer::new();

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
				action = Some(AbstractASTAction::GoTo);
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
		if !matched {
		    term.clear_screen().expect("Could not clear screen!");
            println!("{}", buffer.print_me());
		    println!("Could not parse '{}'", in_char);
		} else if action != None && object != None {
		    let act = action.unwrap();
		    let obj = object.unwrap();
		    let res = buffer.execute(act, obj);
		    term.clear_screen().expect("Could not clear screen!");
            println!("{}", buffer.print_me());
		    println!("Executing {} {}", act, obj);
            match res {
			    Err(message) => println!("{}", message),
			    Ok(()) => (),
		    }

		    action = None;
		    object = None;
		}
	    },
	    console::Key::Escape => break,
	    _ => continue,
	}
    }
}
