use std::collections::HashMap;

mod utils;
use utils::types::TaskStruct;

fn main() {
    println!("ToDoList");

    let mut tasks: HashMap<i32, TaskStruct> = HashMap::new();
    let mut next_task_id: i32 = 1;

    let choice = ["add task", "View all tasks", "Update task", "done task", "Exit"];
    let max: usize = choice.len();

    loop {
        for (index, value) in choice.iter().enumerate(){
            println!("{}. {}", index +1 , value);
        }
        
        // choice
        let choice: usize = loop {
            let mut choice_mode: String = String::new();
            std::io::stdin().read_line(&mut choice_mode).expect("Erreur lors de la lecture");

            let input = choice_mode.trim().parse::<usize>();

            match input {
                Ok(num ) if num > 0 && num <= max => {
                    break num;
                }
                _ => {
                    println!("Veuillez entrer un nombre entre 1 et {} !", max);
                }
            }
        };

        // match action
        match choice {
            1 => utils::actions::add_task_process(&mut tasks, &mut next_task_id),
            2 => utils::actions::print_all_task(&mut tasks),
            3 => utils::actions::edit_process(&mut tasks),
            4 => utils::actions::mark_complete_process(&mut tasks),
            5 => {
                println!("A bientôt !!!");
                break;
            },
            _ => println!("Aucun choix valide !")
        }
    }
}