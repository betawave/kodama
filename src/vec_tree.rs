use std::vec::Vec;

pub struct MarkedTree {
    tree: Tree,
    marker: Marker,
}

type Marker = Vec<usize>;

impl MarkedTree {
    pub fn mark_tree(tree: Tree)-> Self {
        MarkedTree {
            tree: tree, 
            marker: Marker::new(),
        } 
    }

    pub fn select_child(&mut self) {
        if self.get_selected().has_children() {
            self.marker.push(0);  
            println!("joooooooo");
        } else {
            println!("nöööö, geht nicht");
        }
            
    }

    pub fn select_parent(&mut self) {

        if self.marker.is_empty() {
            println!("joo du bist gott, du hast dich selbst erschaffen");
        } else {
            self.marker.pop();
            println!("jeeeeehaaaaa");
        }
    }

    pub fn select_sibling(&mut self) {
        
        if self.marker.is_empty() {
            println!("waisen können keine eltern auswählen"); 
            return;
        }
        
        let current_sibling_index = self.marker[self.marker.len()-1];
        self.select_parent();
        

        if self.get_selected().number_of_creations() > current_sibling_index+1 {
           let last_index = self.marker.len()-1;
           self.marker[last_index]+=1;
           println!(" do the sibling dance"); 
        } else {
            println!("ick bin einzelkind!");
        }
    }

    pub fn get_marker(&self)-> &Marker {
        &self.marker
    }
    
    pub fn get_selected(&self)-> &Tree {

        let mut walker = &self.tree;

        for child_index in self.marker.iter() {
            walker = &walker.children[*child_index];
        }

        walker    
    }

    pub fn get_selected_mutabel(&mut self)-> &mut Tree {

        let mut walker = &mut self.tree;

        for child_index in self.marker.iter() {
            walker = &mut walker.children[*child_index];
        }

        walker    
    }
    
    pub fn delete_selected(&mut self) {
        //this function should take the information of the marker and delete the selected node of the Tree. The function will delete the selected subtree.
        // The marker will be changed to the position of the parent of the deleted subtree.
        if self.marker.is_empty() {
            println!("Die Welt kann sich nicht selbst zerstören"); 
            return;
        }
        let current_sibling_index = self.marker[self.marker.len()-1];
        self.select_parent();
        self.get_selected_mutabel().remove_child(current_sibling_index);
    }

    pub fn create_child() {

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

    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn number_of_creations(&self) -> usize {
        self.children.len()
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
