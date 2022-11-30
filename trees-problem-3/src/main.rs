struct Node {
    data: String,
    children: Vec<Node>,
}

impl Node {
    fn insert(&mut self, edge: &(String, String)) -> bool {
        let mut inserted = false;
        if self.data == edge.0 {
            self.children.push(Node {
                data: edge.1.to_owned(),
                children: Vec::new(),
            });
            inserted = true;
        } else {
            for i in 0..self.children.len() {
                let child = &mut self.children.get(i);
                inserted = match child {
                    &mut Some(node) => node.insert(&edge),
                    &mut None => return false,
                }
            }
        }
        inserted
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("Start entering your list of edges where the first item is the parent and the second item is the child.");
    println!(
        "Enter the edges in the order you wish them to be in the tree. eg. 'A B', 'A C', 'B D', 'B E'"
    );
    let mut root: Option<Node> = None;
    loop {
        let input = prompt_input("Please enter an edge: ")?;
        match input.as_str() {
            "done" => break,
            "q" => return Ok(()),
            _ => {
                root = parse_input(input, root);
                continue;
            }
        };
    }
    Ok(())
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

fn parse_input(input: String, mut root: Option<Node>) -> Option<Node> {
    let input: Vec<&str> = input.split(" ").collect();
    if input.len() != 2 {
        println!(
            "Please enter an edge with a parent and a child separeted by a space. eg. 'A B'\n"
        );
        return root;
    }
    let edge = (input[0].to_string(), input[1].to_string());
    match root {
        Some(mut node) => insert_into_tree(&mut node, edge),
        None => {
            root = Some(Node {
                data: edge.0,
                children: Vec::new(),
            })
        }
    }
    root
}

fn insert_into_tree(parent: &mut Node, edge: (String, String)) {
    if parent.data == edge.0 {}
    for child in parent.children.iter() {}
}
