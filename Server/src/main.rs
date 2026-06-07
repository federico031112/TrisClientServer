use core::time;
use::std::net::{TcpListener, TcpStream};
use std::{io::{self, Read, Write}, thread};
use std::sync::{Arc,Mutex};
use std::time::Duration;
use colored::{self, Colorize};
struct Shared {
    message: String,
    second_player: bool,
    ricevuto: bool
}

fn main() -> std::io::Result<()>{
    let list = TcpListener::bind("127.0.0.1:4321")?;
    let mut n_client: u8 = 0;
    let mut n_thread : u8;
    let  data = Arc::new(Mutex::new(Shared{message: "".to_string(),second_player: false, ricevuto: false}));
    for stream in list.incoming() {
        match stream{
            Ok(stream) => {  
                if n_client == 2 {
                    drop(stream);
                    continue;
                }
                n_client = n_client + 1;
                n_thread = n_client.clone();
                let  data_thread = Arc::clone(&data);
                thread::spawn(move || {
                    match handle_connection(stream, n_thread, &data_thread) {
                        Ok(()) => {

                        }

                        Err(_) => {
                            println!("errore nel thread")
                        }

                    };
                });
            }

            Err(_) => {
                println!("errore di connesione");
            }
                
        }
    }
    Ok(())
}


fn handle_connection(mut stream :TcpStream, id: u8, data: &Arc<Mutex<Shared>>) -> io::Result<()>{
    let mut buffer = [0u8; 1024];
    'game: loop{
        if id == 1 {


            'second_player_waiting: loop{
                let lock = data.lock().unwrap();
                if lock.second_player {
                    println!("{}","partita iniziata".bright_blue());
                    break 'second_player_waiting;
                }
                drop(lock);
                thread::sleep(Duration::from_millis(10));
            }



            //sending the id to the client
            stream.write_all(id.to_string().as_bytes())?;
            stream.flush()?;

            loop {
                println!("{}","In attesa mossa client 1".green());
            //reciving the response from the client
                'getting_message: loop{

                    //getting the play from the client
                    let n = stream.read(&mut buffer);
                    let  msg;     

                    //checking the number of byte recived 
                    match n {
                        Ok(0) => {

                            //handle a client disconnection


                            println!("{}","client 1 disconnesso".green());
                            break 'game;



                        }
                        Ok(n) => {
                            msg = std::str::from_utf8(&buffer[..n]);
                        }
                        Err(e) => {
                            println!("errore {} nella ricezione dei dati",e);
                            continue 'getting_message;
                        }
                    }


                    //checking the string recived
                    let c1;
                    let c2;
                    let n1;
                    let n2;
                    let mossa;
                    match msg {
                        Ok(mut msg) => {
                            msg = msg.trim();
                            mossa = msg;
                            let mut chars = msg.chars();
                            c1 = chars.next();
                            c2 = chars.next();
                            match (c1, c2) {
                                (Some(c1), Some(c2)) => {
                                    n1 = c1.to_digit(10);
                                    n2 = c2.to_digit(10);
                                }
                                _ => {
                                    println!("Nessun carattere ottenuto");
                                    continue 'getting_message;
                                }
                            }
                        }
                        Err(e) => {
                            println!("errore {} nella conversione dei dati ricevuti",e);
                            continue 'getting_message;
                        }
                    }

                    //checking the value of the play
                    match (n1, n2) {
                        (Some(n1), Some(n2)) => {
                            if n1 < 3 && n2 < 3 {
                                println!("{}","risposta del client 1 valida".green());
                            }else{
                                println!("valori ricevuti dal client con id {} non validi", id);
                                continue 'getting_message;
                            }
                        }

                        _ => {
                            println!("valori ricevuti dal client id: {}, non validi",id);
                            continue 'getting_message;
                        }
                    }

                    //sending the play to the other client
                    let mut lock = data.lock().unwrap();
                    lock.message = mossa.to_string();
                    lock.ricevuto = true;
                    drop(lock);

                    break 'getting_message;
                }


                thread::sleep(time::Duration::from_millis(100));
                let mossa1;
                    'waiting_play: loop {
                        let mut lock = data.lock().unwrap();
                        if lock.ricevuto == true {
                            mossa1 = lock.message.clone();
                            lock.ricevuto = false;
                            drop(lock);
                            break 'waiting_play;
                        }
                        drop(lock);
                        thread::sleep(time::Duration::from_millis(10));
                    }
                stream.write_all(mossa1.as_bytes())?;

            }

        }else {

            //setting the presence of the second player
            let mut lock = data.lock().unwrap();
            lock.second_player = true;
            drop(lock);

            //sending the id of the player
            stream.write_all(id.to_string().as_bytes())?;
            stream.flush()?;

            loop {
                let mossa1;

                'waiting_play: loop {
                    let mut lock = data.lock().unwrap();
                    if lock.ricevuto {
                        mossa1 = lock.message.clone();
                        lock.ricevuto = false;
                        println!("{}","client 2 invia mossa".red());
                        stream.write_all(mossa1.as_bytes())?;
                        stream.flush()?;
                        drop(lock);
                        break 'waiting_play;
                    }
                    drop(lock);
                    thread::sleep(time::Duration::from_millis(10));
                }

                println!("{}","In attesa mossa client 2".red());
                'getting_message: loop{

                //getting the play from the client
                    let n = stream.read(&mut buffer);
                    let  msg;     

                    //checking the number of byte recived 
                    match n {
                        Ok(0) => {

                            //handle a client disconnection


                            println!("{}","client 2 disconnesso".red());
                            break 'game;



                        }
                        Ok(n) => {
                            msg = std::str::from_utf8(&buffer[..n]);
                        }
                        Err(e) => {
                            println!("errore {} nella ricezione dei dati",e);
                            continue 'getting_message;
                        }
                    }


                    //checking the string recived
                    let c1;
                    let c2;
                    let n1;
                    let n2;
                    let mossa;
                    match msg {
                        Ok(mut msg) => {
                            msg = msg.trim();
                            mossa = msg;
                            let mut chars = msg.chars();
                            c1 = chars.next();
                            c2 = chars.next();
                            match (c1, c2) {
                                (Some(c1), Some(c2)) => {
                                    n1 = c1.to_digit(10);
                                    n2 = c2.to_digit(10);
                                }
                                _ => {
                                    println!("Nessun carattere ottenuto");
                                    continue 'getting_message;
                                }
                            }
                        }
                        Err(e) => {
                            println!("errore {} nella conversione dei dati ricevuti",e);
                            continue 'getting_message;
                        }
                    }

                    //checking the value of the play
                    match (n1, n2) {
                        (Some(n1), Some(n2)) => {
                            if n1 < 3 && n2 < 3 {
                                println!("{}","risposta del client con id {id} valida".red());
                            }else{
                                println!("valori ricevuti dal client con id {} non validi", id);
                                continue 'getting_message;
                            }
                        }

                        _ => {
                            println!("valori ricevuti dal client id: {}, non validi",id);
                            continue 'getting_message;
                        }
                    }

                    let mut lock = data.lock().unwrap();
                    lock.message = mossa.to_string();
                    lock.ricevuto = true;
                    drop(lock);
                    break 'getting_message;
                }
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }

    Ok(())
    
}
