use std::io; //Librairie input/output io dans std
use rand::Rng; //Librairie random dans rand
use std::cmp::Ordering; //Librairie comparaison dans std

fn main() {
    hello_world();
    devinette();
}

fn devinette() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Le chiffre secret est : {}", secret_number);
    
    println!("Devine le chiffre que j'ai décidé ! : ");
    
    let mut guess = String::new(); 
    //let ça crée un tuple par défaut, inchangeable, sauf si on rajoute mut dessus
    //String::new ça signifie que on crée un nouveau string vide (UTF-8)
    
    io::stdin() //fonction stdin incluse dans io, avec des méthodes en dessous
        .read_line(&mut guess)
        .expect("Echec de lecture...");
    
    let guess: u32 = guess.trim().parse().expect("Tu n'as pas entré un nombre !");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Trop petit !"),
        Ordering::Greater => println!("Trop grand !"),
        Ordering::Equal => println!("Félicitations, tu as gagné !"),
    }
    
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn hello_world() {
    println!("Hello, world!\n");
    std::io::stdin().read_line(&mut String::new()).unwrap(); //pour attendre une touche pressée avant de poursuivre
}
