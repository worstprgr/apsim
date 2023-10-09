mod constants;
use constants as Con;


fn main() {
    let border_top: String = border_top_bottom();
    let mid: String = repeat_line();
    println!("{}", border_top);
    println!("{}", mid);
    println!("{}", border_top);
}

/// Creating border on the x-axis
fn xborder() -> String {
    let x: String = Con::HYPHEN.repeat(Con::XGRIDINNER);
    return x;
}

/// Filling the row with a whitespace
fn empty_line() -> String {
    let x: String = " ".repeat((Con::XGRIDINNER*2) + 1);
    return x;
}

/// creating a single row
fn write_line() -> String {
    let mut y: String = String::new();
    y.push_str(Con::PIPE);
    y.push_str(&*empty_line());
    y.push_str(Con::PIPE);
    return y;
}

/// generating multiple rows
fn repeat_line() -> String {
    let mut lines: String = String::new();

    for _ in 0..Con::YGRIDOUTER {
        lines.push_str(&*format!("{}\n", write_line()))
    }
    let out = lines.strip_suffix("\n");
    let t = out.unwrap().to_string();
    return t;
}

/// creating the top and bottom border
fn border_top_bottom() -> String {
    let mut border_top_bot: String = String::new();
    border_top_bot.push_str(Con::LCORNER);
    border_top_bot.push_str(&*xborder());
    border_top_bot.push_str(Con::RCORNER);
    return border_top_bot;
}

// Output:
//      + - - - - - - - - - - - - - - - - - - - - - - +
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      |                                             |
//      + - - - - - - - - - - - - - - - - - - - - - - +
