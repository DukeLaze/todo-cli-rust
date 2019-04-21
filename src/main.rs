use std::{vec, io, process};

struct TodoList{
    entries : Vec<String>
}

impl TodoList {
    fn add(&mut self, element : String){
        self.entries.push(element);
    }

    fn list_all(&self) {
        for element in &self.entries {
            println!("{}", element);
        }
    }
}

enum Commands{
    Help,
    Add(String),
    ListAll,
    Exit
}

fn main() {
    let mut list = TodoList {
        entries : Vec::<String>::new()
    };

    println!("Super Simple To-Do-List CLI 0.1.0\n >> ");

    loop{
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Unable to read line");

        let split_input : Vec<&str> = input.split_whitespace().collect();

        let command = match split_input[0] {
            "list" => Commands::ListAll,
            "add" => Commands::Add(split_input[1].to_string()),
            "exit" => Commands::Exit,
            _ => Commands::Help
        };

        match command {
            Commands::ListAll => list.list_all(),
            Commands::Add(task) => list.add(task),
            Commands::Help => println!("Nope"),
            Commands::Exit => process::exit(0)
        }
        
    }
}
