fn main() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:5},", x*y);
        }
        println!("");
    }
}
