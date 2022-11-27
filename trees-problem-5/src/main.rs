#[derive(Debug)]
struct Node {
    letter: Option<char>,
    frequency: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() -> Result<(), std::io::Error> {
    // Make a vector to store letters and thier frequencies
    let mut letters: Vec<Node> = Vec::new();
    // Loop to prompt user to input a letter followed by its frequency
    loop {
        let input = prompt_input("Enter a character and frequency: ")?;
        let input = match input.as_str() {
            // When user inputs "done" exit the loop
            "done" => break,
            // When user inputs "q" quit the program
            "q" => return Ok(()),
            // When user inputs "p" use the characters and frequencies from textbook
            "p" => {
                letters = premade_letters();
                break;
            }
            _ => input.split(" "),
        };
        // When user inputs anything else, try to parse the input and add it to letters
        letters.push(match parse_input(input) {
            Some(letter) => letter,
            None => {
                println!("Please enter a character and frequency.\n");
                continue;
            }
        });
    }
    // Sort the letters by ascending frequency
    letters.sort_unstable_by(
        |Node {
             letter: _,
             frequency: a,
             left: _,
             right: _,
         },
         Node {
             letter: _,
             frequency: b,
             left: _,
             right: _,
         }| b.partial_cmp(a).expect("Error while sorting"),
    );
    // println!("{letters:?}");
    let root = build_huffman_code_tree(letters).unwrap();
    // let height = get_height(&root);
    // println!("{height}");
    print_huffman_code(
        &root,
        std::cmp::max(9, get_height(&root).try_into().unwrap()),
    );
    Ok(())
}

fn build_huffman_code_tree(mut letters: Vec<Node>) -> Option<Node> {
    while letters.len() != 1 {
        let node_a = letters.pop()?;
        let node_b = letters.pop()?;
        let new_node = Node {
            letter: None,
            frequency: node_a.frequency + node_b.frequency,
            left: Some(Box::from(node_a)),
            right: Some(Box::from(node_b)),
        };
        let mut index = letters.len();
        for node in letters.iter().rev() {
            if new_node.frequency < node.frequency {
                break;
            } else {
                index = index - 1;
            }
        }
        letters.insert(index, new_node);
    }
    Some(letters.pop()?)
}

fn print_huffman_code(root: &Node, width: usize) {
    println!("+-----------+-{:-<width$}-+", "-");
    println!("| Character | {:<width$} |", "Code Word");
    println!("+-----------+-{:-<width$}-+", "-");
    print_huffman_code_helper(root, String::new(), width);
    println!("+-----------+-{:-<width$}-+", "-");
}

fn print_huffman_code_helper(node: &Node, code: String, width: usize) {
    match &node.left {
        Some(left_node) => print_huffman_code_helper(&*left_node, code.clone() + "0", width),
        None => (),
    }
    match &node.right {
        Some(right_node) => print_huffman_code_helper(&*right_node, code.clone() + "1", width),
        None => (),
    }
    match &node.letter {
        Some(letter) => println!("|     {}     | {:>width$} |", letter, code),
        None => (),
    }
}

fn get_height(node: &Node) -> i32 {
    let mut max_child_height = -1;
    match &node.left {
        Some(left_node) => max_child_height = get_height(&*left_node),
        None => (),
    }
    match &node.right {
        Some(right_node) => {
            max_child_height = std::cmp::max(max_child_height, get_height(&*right_node));
        }
        None => (),
    }
    return max_child_height + 1;
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

/// Function to take in user input and try to parse it into a tuple of a char and a float
fn parse_input(mut input: core::str::Split<&str>) -> Option<Node> {
    Some(Node {
        letter: match input.next() {
            Some(s) => match s.chars().next() {
                Some(c) => Some(c),
                None => return None,
            },
            None => return None,
        },
        frequency: match input.next() {
            Some(s) => match s.parse::<f64>() {
                Ok(f) => f,
                Err(_) => {
                    println!("Unable to parse input, frequency was not a float.");
                    return None;
                }
            },
            None => return None,
        },
        left: None,
        right: None,
    })
}

fn premade_letters() -> Vec<Node> {
    let characters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let frequencies = [
        0.0817, 0.0145, 0.0248, 0.0431, 0.1232, 0.0209, 0.0182, 0.0668, 0.0689, 0.0010, 0.0080,
        0.0397, 0.0277, 0.0662, 0.0781, 0.0156, 0.0009, 0.0572, 0.0628, 0.0905, 0.0304, 0.0102,
        0.0264, 0.0015, 0.0211, 0.0005,
    ];
    let mut letters: Vec<Node> = Vec::new();
    for i in 0..26 {
        letters.push(Node {
            letter: Some(characters[i]),
            frequency: frequencies[i],
            left: None,
            right: None,
        })
    }
    letters
}
