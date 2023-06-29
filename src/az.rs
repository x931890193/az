
pub fn print_az() {
    for item in ('Z'..='a').rev().rev() {
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