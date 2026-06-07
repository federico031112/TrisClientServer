use std::net::TcpStream;
use std::io::{Read,Write};
use crate::streamrw::streamread::read_from_stream;
use crate::streamrw::streamwrite::readplay;
mod streamrw;
use crate::tablefunc::printtable::stampa_tabella;
use crate::tablefunc::validatetable::valida_tabella;
//use crate::tablefunc::validatetable::valida_tabella;
mod tablefunc;
use colored::{self, Colorize};
fn main() -> std::io::Result<()>{
    println!("{}","Inizio gioco tris!".blue());
    println!("");
    println!("{}","Connessione al server...");
    let mut stream = TcpStream::connect("127.0.0.1:4321")?;
    println!("");
    println!("{}","Connessione eseguita correttamente!".green());
    println!("");
    let mut tabella: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut bufferturno = [0u8; 1024];
    let giocatore: Result<&str, std::str::Utf8Error>;

    //attesa del secondo giocatore
    println!("in attesa del secondo giocatore...");
    println!("");
    std::io::stdout().flush()?;
    let numbyte: usize;

    numbyte = stream.read(&mut bufferturno)?;
    if numbyte == 0 {
        println!("Disconnessione...");
        std::io::stdout().flush()?;
    }
    giocatore = std::str::from_utf8(&bufferturno[..numbyte]);
    match giocatore {
        Ok("1") => {
            println!("{}","Secondo giocatore connesso!".green());
            println!("");
            'game: loop{
                println!("è il tuo turno");
                stampa_tabella(3, 3, &tabella);
                //managing the input of the player
                'input: loop{
                    let mossa2 = readplay();
                    let msg = mossa2.trim();
                    let mut chars = msg.chars();
                    let c1 = chars.next();
                    let c2 = chars.next();
                    let n1: Option<u32>;
                    let n2: Option<u32>;
                    match (c1, c2) {
                        (Some(c1), Some(c2)) => {
                            n1 = c1.to_digit(10);
                            n2 = c2.to_digit(10);
                        }

                        _ => {
                            println!("errore nella conversione");
                            continue 'input;
                        }
                    }


                    match (n1,n2) {
                        (Some(riga), Some(colonna)) => {
                            let riga = riga as usize;
                            let colonna = colonna as usize;

                                //check if the values are valid for the table
                            if riga < 3 && colonna < 3 {
                                if tabella[riga][colonna] == ' '{
                                    tabella[riga][colonna] = 'O';
                                    stampa_tabella(3, 3, &tabella);
                                    stream.write_all(msg.as_bytes())?;
                                    break 'input;
                                }else{
                                    println!("tabella già occupata");
                                    continue 'input;
                                }
                            }else{
                                println!("valore inserito non valido");
                                continue 'input;
                            }
                        }
                        _ => {
                            println!("errore");
                            continue 'input;
                        }
                    }

                }








                if valida_tabella(&tabella) {
                    println!("HAI VINTO!");
                    break 'game;
                }





                'server_response: loop {

                    let mossa1;
                    //read the message from the server
                    println!("{}","in attesa della risposta dell'altro giocatore".red());
                    mossa1 = read_from_stream(&mut stream);
                    //check the messagge from the server
                    match mossa1 {
                        Some(rsp) => {
                            if rsp.eq("Exit") || rsp.eq("EXIT") || rsp.eq("exit"){
                                println!("il secondo giocatore ha chiuso la connesione");
                                println!("HAI VINTO");
                                break 'game;
                            }
                            //create a set of chars
                            let mut chars = rsp.chars();

                            //take the first 2 chars
                            let char1 = chars.next();
                            let char2 = chars.next();

                            //convert the chars in u32
                            let n1 = char1.and_then(|c| c.to_digit(10));
                            let n2 = char2.and_then(|c| c.to_digit(10));

                            //check the u32 values
                            match (n1,n2) {
                                (Some(riga), Some(colonna)) => {
                                    let riga = riga as usize;
                                    let colonna = colonna as usize;

                                    //check if the values are valid for the table
                                    if riga < 3 && colonna < 3 {
                                        if tabella[riga][colonna] == ' '{
                                            tabella[riga][colonna] = 'X';
                                            stampa_tabella(3, 3, &tabella);
                                            break 'server_response;
                                        }else{
                                            println!("tabella già occupata");
                                            continue 'server_response;
                                        }
                                    }else{
                                        println!("valore inserito non valido");
                                        continue 'server_response;
                                    }
                                }
                                _ => {
                                    println!("errore");
                                    continue 'server_response; 
                                }
                            }
                        }
                        None => {
                            println!("errore nella ricezione");
                            continue 'server_response;
                        }
                        
                    }

                }
 










                if valida_tabella(&tabella) {
                    println!("HAI PERSO!");
                    break;
                }















            }
        }
        Ok("2") => {
            println!("Secondo giocatore connesso!");
            println!("Attendi il tuo turno...");
            stampa_tabella(3, 3, &tabella);
            let mut flag: bool;
            let mut chiusura: bool = false;
            loop{
                let mossa1;
                //read the message from the server
                mossa1 = read_from_stream(&mut stream);

                //check the messagge from the server
                match mossa1 {
                    Some(rsp) => {
                        if rsp.eq("Exit") || rsp.eq("EXIT") || rsp.eq("exit"){
                            flag = true;
                            chiusura = true;
                            break;
                        }
                        //create a set of chars
                        let mut chars = rsp.chars();

                        //take the first 2 chars
                        let char1 = chars.next();
                        let char2 = chars.next();

                        //convert the chars in u32
                        let n1 = char1.and_then(|c| c.to_digit(10));
                        let n2 = char2.and_then(|c| c.to_digit(10));

                        //check the u32 values
                        match (n1,n2) {
                            (Some(riga), Some(colonna)) => {
                                let riga = riga as usize;
                                let colonna = colonna as usize;

                                //check if the values are valid for the table
                                if riga < 3 && colonna < 3 {
                                    if tabella[riga][colonna] == ' '{
                                        tabella[riga][colonna] = 'O';
                                        stampa_tabella(3, 3, &tabella);
                                        flag = true;
                                    }else{
                                        println!("tabella già occupata");
                                        flag = false;
                                    }
                                }else{
                                    println!("valore inserito non valido");
                                    flag = false;
                                }
                            }
                            _ => {
                                println!("errore");
                                flag = false;
                            }
                        }
                    }
                    None => {
                        println!("errore nella ricezione");
                        flag = false;
                    }
                    
                }
                if !flag {
                    continue;
                }
                if chiusura {
                    println!("il secondo giocatore ha chiuso la connessione...");
                    println!("HAI VINTO");
                    break;
                }










                if valida_tabella(&tabella) {
                    println!("HAI PERSO!");
                    break;
                }







                
                //managing the input of the player
                loop{
                    let mossa2 = readplay();
                    let msg = mossa2.trim();
                    let mut chars = msg.chars();
                    let c1 = chars.next();
                    let c2 = chars.next();
                    let mut n1: Option<u32> = None;
                    let mut n2: Option<u32> = None;
                    match (c1, c2) {
                        (Some(c1), Some(c2)) => {
                            n1 = c1.to_digit(10);
                            n2 = c2.to_digit(10);
                        }

                        _ => {
                            println!("errore nella conversione");
                            flag = false;
                        }
                    }

                    if !flag {
                        continue;
                    }

                    match (n1,n2) {
                        (Some(riga), Some(colonna)) => {
                            let riga = riga as usize;
                            let colonna = colonna as usize;

                                //check if the values are valid for the table
                            if riga < 3 && colonna < 3 {
                                if tabella[riga][colonna] == ' '{
                                    tabella[riga][colonna] = 'X';
                                    stampa_tabella(3, 3, &tabella);
                                    flag = true;
                                }else{
                                    println!("tabella già occupata");
                                    flag = false;
                                }
                            }else{
                                println!("valore inserito non valido");
                                flag = false;
                            }
                        }
                        _ => {
                            println!("errore");
                            flag = false;
                        }
                    }

                    if !flag {
                        continue;
                    }

                    if flag {
                        stream.write_all(msg.as_bytes())?;
                        break;
                    }
                }














                if valida_tabella(&tabella) {
                    println!("HAI VINTO!");
                    break ;
                }













            }
        }
        Ok(_) => {
            println!("Valore ricevuto non valido");
        }
        Err(e) => {
            println!("Errore nella ricezione dei dati {}",e)
        }
    }

    Ok(())
}
    



