use std::vec::Vec;
use std::collections::VecDeque;

pub struct MarkedTree {
    tree: Tree,
    marker: Marker,
}

pub fn print_tree(marked_tree: &MarkedTree) -> String {
    let mut stack = Vec::<(&Tree, usize)>::new();
    let mut queue = VecDeque::<(&Tree, usize)>::new();

    // building the Depth-First-Search-List in queue
    stack.push((&marked_tree.tree, 0));
    while !stack.is_empty() {
        let (temp_tree, temp_generation) = stack.pop().unwrap();

        for child_tree in temp_tree.children.iter().rev() {
            stack.push((child_tree, temp_generation+1));
        } 

        queue.push_back((temp_tree, temp_generation));
    }


    // turning queue into string list
    // Was ist die Queue? -> Ist eine darstellung des Depth-First Traversal unseres Baumes. (Zeigt
    // den WE an den unser Algorithmus geht (durch den Baumm))
    // Durch das abarbeiten der Queue geht nachfolgender code genau diesen weg
    // lines ist ein vector der erarbeiteten outputzeilen
    let mut lines = Vec::<String>::new();
    // counter ist eine zustandsdarstellung (funktion) des algorithmus zur abarbeitung der queue
    let mut counter = Vec::<usize>::new();
    // es wird davon ausgegangen das es immer den root knoten gibt, der auch abgearbeitet werden
    // muss.
    counter.push(1);
    //in der schleife wird nun der counter mit jedem durchgang mit informationen aus der queue (den
    //elementen) gefüttert. Zusätzlich werden immer zwei veränderungen am counter durchgeführt. Mit
    //sciherheit eine dekrementierung. und dazu entweder eine verlängerung oder verkürzung des
    //vektors.
    while !queue.is_empty() {
        // solange die queue nicht leer ist wird mit der naechsten zeile das nächste bisschen
        // information extrahiert
        let (current_elem, current_gen) = queue.pop_front().unwrap();

        // als nächstes gibt es eine zustandsänderung durch veränderung des counters
        //jetzt wird die aktuelle generation des counters dekrementiert
        let last_index = counter.len()-1;
        counter[last_index] -= 1;

        //erhaltene information wird mit der counter_to_prefix() in eine fuer nutzende person
        //lesbare zeile umgewandelt.
        lines.push(counter_to_prefix(&counter) + &String::from("*"));
        
        // falls das aktuelle element der queue kinder hat wird der counter erweitert, falls nicht
        // wird der counter verkürzt
        if current_elem.has_children() {
            // das element hat kinder, also wird die länge des counters um eins erweitert, das
            // erweiterte element beschreibt die menge an kindern des aktuellen elements.
            // durch die struktur der queue (dfs) wird damit garantiert, das im nächsten durchgang
            // der schleife ein kind des aktuellen elements bearbeitet wird.
            counter.push(current_elem.number_of_creations());
        } else {
            // variable zum reduzieren einer abfolge von nullen am ende des vektors.
            let mut end_index = counter.len()-1;

            // die bearbeiteten knotenpunkte werden mithilfe der end_index variable aus dem counter
            // entfernt. 
            while counter[end_index] == 0 {
                counter.pop();
                // hier gibt es einen break falls es sich um das letzte element im counter handelt,
                // dadurch wird nut verhindert das eine usize variable negativ wird.
                if end_index == 0 {break;}
                end_index -= 1;
            }
        }
    }

    // a
    //    b
    //       c
    //          d
    //    e
    //
    //                                  [1]
    // (a,0) [(b,1),(c,2),(d,3),(e,1)]  [0]   <- NOTE no decrement on root node
    // _____ [(b,1),(c,2),(d,3),(e,1)]  [0,2]  <- ich habe kinder, also push anzahl
    // (b,1) [(c,2),(d,3),(e,1)]        [0,1]  <- ich bin abgearbeitet, also decrement meine gen.
    // _____ [(c,2),(d,3),(e,1)]        [0,1,1] <- ich habe kinder, also push anzahl
    // (c,2) [(d,3),(e,1)]              [0,1,0] <- ich bin abgearbeite, alseo decr. meine gen.
    // _____ [(d,3),(e,1)]              [0,1,0,1] <- ich habe kinder, also push anzahl
    // (d,3) [(e,1)]                    [0,1,0,0] <- ich bin abgearbeitet, also decr. meine gen.
    // _____ [(e,1)]                    [0,1] <- poppe alle abgearbeiteten subbäume, blätter etc
    // (e,1) []                         [0,0] <- ich bin abgearbeitet also, decr. mene gen.
    // _____ []                         [] <- poppe alle abgearbeiteten subbäume, blätter etc
    
    let mut result = String::new();

    for line in lines.into_iter(){
        result.push_str(line.as_str());
        result.push_str("\n");
    }
    result
}

