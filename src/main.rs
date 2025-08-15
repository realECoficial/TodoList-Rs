use chrono::prelude::*;
use std::path::Path;
use std::fs;
use std::io;
use users::{get_user_by_uid, get_current_uid};


fn current_time(){
    let dt1: DateTime<Local> = Local::now();
    println!("{}",dt1.format("Fecha: %Y-%m-%d \nHora: %H:%M:%S").to_string());
   
}

fn main()  {
#[warn(unused_must_use)]
    let mut letters: HashMap<char, i32> = HashMap::new();
   //TODO tengo hacer un match que tome el dia y segun eso, actualizar la visibilidad de las tareas
   //eje: lunes: rust = 1 matematicas = 1 filosofia = 0
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hola, {:?}!", user.name());
    current_time();     
   
    println!("");
    
    println!("Estas son las cosas que tienes que hacer hoy:");


} 
