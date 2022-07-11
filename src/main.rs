use console::Term;

fn main() {
    let term = Term::stdout();
    term.clear_screen().expect("Could not clear screen!");
    loop {
	let input = term.read_key().unwrap();
	match input {
	    console::Key::Char(in_char) => {
		term.clear_screen().expect("Could not clear screen!");
		println!("You entered '{}'", in_char);
	    },
	    console::Key::Escape => break,
	    _ => continue,
	}
    }
}
