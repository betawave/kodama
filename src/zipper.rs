use std::mem;

pub struct Chain<T> {
    younglings: Vec<T>,
    ancestors: Box<Path<T>>,
    elders: Vec<T>
}

type Path<T> = Option<Chain<T>>;

pub struct Zipper<T> {
    cursor: T,
    context: Path<T>
}

impl<T> Zipper<T> {
    pub fn new(tree: T) -> Self {
	Zipper {
	    cursor: tree,
	    context: None
	}
    }

    pub fn up(&mut self) {
    }

    pub fn down(&mut self) {
    }

    pub fn previous(&mut self) {
    }

    pub fn next(&mut self) -> Result<(), ZipErr> {
	match &mut self.context {
	    None => Err(ZipErr::NoNext),
	    Some(chain) => {
		let nc = chain.elders.pop();
		let mut new_cursor = match nc {
		    None => return Err(ZipErr::NoNext),
		    Some(nc) => nc
		};
		mem::swap(&mut self.cursor, &mut new_cursor);
		chain.younglings.push(new_cursor);
		Ok(())
	    }
	}
    }
}

pub enum ZipErr {
    NoNext,
    NoPrev,
    NoAbove,
    NoBelow,
}
