use std::process::Command;
use chrono::{DateTime, Utc};

struct Task {
    done: bool,
    name: String,
    created_at: DateTime<Utc>
}


fn main() {
    println!("ToDoList");

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1. Add task");
        println!("2. View all tasks");
        println!("3. Exit");
        println!("Enter your choice: ");

        let choice: u8 = loop {
            let mut choice_mode = String::new();
            std::io::stdin().read_line(&mut choice_mode).expect("Erreur lors de la lecture");
            let input = choice_mode.trim().parse::<u8>();
            match input {
                Ok(num) if num > 0 && num < 4 => {
                    break num;
                }
                _ => {
                    println!("Veuillez entrer un nombre entre 1 et 3 !");
                }
            }
        };

        match choice {
            1 => add_task(&mut tasks),
            2 => print_all_task(&mut tasks),
            3 => {
                println!("A bientôt");
                break;
            },
            _ => println!("Aucun choix valide !")
        }
    }
}

fn print_all_task(tasks: &mut Vec<Task>){
    clear_console();
    println!("Liste des tâches :");
    for item in tasks {
        println!("Status : {} - name : {} - crée le : {}", 
        item.done,
        item.name,
        item.created_at
        )
    }
}

fn add_task(tasks: &mut Vec<Task>){
    clear_console();
    println!("Nom de votre nouvelle tâche :");
    let mut new_task: String = String::new();
    std::io::stdin().read_line(&mut new_task).expect("Erreur lors de la lecture");
    let tmp_new_task: String = new_task.trim().to_string();
    let tmp_task: Task = Task {
        done: false,
        name: tmp_new_task.clone(),
        created_at: Utc::now()
    };
    tasks.push(tmp_task);
    println!("Tâches ajoutées : {}", tmp_new_task);
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Erreur lors de la commande pour nettoyer la console");
    } else {
        Command::new("clear")
            .status()
            .expect("Erreur lors de la commande pour nettoyer la console");
    }
}
