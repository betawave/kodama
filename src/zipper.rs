use std::mem;

pub trait Tree {
    fn empty() -> Self;
    fn destruct(self) -> Vec<Self> where Self: Sized;
}

pub struct Chain<T: Tree> {
    younglings: Vec<T>,
    ancestors: Box<Path<T>>,
    elders: Vec<T>
}

type Path<T: Tree> = Option<Chain<T>>;

pub struct Zipper<T: Tree> {
    cursor: T,
    context: Path<T>
}

impl<T> Zipper<T> where T: Tree {
    pub fn new(tree: T) -> Self {
	Zipper {
	    cursor: tree,
	    context: None
	}
    }

    pub fn up(&mut self) {
    }

    pub fn down(&mut self) -> Result<(), ZipErr> {
	let mut old_cursor = T::empty();
	mem::swap(&mut old_cursor, &mut self.cursor);
	let mut children = old_cursor.destruct();
	let nc = children.pop();
	self.cursor = match nc {
	    None => return Err(ZipErr::NoBelow),
	    Some(nc) => nc
	};
	let old_context = self.context.take();
	self.context = Some( Chain {
	    younglings: Vec::new(),
	    ancestors: Box::new(old_context),
	    elders: children
	});
	Ok(())
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
