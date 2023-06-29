
pub fn print_az() {
    for item in 'a'..='z' {
        println!("{}", item);
    }
    for item in 'A'..='Z' {
        println!("{}", item);
    }
}

pub mod sub_az {
    pub fn print_az() {
        for item in 'A'..='z' {
            println!("{}", item);
        }
    }
}