fn counter_to_prefix(counter: &Vec<usize>) -> String {
    let mut prefix = String::new();
    let max_index = counter.len()-1;
    for (index, cnt) in counter.iter().enumerate() {
            // laufe über die counter liste, behandle alle elemente außer dem letzten und dem
            // ersten gleich. Das letzte etwas anders, ignoriere das erste
            if index == 0 {continue;}

            let (cont_prefix, end_prefix) = if max_index == index {
                ("├──", "└──")
            } else {
                ("│  ", "   ")
            };

            if cnt == &0 {
                prefix.push_str(&String::from(end_prefix));
            } else {
                prefix.push_str(&String::from(cont_prefix));
            }

            // Die beiden obereren verzweigungen bedeuten das selbe wie folgendes:
            // if max_index == index {
            //     if cnt > &1 {
            //         prefix.push_str(&String::from("├──"));
            //     } else {
            //         prefix.push_str(&String::from("└──"));
            //     }
            // } else {
            //     if cnt > &1 {
            //         prefix.push_str(&String::from("│  "));
            //     } else {
            //         prefix.push_str(&String::from("   "));
            //     }
            // }
    }
    prefix
}

// f - e
//   - b -c 
//       -d
//   - a
//   [f]
//   [e]
//   [b]
//   [c]
//   [d]
//   [a]
//   
//      Stack                  Liste
//  1. [(f,0)]                []
//  2. [(e,1), (b,1), (a,1)]  [(f,0)]
//  3. [(b,1), (a,1)]         [(f,0), (e,1)]
//  4. [(c,2), (d,2), (a,1)]  [(f,0), (e,1), (b,1)]
//  5. [(d,2), (a,1)]         [(f,0), (e,1), (b,1), (c,2)]
//  6. [(a,1)]                [(f,0), (e,1), (b,1), (c,2), (d,2)]
//  7. []                     [(f,0), (e,1), (b,1), (c,2), (d,2), (a,1)]
// als stack wird Vec genutzt und die Liste ist eine queue und dafür kann VecDeque genutzt werden
//
// Nun verarbeiten wir die generierte queue [(f,0), (e,1), (b,1), (c,2), (d,2), (a,1)]
// wir bauen einen 'counter' auf, welcher ein Vektor ist
// Wann immer die Generation vom aktuellen element kleiner ist das jenes das wir als naechstes
//   bearbeiten wollen, dann fuegen wir einen neuen counter an das ende der Liste
//
// Beispiel:
// Curr  Queue                               Counter    Prefix
// (f,0) [(e,1), (b,1), (c,2), (d,2), (a,1)] []      => 
// (e,1) [(b,1), (c,2), (d,2), (a,1)]        [3]     => ├── 
// (b,1) [(c,2), (d,2), (a,1)]               [2]     => ├── 
// (c,2) [(d,2), (a,1)]                      [2, 2]  => │  ├── 
// (d,2) [(a,1)]                             [2, 1]  => │  └── 
// (a,1) []                                  [1]     => └── 
//
// Interpretation des counters:
// fuer jedes element des counter kommt ein prefix dazu, wenn am ende des counter vektors, dann
//   1   => "└──"
//   > 1 => "├──"
// wenn nicht am ende des counter vectors:
//   1   => "   "
//   > 1 => "│  "

type Marker = Vec<usize>;

impl MarkedTree {
    pub fn new() -> Self {
        Self::mark_tree(Tree::new())
    }

    pub fn mark_tree(tree: Tree) -> Self {
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
           //let last_index = self.marker.len()-1;
           //self.marker[last_index]+=1;
           self.marker.push(current_sibling_index+1);
           println!(" do the sibling dance"); 
        } else {
            println!("ick bin einzelkind!");
        }
    }
    
    pub fn get_selected(&self)-> &Tree {
        let mut walker = &self.tree;

        for child_index in self.marker.iter() {
            walker = &walker.children[*child_index];
        }

        walker    
    }

    pub fn get_selected_mutable(&mut self)-> &mut Tree {
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
        self.get_selected_mutable().remove_child(current_sibling_index);
    }

    pub fn create_child(&mut self) {
        self.get_selected_mutable().add_child(Tree::new());
    }

    pub fn print_me(&self) -> String {

        let mut result = String::new();

        for line in self.tree.print_me().into_iter(){
            result.push_str(line.as_str());
            result.push_str("\n");
        }
        result
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

    pub fn print_me(&self) -> Vec<String> {
        let mut rows = Vec::<String>::new();
        rows.push(String::from("*"));
        let last_child_index = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let mut child_rows = child.print_me();
            let mut child_child_prefix = "";
            let mut child_child_child_prefix = "";
            if i+1 == last_child_index {
                child_child_prefix = "└── ";
                child_child_child_prefix = "    ";
            } else {
                child_child_prefix = "├── ";
                child_child_child_prefix = "|   ";
            }
            let mut nu_child_rows = Vec::<String>::new();
            for (i, child_row) in child_rows.iter().enumerate() {
                if i == 0 {
                    nu_child_rows.push(String::from(child_child_prefix) + child_row);
                } else {
                    nu_child_rows.push(String::from(child_child_child_prefix) + child_row);
                }
            }
            rows.append(&mut nu_child_rows);
        }
        rows
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
