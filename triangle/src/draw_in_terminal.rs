pub fn draw_in_terminal() {
    let height = 5;

    for i in 1..height {
        let spaces = " ".repeat(height - i);
        let stars = "*".repeat(2 * i - 1);
        println!("{}{}", spaces, stars);
    }
}