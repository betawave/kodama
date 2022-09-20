use std::vec::Vec;

pub struct MarkedTree {
    tree: Tree,
    marker: Marker,
}

type Marker = Vec<u16>;

impl MarkedTree {
}

pub struct Tree {
    children: Vec<Tree>,
}

impl Tree {
    pub fn new() -> Self {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_root() {
        let root = Tree {
            children: Vec::new(),
        };
    }

    #[test]
    fn create_root_with_child() {
        let mut root = Tree {
            children: vec![Tree {
                            children: Vec::new(),
                           }],
        };
    }
}
