use std::io ;

fn main() {
    let mut input = "Donkey".to_string() ;

    io::stdin().read_line(&mut input) ;

    borrow_string(&input) ;
    own_string(input) ;
}

fn borrow_string(s: &String) {
    println!("{}", s) ;
}

fn own_string(s: String) {
    println!("{}", s) ;
}
