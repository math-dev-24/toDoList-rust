use std::{process::Command};
use chrono::Utc;
use std::collections::HashMap;

use super::types;
use types::TaskStruct;
use types::Task;
use types::TaskStatus;

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

pub fn print_all_task(tasks: &mut HashMap<i32, TaskStruct>){
    clear_console();
    println!("Liste des tâches :");
    for (index, item) in tasks.iter() {
        println!("{} - Status : {:?} - name : {} - crée le : {} \n Description : \n {}", 
        index,
        item.status(),
        item.name,
        item.created_at.format("%d/%m/%Y"),
        item.description()
        )
    }
}

pub fn add_task_process(tasks: &mut HashMap<i32, TaskStruct>, next_task_id: &mut i32){
    clear_console();
    println!("Nom de votre nouvelle tâche :");
    let mut new_task: String = String::new();
    std::io::stdin().read_line(&mut new_task).expect("Erreur lors de la lecture");
    let tmp_new_task: String = new_task.trim().to_string();

    println!("Description de votre tâches :");
    let mut new_description: String = String::new();
    std::io::stdin().read_line(&mut new_description).expect("Erreur lors de la lecture");

    add_task(tasks, tmp_new_task, new_description.trim().to_string(), next_task_id);
}

fn add_task(tasks: &mut HashMap<i32, TaskStruct>, name: String, description: String, next_task_id: &mut i32) {
    let tmp_task: TaskStruct = TaskStruct {
        name: name,
        done: false,
        created_at: Utc::now(),
        status: TaskStatus::Incomplete,
        description: description
    };
    tasks.insert(*next_task_id,tmp_task );
    *next_task_id += 1;
}


pub fn mark_complete_process(tasks: &mut HashMap<i32, TaskStruct>) {
    clear_console();
    println!("Choix de la tâches à terminé :");
    let mut index_task: String = String::new();
    std::io::stdin().read_line(&mut index_task).expect("Erreur lors de la lecture");
    match index_task.trim().parse::<i32>() {
        Ok(num) => {
            mark_complete(tasks, num);
        },
        Err(_) => {
            println!("Erreur lors de la lecture")
        }
    }
}

fn mark_complete(tasks: &mut HashMap<i32, TaskStruct>, id: i32){
    if let Some(value) = tasks.get_mut(&id) {
        value.mark_complete();
    }else {
        println!("Tâches invalide !")
    }
}

pub fn edit_process(tasks: &mut HashMap<i32, TaskStruct>) {
    clear_console();
    println!("Choix de la tâche à update :");
    let mut index_task: String = String::new();
    std::io::stdin().read_line(&mut index_task).expect("Erreur lors de la récupération !");

    match index_task.trim().parse::<i32>() {
        Ok(num) => {
            match tasks.get_mut(&num) {
                Some(task) => {
                    let mut tmp_name: String = String::new();
                    let mut tmp_desc: String = String::new();
                    println!("Nouveau nom :");
                    std::io::stdin().read_line(&mut tmp_name).expect("Erreur lors de la récupération !");
                    println!("Nouvelle description :");
                    std::io::stdin().read_line(&mut tmp_desc).expect("Erreur lors de la récupération !");
                    if tmp_name.trim() != "_" && !tmp_name.trim().is_empty() {
                        task.name = tmp_name.trim().to_string();
                    }
                    if tmp_desc.trim() != "_"  && !tmp_desc.trim().is_empty(){
                        task.description = tmp_desc.trim().to_string();
                    }
                    println!("Tâches {} mise à jours", task.name);
                },
                None => println!("Tâche inexistante !")
            }
        },
        Err(_) => {
            println!("une erreur est survenu");
        }
    }

}
