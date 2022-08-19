mod practice;
use invert_tree::Tree;
use linkedlist::LList;
use practice::*;

fn main() {
    println!("Paths(4, 5): {}", paths::paths(4, 5));
    println!("Fizzbuzz(15): {:?}", fizzbuzz::fizzbuzz(15));
    println!(
        "Invert_Tree({}) {:?}",
        "{5: {6: {7, 8}, 7}}",
        invert_tree::invert_tree(Tree {
            val: 5,
            left: Some(Box::new(Tree {
                val: 6,
                left: Some(Box::new(Tree {
                    val: 7,
                    left: None,
                    right: None
                })),
                right: Some(Box::new(Tree {
                    val: 8,
                    left: None,
                    right: None
                }))
            })),
            right: Some(Box::new(Tree {
                val: 7,
                left: None,
                right: None
            }))
        })
    );

    {
        let mut llist = LList::new(5);
        llist.append(6);
        llist.append(7);
        llist.append(8);

        print!("[ ");
        for x in llist {
            print!("{} ", x);
        }
        print!("]\n");

        let mut llist = LList::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        llist.delete(8).unwrap();
        llist.set(0, 0).unwrap();
        println!("[0]: {}", llist.get(0).unwrap());

        print!("[ ");
        for x in llist {
            print!("{} ", x);
        }
        print!("]\n");
    }
}
