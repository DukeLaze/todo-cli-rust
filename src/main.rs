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
    Error(String),
    Exit
}


fn help() {
    println!("Commands:\n 
        help | Lists the available options\n
        add [-text] | adds a new ToDo element with your description\n
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

        if split_input.is_empty() {
            continue;
        }
        
        let command = match split_input[0].trim() {
            "list" => Commands::ListAll,
            "add" => {
                if !split_input[1].trim().is_empty() {
                    let cmd = Commands::Add(TodoElement::new(list.index, split_input[1].to_string(), false));
                    list.index += 1;
                    cmd
                }
                else {
                    Commands::Error("[add] requires a non-empty text parameter [-text]".to_string())
                }
            },
            "update" => {
                match split_input[1].trim().parse::<u32>() {
                    Ok(int) => Commands::UpdateText(int, split_input[2].to_string()),
                    Err(_e) => {
                    Commands::Error("The first paramter should be a positive Integer, \
                        ex. [update [-123] [-some text]]".to_string())
                    }
                }
            },
            "complete" => {

                match split_input[1].trim().parse::<u32>() {
                    Ok(int) => Commands::Complete(int),
                    Err(_e) => {
                    Commands::Error("The first paramter should be a positive Integer, \
                        ex. [complete [-123]]".to_string())
                    }
                }
            }
            "exit" => Commands::Exit,
            "help" => Commands::Help,
            _ => Commands::Error("Input not recognized as a command. \
                Commands are case sensitive. Use command [help] for available commands.".to_string())
        };

        match command {
            Commands::ListAll => list.list_all(),
            Commands::Add(task) => list.add(task),
            Commands::Help => help(),
            Commands::UpdateText(id, text) => list.update_text(id, text),
            Commands::Complete(id) => list.entries[id as usize].done = true,
            Commands::Error(e) => println!("{}", e),
            Commands::Exit => process::exit(0)
        }
        
    }
}
