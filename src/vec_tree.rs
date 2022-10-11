use std::vec::Vec;

pub struct MarkedTree {
    tree: Tree,
    marker: Marker,
}
// does each entry of vector describe one depth level or are we mimicing a depth-first search until
// we hit a leaf. 3 -> 1 -> 2 -> 0 -> 0 -> 0 -> 1 -> 0. with this sequence the following tree is
// described. the root has 3 children, the first one has 1 child which has 2 childs (both are
// leafs). the second child of the root node is a leaf (that means 0 in the sequence). The third
// child of the root node has one child which also has a leaf as a child.
// * is a node 
// *
// | \ \
// *  * *
// |    |
// *    *
// | \
// *  *
type Marker = Vec<u16>;

impl MarkedTree {
    pub fn mark_tree(tree: Tree)-> Self {
        MarkedTree {
            tree: tree, 
            marker: Marker::new(),
        } 
    }

    pub fn select_child(&mut self) {
    // change Marker in the right way, but i forgot how we want to implement the marker.
    //self.marker.change_vec()
    
    }

    pub fn select_parent(&mut self) {

    }

    pub fn select_sibling(&mut self) {

    }

    pub fn read_marker(&self)-> Marker {
        &self.marker
    }
    
    pub fn delete_child() {
        //this function should take the information of the marker and delete the specified child of
        //the Tree. Should it delete the whole subtree or should it only delete the node and the
        //grandparent is now responsible for the children?
        //
        //Does the Marker need to be resetet?
    }
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
