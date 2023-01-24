fn main() {
    println!("Hello, world!");
    let matches = cli::parse_args();
    println!("{:?}", matches);
}
