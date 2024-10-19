mod task_struct;
use crate::task_struct::Task;
mod utils;

fn main() {
    println!("ToDoList");

    let mut tasks: Vec<Task> = Vec::new();
    let max: u8 = 4;

    loop {
        // listing action
        println!("1. Add task");
        println!("2. View all tasks");
        println!("3. Update task");
        println!("4. Exit");
        println!("Enter your choice: ");
        
        // choice
        let choice: u8 = loop {
            let mut choice_mode: String = String::new();
            std::io::stdin().read_line(&mut choice_mode).expect("Erreur lors de la lecture");
            let input = choice_mode.trim().parse::<u8>();
            match input {
                Ok(num) if num > 0 && num <= max => {
                    break num;
                }
                _ => {
                    println!("Veuillez entrer un nombre entre 1 et {} !", max);
                }
            }
        };

        // match action
        match choice {
            1 => utils::add_task(&mut tasks),
            2 => utils::print_all_task(&mut tasks),
            3 => utils::edit_task(&mut tasks),
            4 => {
                println!("A bientôt");
                break;
            },
            _ => println!("Aucun choix valide !")
        }
    }
}