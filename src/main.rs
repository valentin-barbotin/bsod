use crossterm::style::Color;

fn main() {
    let (width, height) = crossterm::terminal::size().unwrap();
    let mut buffer = String::new();
    buffer.push(' ');
    buffer = buffer.repeat(width as usize);
    buffer = buffer.repeat(height as usize);

    let text = "Your device ran into a problem and needs to restart. We're just collecting some error info, and then we'll restart for you";
    let x = width / 2 - text.len() as u16 / 2;
    let y = height / 2;
    println!("{} {}", x, y);
    buffer.replace_range(
        (y as usize * width as usize + x as usize)..(y as usize * width as usize + x as usize + text.len()),
        text
    );

    let color = "#0827F5";
    let color = color.trim_start_matches('#');
    let color = u32::from_str_radix(color, 16).unwrap();
    let color = Color::Rgb {
        r: (color >> 16) as u8,
        g: (color >> 8) as u8,
        b: color as u8,
    };

    crossterm::execute!(
        std::io::stdout(),
        crossterm::style::SetBackgroundColor(color),
        // crossterm::style::SetBackgroundColor(crossterm::style::Color::DarkBlue),
        crossterm::style::Print(buffer),
    ).unwrap();

    // block the ui
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
