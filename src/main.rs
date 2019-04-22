use std::{io, io::Write, process};

pub struct TodoElement{
    pub id : u32,
    pub text : String,
    pub done : bool
}

impl TodoElement{
    fn new(id : u32, text : String, done : bool) -> TodoElement {
        TodoElement {
            id,
            text,
            done
        }
    }
}

pub struct TodoList{
    pub entries : Vec<TodoElement>,
    pub index : u32
}

impl TodoList {
    fn add(&mut self, element : TodoElement){
        self.entries.push(element);
    }

    fn list_all(&self) {
        for element in &self.entries {
            println!("Id: {} - Status: {}\nTask: {}", element.id, if element.done {"[Completed]"} else {"[Uncompleted]"}, element.text);
        }
    }

    fn update_text(&mut self, id : u32, text : String){
        self.entries[id as usize].text = text;
    }
}

enum Commands{
    Help,
    Add(TodoElement),
    ListAll,
    UpdateText(u32, String),
    Complete(u32),
    Exit
}


fn help() {
    println!("Commands:\n 
        help | Lists the available options\n
        add [-text] | adds a new to do element with your description\n
        list | Lists all the ToDo elements\n
        update [-index] [-text] | Updates the element at index with the new text\n
        complete [-index] | Marks the element at index as completed
        exit - Close the program", );
}

fn main() {
    let mut list = TodoList {
        entries : Vec::<TodoElement>::new(),
        index : 0
    };

    println!("Super Simple To-Do-List CLI 0.1.0");

    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read line");

        let split_input : Vec<&str> = input.split(|c| c=='\n' || c=='-').collect();

        if split_input.len() == 0 {
            continue;
            println!("Shouldn't reach this", )
        }
        let command = match split_input[0].trim() {
            "list" => Commands::ListAll,
            "add" => {
                let cmd = Commands::Add(TodoElement::new(list.index, split_input[1].to_string(), false));
                list.index = list.index + 1;
                cmd
            },
            "update" => {
                Commands::UpdateText(split_input[1].parse::<u32>().unwrap(), split_input[2].to_string())
            },
            "complete" => {
                Commands::Complete(split_input[1].parse::<u32>().unwrap())
            }
            "exit" => Commands::Exit,
            _ => Commands::Help
        };

        match command {
            Commands::ListAll => list.list_all(),
            Commands::Add(task) => list.add(task),
            Commands::Help => help(),
            Commands::UpdateText(id, text) => list.update_text(id, text),
            Commands::Complete(id) => list.entries[id as usize].done = true,
            Commands::Exit => process::exit(0)
        }
        
    }
}
