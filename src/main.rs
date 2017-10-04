extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Acerte o número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite um número de 1 a 100:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Não consegui ler a linha.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("Você digitou {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Muito baixo!"),
            Ordering::Greater   => println!("Muito grande!"),
            Ordering::Equal     => {
                println!("Você ganhou!");
                break;
            }
        }
    }
}
