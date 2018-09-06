pub mod colours {

    // enum Colour{
    //     Black = 30,
    //     Red,
    //     Green,
    //     Yellow,
    //     Blue,
    //     Magenta,
    //     Cyan,
    //     White,
    //     Bright_Black = 90,
    //     Bright_Red,
    //     Bright_Green,
    //     Bright_Yellow,
    //     Bright_Blue,
    //     Bright_Magenta,
    //     Bright_Cyan,
    //     Bright_White
    // }

    pub fn colourify(data: &Vec<Vec<f32>>, inner_len: usize) -> String {
        let mut out = String::from("");

        for y_line in data {
            for index in 0..inner_len {
                if y_line[index] > 1.0 {
                    panic!("Error percent was above 1.0");
                }

                out.push('\u{001b}');
                out.push('[');

                let i = if y_line[index] > 0.5 {
                    90 + (((y_line[index] as f32 - 0.5) * 350.0) as usize)
                } else {
                    30 + (((y_line[index] as f32) * 350.0) as usize)
                };

                //let i = if y_line[index]

                out.push_str(&i.to_string());
                out.push('m');
                out.push_str("\u{2593}");
            }
            out.push('\n');
        }

        out.push_str("\u{2593}[0m");

        out
    }

    pub fn greyify(data: &Vec<Vec<f32>>, inner_len: usize) -> String {
        let mut out = String::from("");

        for y_line in data {
            for index in 0..inner_len {
                if y_line[index] > 1.0 {
                    panic!("Error percent was above 1.0");
                }

                let percent = y_line[index];

                if percent <= 0.16666 {
                    out.push_str("\u{001b}[38;5;240m\u{2591}");
                    out.push_str("\u{001b}[38;5;240m\u{2591}");
                } else if percent <= 0.33333 {
                    out.push_str("\u{001b}[38;5;240m\u{2592}");
                    out.push_str("\u{001b}[38;5;240m\u{2592}");
                } else if percent <= 0.5 {
                    out.push_str("\u{001b}[38;5;240m\u{2593}");
                    out.push_str("\u{001b}[38;5;240m\u{2593}");
                } else if percent <= 0.66666 {
                    out.push_str("\u{001b}[38;5;248m\u{2591}");
                    out.push_str("\u{001b}[38;5;248m\u{2591}");
                } else if percent <= 0.83333 {
                    out.push_str("\u{001b}[38;5;248m\u{2592}");
                    out.push_str("\u{001b}[38;5;248m\u{2592}");
                } else {
                    out.push_str("\u{001b}[38;5;248m\u{2593}");
                    out.push_str("\u{001b}[38;5;248m\u{2593}");
                }
            }
            out.push('\n');
        }

        out
    }
}
