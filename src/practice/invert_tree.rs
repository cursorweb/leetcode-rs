#[derive(PartialEq, Debug)]
pub struct Tree {
    pub val: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

pub fn invert_tree(tree: Tree) -> Box<Tree> {
    if tree.left == None && tree.right == None {
        return Box::new(tree);
    } else {
        Box::new(Tree {
            val: tree.val,
            left: match tree.right {
                Some(t) => Some(invert_tree(*t)),
                None => None,
            },
            right: match tree.left {
                Some(t) => Some(invert_tree(*t)),
                None => None,
            },
        })
    }
}
