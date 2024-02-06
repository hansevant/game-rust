use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Tebak Angka!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);

    loop {
        println!("Masukkan Angka !");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tebakkanmu : {}", guess);
    
        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater => println!("terlalu besar"),
            Ordering::Equal => {
                println!("Benar");
                break;
            }
        } 
    }
}
