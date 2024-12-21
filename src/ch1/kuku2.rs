fn main() {
    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:5}", x*y))
            .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}
