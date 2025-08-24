use core::task;
use std::{io, ops::Add};

enum Actions {
    Add,
    Remove,
    List,
    Quit,
}

struct Task {
    id_number: u32,
    description: String,
    status: (String, String),
}

fn main() {
    let mut _text: Vec<Task> = Vec::new();

    task_list(&mut _text);
}

fn task_list(_text: &mut Vec<Task>) {
    loop {
        let mut prompt = String::new();

        let desc = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let actions: Actions = match prompt.trim() {
        "add" => Actions::Add,
        "remove" => Actions::Remove,
        "list" => Actions::List,
        "quit" => Actions::Quit,
        _ => {
            println!("Unknown Command");
            continue;
        }
    };

        match actions {
            Actions::Add => {
                let task = Task {
                    id_number: (_text.len() + 1) as u32,
                    description: desc.clone(),
                    status: ("Done".to_string(), "Pending".to_string()),
                };
                println!("Task added: {}", desc);
                _text.push(task);
            }
            Actions::Remove => {
                if !_text.is_empty() {
                    let removed_id = _text[0].id_number;
                    _text.remove(0);
                    println!("Task {} removed.", removed_id);
                } else {
                    println!("No task to remove.");
                }
            }
            Actions::List => {
                for t in _text.iter() {
                    println!("{} {} {}", t.id_number, t.description, t.status.1);
                }
            }
            Actions::Quit => {
                break;
            }
        }
    }
}
