use super::GridNode;
use crate::sim::color::ColorData;
use std::collections::BTreeMap;

fn f64_to_u8(f: f64) -> u8 {
    (f * u8::MAX as f64) as u8
}

pub fn clear_terminal() {
    print!("\x1b[2J\x1b[?25l\x1b[=19h");
}

pub fn reset_terminal() {
    print!("\x1b[?25h");
}

pub fn paint_terminal(data: &BTreeMap<GridNode, ColorData>, max_x: i16, max_y: i16) {
    let mut result: String = "\x1b[1;1H".into();
    let last_x = &mut 0;
    let last_y = &mut 0;

    let fill_y = |y: &mut i16, r: &mut String| {
        let mut y_delta = (max_y + 1 - *y) * 3;
        *y = 0;
        if y_delta < 0 {
            y_delta = 0;
        }
        *r += format!("{: >1$}\n", "", y_delta as usize).as_str();
    };

    for (node, color) in data {
        let x_delta = node.0 - *last_x;

        if x_delta > 0 {
            for _ in 0..x_delta {
                fill_y(last_y, &mut result);
            }
            *last_x = node.0;
            *last_y = 0;
        }
        let y_delta = (node.1 - *last_y) * 3;
        *last_y = node.1 + 1;

        let node_disp = format!(
            "\x1b[38;2;{};{};{}m{}\x1B[0m",
            &f64_to_u8(color.0),
            &f64_to_u8(color.1),
            &f64_to_u8(color.2),
            "x▓x"
        );

        result += format!("{: >1$}{node_disp}", "", y_delta as usize).as_str();
    }
    for _ in *last_x..max_x + 1 {
        fill_y(last_y, &mut result);
    }
    println!("{result}");
}
