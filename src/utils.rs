use std::{process::Command, usize};
use chrono::Utc;
use crate::task_struct::Task;

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

pub fn print_all_task(tasks: &mut Vec<Task>){
    clear_console();
    println!("Liste des tâches :");
    for item in tasks {
        println!("Status : {} - name : {} - crée le : {}", 
        if item.done {"Terminé"} else {"Ouvert"},
        item.name,
        item.created_at.format("%d/%m/%Y")
        )
    }
}

pub fn add_task(tasks: &mut Vec<Task>){
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

pub fn edit_task(tasks: &mut Vec<Task>){
    clear_console();
    println!("Choix de la tâche à modifier :");
    let quantity: usize = tasks.len();

    let choice: usize = loop {
        for i in 1..quantity {
            println!("{} - {}", i, tasks[i].name);
        }
        let mut choice_mode: String = String::new();
        std::io::stdin().read_line(&mut choice_mode).expect("Erreur lors de la lecture");
        let input = choice_mode.trim().parse();
        match input {
            Ok(num) if num > 0 && num < quantity => {
                break num;
            }
            _ => {
                println!("Veuillez entrer un nombre entre 1 et {} !", quantity);
            }
        }
    };
    
    let before_name: String = tasks[choice].name.clone();
    let mut new_name: String =  String::new();

    let mut tmp_task: String = String::new();
    std::io::stdin().read_line(&mut tmp_task).expect("Erreur lors de la lecture");
    new_name = tmp_task.trim().to_string();
    tasks[choice].name = new_name.clone();

    println!("{} et renommer par {}", before_name, new_name);

}
