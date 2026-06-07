use std::io::{BufRead, BufReader};

pub fn readplay () -> String {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    let mut bufreader = BufReader::new(stdin);
    println!("inserisci dove vuoi giocare (riga e colonna, (esempio: 1 1)): ");
    let _ = bufreader.read_line(&mut buffer);
    buffer

}