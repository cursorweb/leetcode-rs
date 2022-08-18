mod practice;
use practice::*;
use invert_tree::Tree;

fn main() {
    println!("Paths(4, 5): {}", paths::paths(4, 5));
    println!("Fizzbuzz(15): {:?}", fizzbuzz::fizzbuzz(15));
    println!("Invert_Tree({}) {:?}", "{5: {6: {7, 8}, 7}}", invert_tree::invert_tree(Tree {
        val: 5,
        left: Some(Box::new(Tree {
            val: 6,
            left: Some(Box::new(Tree { val: 7, left: None, right: None })),
            right: Some(Box::new(Tree { val: 8, left: None, right: None }))
        })),
        right: Some(Box::new(Tree {
            val: 7,
            left: None,
            right: None
        }))
    }));
}
