use crate::*;
use fun::*;
use practice::*;

use std::collections::HashMap;

macro_rules! category {
    ($cat:tt $(, $name:literal => $code:block )+) => {
        pub fn $cat(funcs: &mut HashMap<String, fn() -> ()>) -> (String, Vec<String>) {
            let mut out = Vec::new();

            $(
                out.push($name.to_string());
                funcs.insert($name.to_string(), || {
                    $code
                });
            )*

            (stringify!($cat).into(), out)
        }
    };
}

category!(
    practice,
    "paths" => {
        println!("Paths(4, 5): {}", paths::paths(4, 5));
        println!("SlowPaths(3, 3): {}", paths::slow_path(3, 3));
    },
    "fizzbuzz" => {
        println!("Fizzbuzz(15): {:?}", fizzbuzz::fizzbuzz(15));
    },
    "invert-tree" => {
        use invert_tree::Tree;
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
    },
    "ransom" => {
        println!("Make ransom 'you suck' out of 'hello ransom world': {}", ransom::can_construct("you suck".into(), "hello ransom world".into()));
        println!("Make ransom 'hello' out of 'the bellows': {}", ransom::ransom("hello".into(), "the bellows".into()))
    },
    "twosum" => {
        println!("[3, 2, 4] (6): {:?}", twosum::two_sum(vec![3, 2, 4], 6));
        println!("[-1, -2, -3, -4, -5] (-9): {:?}", twosum::two_sum(vec![-1, -2, -3, -4, -5], -9));
    }
);

category!(
    fun,
    "linked-list" => {
        use linkedlist::LList;
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
    },
    "merge-sort" => {
        let v = vec![3, 19, 18, 16, 6, 15, 12, 14, 11, 1];
        let s = format!("{:?}", v);
        println!("Merge sort: {} -> {:?}", s, mergesort::merge_sort(v));

        let v = vec![15, 18, 16, 4, 10, 10, 12, 11, 15, 16];
        let s = format!("{:?}", v);
        println!(
            "Merge sort: {} -> {:?}",
            s,
            mergesort::merge_sortf(v, |a: &i32, b: &i32| a < b && (a - b).abs() > 2)
        );
    },
    "dict-req" => {
        let mut parent = dictreq::Environ::new();
        parent.set("log".into(), 6);

        let mut env = dictreq::Environ::new();
        env.set("my_var".into(), 5);
        env.set_enc(parent);

        println!("my_var = {}", env.get(&"my_var".into()).unwrap());
        println!("log = {}", env.get(&"log".into()).unwrap());
        println!("null = {:?}", env.get(&"null".into()));

        env.assign("log".into(), 7).unwrap();
        println!("log = {}", env.get(&"log".into()).unwrap());

        match env.assign("null".into(), 7) {
            Err(()) => println!("null is undef"),
            Ok(()) => {}
        }
    },
    "finally" => {
        use finally::Counter;
        let mut counter = Counter::new();

        loop {
            match counter.incr() {
                Ok(v) => println!("x is {v}"),
                Err(e) => {
                    println!("'{e}'");
                    break
                },
            }
        }

        match counter.incr() {
            Ok(_) => {}
            Err(_) => {}
        }

        println!("Calls: {}", counter.calls);
    }
);
