mod models;

pub mod universal {
    tonic::include_proto!("universal");
}

fn main() {
    print_ln(String::from("hello, world!"));
}

fn print_ln<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}
