use users::{get_user_by_uid, get_current_uid};

use chrono::prelude::*;
use chrono::Weekday::*;
use std::collections::HashMap;
use chrono::WeekdaySet;


const MTWRFSU: [char; 7] = ['M', 'T', 'W', 'R', 'F', 'S', 'U'];

fn current_time(){
    let dt1: DateTime<Local> = Local::now();
    println!("{}",dt1.format("Fecha: %Y-%m-%d \nHora: %H:%M:%S").to_string());
   
}
fn week_days(task: &mut HashMap<&str, i32>) {
    
    let today = Local::now().weekday();
//for i in varr.iter(Mon) {
    match today {
        Mon => task.insert("Matematicas",0),//Matematica
        Tue => task.insert("Rust",0),//Rust 
        Wed => task.insert("Filosofia",0),//Filosofia 
        Thu => task.insert("Matematicas",0),//Matematica
        Fri => task.insert("Rust",0),//Rust 
        Sat => task.insert("Filosofia",0),//Filosofia 
        _ => unreachable!(), 
    };
    //}
}
fn main()  {
    let mut letters: HashMap<&str, i32> = HashMap::new();
    letters.insert("Rust",1);//Rust 
    letters.insert("Filosofia",1);//Filosofia 
    letters.insert("Matematicas",1);//Matematicas 
    
    //TODO tengo hacer un match que tome el dia y segun eso, actualizar la visibilidad de las tareas
   //eje: lunes: rust = 1 matematicas = 1 filosofia = 0
     
    week_days(&mut letters);

    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hola, {:?}!", user.name());
    current_time();     
   
    println!("");
    println!("Estas son las cosas que tienes que hacer hoy: ");
    println!("");
     
    for (key, value) in &letters {
        if *value != 0 {
            println!("\tTask: {}", key);
        } 
    }
}









//unused unless i learn how to implement those

/*
 *
use std::path::Path;
use std::fs;
use std::io;
 * */





