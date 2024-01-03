use core::panic;
use std::{env,fs,io::{stdin,BufReader,Read, BufWriter, Write}};

pub struct Conf{
    os:String,
    pub path:String,
    pub output:Vec<String>,
    pub target:Vec<String>
}
impl Conf{
    pub fn new()->Conf{
        if search("g++.conf"){
            match fs::OpenOptions::new().read(true).open("g++.conf"){
                Ok(mut f)=>{
                    let mut reader = BufReader::new(&mut f);
                    let mut content = String::new();
                    let _ = reader.read_to_string(&mut content);
                    let mut os = String::new();
                    let mut path = String::new();
                    let mut output:Vec<String> = vec![];
                    let mut targets:Vec<String> = vec![];    // Handle
                    for line in content.lines(){
                        let v = line.split(':');
                        let v:Vec<&str> = v.collect();
                        match v[0].trim(){
                            "OperatingSys"=>{
                                os = v[1].trim().to_string();
                                path = define_os(&os);
                            },
                            "Target"=>{
                                let t = v[1].split(',');
                                for c in t{
                                    targets.push(c.trim().to_string());
                                }
                            },
                            "Output"=>{
                                let o = v[1].split(',');
                                for c in o{
                                    output.push(c.trim().to_string());
                                }
                            }
                            _=>{}
                        }
                    }
                    return Conf{os:os,path:path,output:output,target:targets}
                }
                Err(err)=>{
                    panic!("Erreur : {}",err.to_string());
                }
            }
        }
        else{
            let mut sys = String::new();
            let path:String;
            let os = env::var("OS");
            if let Ok(os) = os{
                sys = os;
                path = define_os(&sys);
            }
            else{
                println!("Veuillez spécifier votre Système :");
                println!("(Linux/Windows/Android/Darwin");
                let _ = stdin().read_line(&mut sys);
                sys = sys.trim_end().to_string();
                path = define_os(&sys);
                  
            }
            let mut target:Vec<String> = vec![];
            target.push("main.cpp".to_string());
            let mut output:Vec<String> = vec![];
            output.push(String::from("main"));
            let config = Conf{os:sys,path:path,output:output,target:target};
            config.make_conf();
            return config
        }
    }

    fn make_conf<'a>(&self){
        match fs::OpenOptions::new().create_new(true).write(true).open("./g++.conf"){
            Ok(f)=>{
                let mut check = false;
                let mut arr_err:Vec<String> = vec![String::new();4];
                let mut writer = BufWriter::new(&f);
                if let Err(err) = writer.write(format!("OperatingSys : {}\n",self.os).as_bytes()){
                    check=true;
                    arr_err[0]=err.to_string();
                }
                if let Err(err) = writer.write(format!("PathCompile : {}\n",self.path).as_bytes()){
                    check=true;
                    arr_err[1]=err.to_string();
                }
                if let Err(err) = writer.write(format!("Target : {}\n",self.target[0]).as_bytes()){
                    check=true;
                    arr_err[2]=err.to_string();
                }
                if let Err(err) = writer.write(format!("Output : {}",self.output[0]).as_bytes()){
                    check=true;
                    arr_err[3]=err.to_string();
                }
                if check{
                    println!("Une erreur est survenue durant la création du config.");
                    for e in arr_err{
                        println!("{}",e);
                    }
                }
            },
            Err(err)=>{
                panic!("Erreur à la conception du config : {}",err.to_string())
            }
        }
    }
}

fn search(path:&str)->bool{
    match fs::metadata(path){
        Ok(_x)=>{return true},
        Err(_err)=>{return false}
    }
}

fn define_os(os:&String)->String{
    match os.to_lowercase().as_str(){
        "linux"=>{return "/usr/bin".to_string();},
        "windows"=>{return String::from("C:\\MinGW\\bin\\");},
        "windows_nt"=>{return String::from("C:\\MinGW\\bin\\");},
        "darwin"=>{return "/usr/bin/g++".to_string();},
        "android"=>{return "/system/bin/g++".to_string();}
        _=>{
            panic!("Système d'exploitation non reconnu.")
        }
    }
}