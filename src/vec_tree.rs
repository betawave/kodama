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
        Tree {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Tree) {
        self.children.push(child);
    }

    pub fn remove_child() {

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_tree_with_no_children() {
        Tree::new();
    } 

    #[test] 
    fn create_tree_with_child() {
        let mut tree = Tree::new();
        tree.add_child(Tree::new());
        tree.add_child(Tree::new());
    }
}
