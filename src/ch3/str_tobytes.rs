fn main() {
    let pr = "猫に小判";

    // 1バイトずつ表示
    for byte in pr.bytes() {
        print!("{:2x} ", byte);
    }

    // バイト数を得る
    println!("\nバイト数={}B", pr.len());
}
