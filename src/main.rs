use chrono;

mod digits;

fn main() {
    print!("\x1b[2J");
    print!("\x1b[?25l");
    loop {
        let time = chrono::Local::now().format("%T%.3f").to_string();
        for row in &digits::DIGITS {
            for c in time.chars() {
                let idx = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    '.' => 11,
                    _ => 100,
                };
                print!(
                    "{} ",
                    if idx < row.len() {
                        row[idx].to_string()
                    } else {
                        " ".repeat(row[0].len())
                    }
                );
            }
            println!();
        }
        print!("\x1b[7A");
    }
}
