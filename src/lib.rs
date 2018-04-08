pub mod tree;

#[cfg(test)]
mod tests {

    use super::tree::Tree;

    #[test]
    fn test_contains() {
        let mut tree: Tree<String> = Tree::new();
        tree.insert(String::from("C"));
        tree.insert(String::from("B"));
        tree.insert(String::from("D"));
        assert!(!tree.contains(&String::from("A")));
        assert!(tree.contains(&String::from("B")));
        assert!(tree.contains(&String::from("C")));
        assert!(tree.contains(&String::from("D")));
        assert!(!tree.contains(&String::from("E")));
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut tree: Tree<String> = Tree::new();
        tree.insert(String::from("B"));
        tree.insert(String::from("A"));
        tree.insert(String::from("C"));
        println!("Tree: {:?}", tree);
        let old_size = tree.size();
        assert_eq!(Some(String::from("B")), tree.delete(&String::from("B")));
        assert_eq!(old_size - 1, tree.size());
        println!("Tree: after delete {:?}", tree);
    }

    #[test]
    fn remove_unknown_node() {
        let mut tree: Tree<String> = Tree::new();
        tree.insert(String::from("B"));
        tree.insert(String::from("A"));
        tree.insert(String::from("C"));
        println!("Tree: {:?}", tree);
        let old_size = tree.size();
        assert_eq!(None, tree.delete(&String::from("D")));
        assert_eq!(old_size, tree.size());
        println!("Tree: after delete {:?}", tree);
    }

    #[test]
    fn remove_from_empty_tree() {
        let mut tree: Tree<String> = Tree::new();
        println!("Tree: {:?}", tree);
        assert_eq!(0, tree.size());
        assert_eq!(None, tree.delete(&String::from("D")));
        assert_eq!(0, tree.size());
        println!("Tree: after delete {:?}", tree);
    }

    #[test]
    fn iterate() {
        let mut tree: Tree<String> = Tree::new();
        tree.insert(String::from("D"));
        tree.insert(String::from("A"));
        tree.insert(String::from("B"));
        tree.insert(String::from("C"));
        tree.insert(String::from("F"));
        tree.insert(String::from("E"));
        tree.insert(String::from("G"));
        let mut s = String::new();
        for c in &tree {
            s += c;
        }
        assert_eq!("ABCDEFG", s);
    }

}
