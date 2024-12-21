fn main() {
    let moon = 384400.0; // km
    let car = 80.0; // km/h
    let btrain = 300.0; // km/h
    println!("車で月まで{}日", moon/car/24.0);
    println!("月まで新幹線で{}日", moon/btrain/24.0);
}
