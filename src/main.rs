use std::vec;
use std::io;
fn main() {
    println!("Hello, world!, welcome to my Todo list cli");
    let mut todo_list = TodoItems {
        items: Vec::new(),
    };
    let (title, description, priority) = get_user_input();

    let item = build_todo_item(title, description, priority);

    todo_list.add_item(item);
    let g_title: &str = "Go to Dinner";

    let item1 = todo_list.get_item(&g_title);

    match item1 {
        Some(val) => println!("{}", val.repr()),
        None => println!("Could not find item"),
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
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
    fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
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

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn show_items(&self) {
        for x in &self.items {
            println!("{}", x.repr());
            println!("//////////////////////////////");
        }
    }
}
