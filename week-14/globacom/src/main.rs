use std::io::Read;

fn main (){
    let std::fs::File::open("staff_tb.sql"):unwrap();
    let mut contents = String::new();
    read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}