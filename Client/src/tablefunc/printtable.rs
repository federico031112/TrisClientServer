pub fn stampa_tabella(righe: usize, colonne: usize, tabella: &[[char; 3]; 3]) {
    let mut i = 0;
    println!("");
    println!("  0   1   2 ");
    while i < righe {
        let mut j = 0;
        print!("{}",i);
        while j < colonne {
            print!(" {} ",tabella[i][j]);
            if j < colonne-1 {
                print!("|")
            }
            j = j +1;
        }
        println!("");
        if i < colonne-1 {
            println!(" ---+---+---")
        }
        i = i + 1;
    }
    println!("");
}