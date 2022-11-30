// Author: Grant Duchars
mod tree;
use tree::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start entering your list of edges where the first item is the parent and the second item is the child.");
    println!(
        "Enter the edges in the order you wish them to be in the tree. eg. 'A B', 'A C', 'B D', 'B E'"
    );
    println!("You will also have to give which side of the parent the child node will go. eg. 'A B (L or R)'\n");
    let mut tree = Tree::new();
    // Loop to build the tree from user input
    loop {
        let input = prompt_input("Please enter an edge: ")?;
        match input.as_str() {
            "done" => break,
            "q" => return Ok(()),
            _ => {
                let edge = match parse_input(input) {
                    Some(edge) => edge,
                    None => {
                        println!("Please enter a parent, a child, and whether it is left or right for each edge. eg. 'A B (L or R)'\n");
                        continue;
                    }
                };
                match tree.insert(edge) {
                    true => (),
                    false => println!("Unable to insert given edge into the tree.\n"),
                };
                continue;
            }
        };
    }
    println!();
    // Loop to print out tree in preorder, inorder, and postorder traversal
    loop {
        let input = prompt_input("Which order for printing: ")?;
        match input.as_str() {
            "pre" => tree.print_preorder(),
            "in" => tree.print_inorder(),
            "post" => tree.print_postorder(),
            "q" => return Ok(()),
            _ => {
                println!("Please enter which order you want to print out the tree. The options are 'pre', 'in', or 'post'.\n");
                continue;
            }
        }
    }
}

/// Function to prompt the user with given string and return user's input with trailing newlines removed
fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{prompt}");
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.trim_end_matches(['\r', '\n']).len());
    Ok(input)
}

fn parse_input(input: String) -> Option<(String, String, bool)> {
    let input: Vec<&str> = input.split(" ").collect();
    if input.len() != 3 {
        return None;
    }
    let left = match input[2] {
        "L" => true,
        "R" => false,
        _ => return None,
    };
    Some((input[0].to_string(), input[1].to_string(), left))
}
