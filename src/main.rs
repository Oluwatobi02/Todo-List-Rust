use std::io::{self, Write};
use std::fs::{read_to_string, File};

fn main() {
    println!("Hello, world!, welcome to my Todo list cli");
    let mut todo_list = TodoItems {
        items: Vec::new(),
    };
    todo_list.load_items();
    loop {

        let task = ask_user();
        match task.trim() {
        "create" => create_task(&mut todo_list),
        // "update" => update_task(),
        "delete" => delete_task(&mut todo_list),
        "read" => read_task(&todo_list),
        "read all" => todo_list.show_items(),
        _ => println!("Invalid input")
    }
} 
}

fn create_task(todo_list: &mut TodoItems) {
    let (title, description, priority) = get_user_input();
    let item = build_todo_item(title, description, priority);
    todo_list.add_item(item, "self");
}
fn delete_task(todo_list: &mut TodoItems) {
    let mut item_to_delete = String::new();
    println!("What do you want to delete?");
    io::stdin().read_line(&mut item_to_delete).expect("Enter a valid task");
    let result = todo_list.delete_item(item_to_delete.trim());
    match result {
        Some(_val) => println!("Deleted item!"),
        _ => println!("Could not find item"),
    }
}

fn read_task(todo_list: &TodoItems){
    let mut item_to_read = String::new();
    println!("What do you want to read?");
    io::stdin().read_line(&mut item_to_read).expect("Enter a valid task");

    let item1 = todo_list.get_item(item_to_read.trim());

    match item1 {
        Some(val) => println!("{}", val.repr()),
        _ => println!("Could not find item"),
    }  
}
fn ask_user() -> String {
    loop {
    let mut task = String::new();
    println!("What do you want to do? [create || update || delete || read || read all]");
    io::stdin().read_line(&mut task).expect("Enter a valid task");

    match task.trim() {
        "create" => return task.to_string(),
        "update" => return task.to_string(),
        "delete" => return task.to_string(),
        "read" => return task.to_string(),
        "read all" => return task.to_string(),
        _ => println!("Please enter a valid task")
    }
    }

}
fn build_todo_item(title: String, description: String, priority: i32) -> Item {
    let item  = Item {
        title,
        description,
        priority
    };
    return item;
}

fn get_user_input() -> (String, String, i32) {
    
    loop {
        let mut title = String::new();
        let mut description = String::new();
        let mut priority = String::new();
        println!("Enter the title of your todo item");
        io::stdin().read_line(&mut title).expect("Enter a valid title");

        println!("Enter the description of your todo item");
        io::stdin().read_line(&mut description).expect("Enter a valid description");

        println!("Enter the priority of your todo item");
        io::stdin().read_line(&mut priority).expect("Enter a valid priority");

        title = title.trim().to_string();
        description = description.trim().to_string();
        
        if title.is_empty() || description.is_empty(){
            println!("Title and description cannot be empty. Please try again.");
            continue;
        }
        let priority: i32 = match priority.trim().parse() {
            Ok(num) if num >= 1 && num <= 10 => num,
            _ => {
                println!("Priority must be an integer between 1 and 10. Please try again.");
                continue;
            }
        };
        
                return (title, description, priority);
            }
}




struct Item {
    title: String,
    description: String,
    priority: i32,
}

impl Item {
    fn get_description(&self) -> &String {
        &self.description
    }
    fn get_priority(&self) -> &i32 {
        &self.priority
    }
    fn get_title(&self) -> &String {
        &self.title
    }
    // fn set_title(&mut self, title: String) {
    //     self.title = title;
    // }
    // fn set_description(&mut self, description: String) {
    //     self.description = description;
    // }
    // fn set_priority(&mut self, priority: i32) {
    //     self.priority = priority;
    // }
    
    fn repr(&self) -> String{
        let res = format!("Title: {} \n Description: {} \n Priority: {}", &self.get_title(), &self.get_description(), &self.get_priority());
        res
    }
}


struct TodoItems {
    items: Vec<Item>,
}

impl TodoItems {
    fn get_item(&self, title: &str) -> Option<&Item> {
        for item in self.items.iter() {
            if item.get_title() == title {
                return Some(item);
            }
        }
        return None;
    }

    fn add_item(&mut self, item: Item, from: &str) {
        self.items.push(item);
        let _ = self.write_to_file(from);
    }

    fn delete_item(&mut self, title: &str) -> Option<bool> {
        if let Some(item_position) = self.items.iter().position(|item| item.get_title() == title) {
            self.items.swap_remove(item_position);
            let _ = self.write_to_file("self");
            return Some(true);
        }
        else {
            return None
        }
    }

    fn show_items(&self) {
        for x in &self.items {
            println!("{}", x.repr());
            println!("//////////////////////////////");
        }
    }

    fn load_items(&mut self) {
        let file_name = "db.txt";

        let content = match read_to_string(file_name) {
            Ok(data) => data,
            Err(_) => {
                println!("Could not read the file. Starting with an empty list.");
                return;
            }
        };

        let mut lines = content.lines();

        while let Some(title) = lines.next() {
            let description = match lines.next() {
                Some(desc) => desc.trim().to_string(),
                _ => {
                    println!("Incomplete data for item: {}", title);
                    continue;
                }
            };

            let priority = match lines.next() {
                Some(pri) => pri.trim().parse::<i32>().ok(),
                _ => None,
            };

            // Check if priority is valid
            if let Some(priority) = priority {
                let item = Item {
                    title: title.trim().to_string(),
                    description,
                    priority,
                };
                self.add_item(item, "load items");
            } else {
                println!("Invalid priority for item: {}", title);
            }

            // Skip the delimiter line (if it exists)
            if let Some(delimiter) = lines.next() {
                if delimiter.trim() != "/////////////////////////////////////////////////////////////" {
                    println!("Unexpected format. Skipping malformed item.");
                }
            }
        }

        println!("Loaded items from the file.");
    }
    fn write_to_file(&self, from : &str) -> io::Result<()> {
        if from != "load items" {
            let file_name = "db.txt";
            let mut file = File::create(file_name)?;
            for x in &self.items {
                writeln!(file, "{}", x.get_title())?;
                writeln!(file, "{}", x.get_description())?;
                writeln!(file, "{}", x.get_priority())?;
                writeln!(file, "/////////////////////////////////////////////////////////////")?;
                println!("done writing to file");
            }
        }
        Ok(())
    }
}
