// #![deny(clippy::pedantic)]

use std::io;
use std::io::Error;

use redblacktree::RedBlackTree;

fn main() -> Result<(), Error> {
    let mut tree = RedBlackTree::new();
    loop {
        let mut input = String::new();
        println!("input: ");
        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong");

        let args: Vec<&str> = input.split_whitespace().collect();

        let command: (&str, u32) = (
            args[0],
            if args.len() > 1 {
                args[1].parse().unwrap_or_default()
            } else {
                0
            },
        );

        match command.0 {
            "insert" | "add" | "put" => {
                tree.insert(command.1);
            }
            "print" | "display" | "show" => {
                tree.print();
            }
            "height" => {
                println!("max height: {}", tree.height());
            }
            "red" => {
                println!("{} red nodes", tree.red_count());
            }
            "remove" | "delete" => {
                tree.remove(command.1);
            }
            "exit" | "break" | "quit" => break,
            _ => {
                println!("what?");
            }
        };

        
    }
    Ok(())
}
