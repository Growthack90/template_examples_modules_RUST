mod inaccessible;
pub mod nested;

pub fn funzione_home() {
    println!("funzione home");
}

pub fn funzione_login() {
    println!("funzione login");
}

fn funzione_privata() {
    println!("accesso privato");
}

pub fn accesso_indiretto() {
    print!("accesso indiretto\n");

    funzione_privata();
}
