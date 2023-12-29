use std::process::{Command,Stdio};
use std::io::stdin;
mod compil;
use compil::compile;
mod conf;

fn main() {
    println!("|| Lancement de la compilation...");
    match compile(){
        Ok(list)=>{
            if list.len() != 1{
                println!("Exécuter les programmes ?(Y/n)");
            }
            else{
                println!("Exécuter le programme ?(Y/n)");
            }
            let mut rep = String::new();
            let _ = stdin().read_line(&mut rep);
            match rep.to_lowercase().trim_end(){
                "y"=>{launch(list)},
                "n"=>{},
                _=>{}
            }
        },
        Err(())=>{println!("|| Fin d'exécution.")}
    }
}

fn launch(list:Vec<String>){
    for soft in list{
        match Command::new(soft).current_dir("./").stdout(Stdio::piped()).spawn(){
            Ok(x)=>{
                match x.wait_with_output(){
                    Ok(y)=>{
                        let mut compt = 0;
                        let data = String::from_utf8_lossy(&y.stdout);
                        for l in data.lines(){
                            println!("{}: {}",compt,l);
                            compt+=1;
                        }
                    },
                    Err(err)=>{println!("Erreur : {}",err)}
                }
            },
            Err(err)=>{println!("Erreur : {}",err.to_string())}
        }
    }
}

