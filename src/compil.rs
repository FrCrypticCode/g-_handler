use std::thread;
use std::fs::{metadata,OpenOptions,remove_file};
use std::io::{BufWriter,Write};
use std::process::{Command,Stdio};
use crate::conf::Conf;

pub fn compile()->Result<Vec<String>,()>{
    let config = Conf::new();
    let mut compt = 0;
    let mut res:Vec<bool> = vec![];
    if config.target.len() == config.output.len(){
        let mut list:Vec<String> = vec![];
        for t in config.target{
            let o = &config.output[compt];
            res.push(launch_comp(&config.path, t, &o));
            list.push(o.to_owned());
            compt +=1;
        }
        for b in res{
            if !b{
                return Err(())
            }
        }
        return Ok(list)
    }
    else{
        panic!("Erreur de configuration du conf.");
    }
}

fn launch_comp(path:&String,t:String,o:&String)->bool{
    match Command::new("g++").current_dir("./")
    .env("PATH", path)   // 
    .env("CC", "g++")
    .arg(t)
    .arg("-o")
    .arg(o)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped()).spawn(){
        Ok(x)=>{
            match x.wait_with_output(){
                Ok(y)=>{
                    if !y.status.success(){
                        let task = thread::spawn(||{comp_log(y.stderr)});
                        println!("Erreur lors de la compilation, veuillez consulter LogsG++");
                        let _ = task.join();
                        return false;
                    }
                    else{
                        let task = thread::spawn(||{comp_log(vec![])});
                        println!("Compilation réussie.");
                        let _ = task.join();
                        return true;
                    }
                },
                Err(err)=>{
                    println!("Erreur : {}",err.to_string());
                    panic!();
                }
            }
        },
        Err(err)=>{
            println!("Erreur : {}",err.to_string());
            panic!()
        }
    }
}

fn comp_log(log:Vec<u8>){
    if log.len() == 0{
        match metadata("LogsG++.txt"){
            Ok(_x)=>{
                if let Err(err) = remove_file("LogsG++.txt"){
                    println!("Erreur suppression log obsolète : {}",err.to_string())
                }
            },
            Err(_err)=>{}
        }
        
    }
    else{
        if let Ok(mut f) = OpenOptions::new().write(true).open("LogsG++.txt"){
            let mut writer = BufWriter::new(&mut f);
            if let Err(err) = writer.write_all(&log){
                println!("Erreur dans l'écriture du log : {}",err.to_string());
            }
        }
        else{
            let f = OpenOptions::new().create_new(true).write(true).open("LogsG++.txt");
            if f.is_ok(){
                let mut f = f.unwrap();
                let mut writer = BufWriter::new(&mut f);
                if let Err(err) = writer.write_all(&log){
                    println!("Erreur dans l'écriture du log : {}",err.to_string());
                }
            }
            else{
                panic!("Problème sur la conception du fichier : {}",f.unwrap_err())
            }
        }
    }
}