use std::io::{Read, Write};
use std::net::TcpStream;
use std::{net::TcpListener};
use std::thread;
use std::sync::{Arc , Mutex} ; 
use std::collections::HashMap; 


fn handle_client(mut stream:TcpStream ,store:Arc<Mutex<HashMap<String,String>>>){
    let mut buffer = [0;1024]; 
    let mut pending = String::new(); 
    loop {
        let n = match  stream.read(&mut buffer){
            Ok(0) => break , 
            Ok(n) => n , 
            Err(_) => break , 
        }; 
        let chunk = String::from_utf8_lossy(&buffer[..n]); 
        pending.push_str(&chunk); 

        while let Some(pos) = pending.find('\n'){
            let line = pending[..pos].trim().to_string(); 
            pending = pending[pos+1..].to_string(); 

            if line.is_empty() {
                continue ; 
            }

            let parts:Vec<&str> = line.split_whitespace().collect(); 
            if parts.is_empty(){
                continue;
            }
            let  response ; 
            if parts[0] == "GET" {
                if parts.len() != 2 {
                    response = "usage  GET <key>\n".to_string(); 
                }else {
                    let key = parts[1]; 
                    let store_guard = store.lock().unwrap(); 
                    let val  = store_guard.get(key); 
                    match val {
                        Some(v) => {
                            response = format!("{}\n",v); 
                        }, 
                        None => {
                            response = "not found\n".to_string(); 
                        }
                    }
                }
            }
            else if parts[0] == "SET"{
                if parts.len() != 3 {
                    response = "usage SET <key> <value > \n".to_string(); 
                }else {
                    let key = parts[1]; 
                    let value = parts[2]; 
                    let mut store_guard = store.lock().unwrap() ;
                    store_guard.insert(key.to_string() , value.to_string()); 
                    response = "OK \n".to_string(); 

                }
            }else if parts[0] == "DEL"{
                if parts.len() != 2 {
                    response = "usage DEL <key> ".to_string(); 
                }else {
                    let key = parts[1]; 
                    let mut store_guard = store.lock().unwrap(); 
                    let val = store_guard.remove(key); 
                    match val {
                        Some(v) => {
                            response = format!("deleted {}" , v); 
                        }, 
                        None => {
                            response = "not found such value ".to_string(); 
                        }
                    }
                    }
             } else {
                response = "invalid commmand".to_string(); 
            }

            stream.write_all(response.as_bytes()).unwrap();
          
        }

    }
}





fn main (){
    let listner = TcpListener::bind("127.0.0.1:7978").unwrap();
    println!("concurrent server listening on port localhost:7978"); 
    let store =  Arc::new(Mutex::new(HashMap::<String,String>::new())) ;
    loop {
        let (stream , addr )  = listner.accept().unwrap();
        println!("new client connected :  {}", addr); 

       let store = Arc::clone(&store); 
        thread::spawn(move || {
            println!("Handling client {}", addr);
            handle_client(stream,store);
        });
        
    }
}