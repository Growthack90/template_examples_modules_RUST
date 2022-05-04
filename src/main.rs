mod home;

fn funzione_principale() {
    println!("funzione principale");
}

fn main() {
    
    funzione_principale();
    
    home::funzione_home();

    home::funzione_login();

    home::accesso_indiretto();

    home::nested::funzione_annidata();
/* per runnare questa funzione, mettere "pub" a riga 6 nel file "nested.rs"*/ 
    // home::nested::private_function();

/* per runnare questa funzione, mettere "pub" a riga 1 nel file "mod.rs"*/ 
    // home::inaccessible::public_function();

}


