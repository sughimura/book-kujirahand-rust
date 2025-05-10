fn main() {
    let pr = "窮鼠猫を噛む";

    // 1文字ずつ表示
    for char in pr.chars() {
        print!("[{}]", char);
    }

    // 文字数を調べる
    println!("\n文字数={}字", pr.chars().count());

    // Vec<char>に変換して処理する
    let pr_chars: Vec<char> = pr.chars().collect();
    for char in pr_chars.iter() {
        print!("({})", char);
    }
    println!("\n文字数={}字", pr_chars.len());
}
