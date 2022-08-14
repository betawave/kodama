use std::vec::Vec;

pub struct TreeNode<'a> {
    parent: Option<&'a TreeNode<'a>>, 
    children: Vec<TreeNode<'a>>,
}

impl<'a> TreeNode<'a> {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_root() {
        let root = TreeNode {
            parent: None,
            children: Vec::new(),
        };
    }

    #[test]
    fn create_root_with_child() {
        let mut root = TreeNode {
            parent: None,
            children: vec![TreeNode {
                            parent: None,
                            children: Vec::new(),
                           }],
        };
        root.children[0].parent = Some(&root);
    }
}
