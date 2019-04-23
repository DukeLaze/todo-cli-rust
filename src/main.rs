use std::{io, io::Write, io::prelude::*, io::BufReader, process, fs, fs::File, env};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList{
    pub entries : Vec<TodoElement>,
    pub index : u32
}

impl TodoList {

    fn from_file(filepath : String) -> Option<TodoList> {
        let file = match File::open(&filepath) {
            Ok(f) => f,
            Err(e) => {
                println!("Error reading from file: {} - {}", filepath, e);
                return None;
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        match buf_reader.read_to_string(&mut content) {
            Ok(_bytes) => (),
            Err(e) => {
                println!("Error reading from file: {} - {}", filepath, e);
                return None;
            }
        };

        match serde_json::from_str(&content) {
            Ok(t) => t,
            Err(e) => {
                println!("Error deserializing TodoList: {}", e);
                None
            }
        }
    }

    fn to_file(&mut self, filepath : String) {
        let mut file = match File::create(filepath) {
            Ok(f) => f,
            Err(e) => {
                println!("Error when saving file: {}", e);
                return;
            }
        };

        let serialized : String = serde_json::to_string(self).unwrap();
        match file.write_all(serialized.as_bytes()) {
            Ok(_ok) => (),
            Err(e) => println!("Error when saving file: {}", e)
        };
    }

    fn add(&mut self, element : TodoElement){
        self.entries.push(element);
    }

    fn list_all(&self) {
        for element in &self.entries {
            println!("Id: {} - Status: {}\nTask: {}", element.id, 
                if element.done {"[Completed]"} else {"[Uncompleted]"}, element.text);
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
    Load(String),
    Save(String),
    New,
    Exit
}


fn help() {
    println!("Commands:\n 
        help | Lists the available options\n
        add [-text] | adds a new ToDo element with your description\n
        list | Lists all the ToDo elements\n
        update [-index] [-text] | Updates the element at index with the new text\n
        complete [-index] | Marks the element at index as completed\n
        save [-filename] | Saves the current list to the specified file.\n
        load [-filename] | Loads the specified list\n
        new | creates a new empty list\n
        exit - Close the program", );
}

fn main() {
    let data_path : String = match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.push("data/");
            match path.to_str() {
                Some(s) => s.to_string(),
                None => panic!()
            } 
        },

        Err(e) => {
            println!("Could not get executable path. Error: {}", e);
            panic!();
        }
    };

    match fs::create_dir_all(&data_path) {
        Ok(_ok) => (),
        Err(e) => {
            panic!("Error creating the data directory. Shutting down. Error: {}", e)
        }
    }

    let mut list = TodoList {
        entries : Vec::<TodoElement>::new(),
        index : 0
    };

    println!("Super Simple To-Do-List CLI 0.1.0");
    println!("Your lists will be saved here: {}", data_path);

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
                let i = match split_input[1].trim().parse::<u32>() {
                    Ok(int) => int,
                    Err(_e) => {
                        std::u32::MAX
                    }                   
                };

                if i <= list.index {
                    Commands::UpdateText(i, split_input[2].to_string())
                }

                else if i == std::u32::MAX{
                        Commands::Error("The first paramter should be a positive Integer, \
                            ex. [update [-123] [-some text]]".to_string())
                }
                
                else{
                    Commands::Error("The specified index is outside of the element range. \
                        Try [list] to check the indexes".to_string())
                }
            },
            "complete" => {
                let i = match split_input[1].trim().parse::<u32>() {
                    Ok(int) => int,
                    Err(_e) => {
                        std::u32::MAX
                    }
                };

                if i <= list.index {
                    Commands::Complete(i)
                }

                else if i == std::u32::MAX {
                        Commands::Error("The first paramter should be a positive Integer, \
                            ex. [complete [-123]]".to_string())
                }
                
                else{
                    Commands::Error("The specified index is outside of the element range. \
                        Try [list] to check the indexes".to_string())
                }
            },
            "load"  => Commands::Load(data_path.clone() + &split_input[1].trim().to_string()),
            "save"  => Commands::Save(data_path.clone() + &split_input[1].trim().to_string()),
            "exit"  => Commands::Exit,
            "new"   => Commands::New,
            "help"  => Commands::Help,
            _       => Commands::Error("Input not recognized as a command. \
                                    Commands are case sensitive. Use command [help] for available commands.".to_string())
        };

        match command {
            Commands::ListAll => list.list_all(),
            Commands::Add(task) => list.add(task),
            Commands::Help => help(),
            Commands::UpdateText(id, text) => list.update_text(id, text),
            Commands::Complete(id) => list.entries[id as usize].done = true,
            Commands::Error(e) => println!("{}", e),
            Commands::Exit => process::exit(0),
            Commands::Load(filepath) => {
                    list = match TodoList::from_file(filepath){
                        Some(list) => list,
                        None => {
                            println!("Unable to read Todo list, are you sure the file exists?", );
                            list
                        }
                    }
                },
            Commands::Save(filepath) => list.to_file(filepath),
            Commands::New => {
                list = 
                TodoList {
                    entries : Vec::<TodoElement>::new(),
                    index : 0
                };
            }
        } 
    }
}
