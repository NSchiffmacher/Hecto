use termion::color;

#[derive(PartialEq)]
pub enum Type {
    Number,
    SearchMatch,
    String,
    None,
}

impl Type {
    pub const fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::SearchMatch => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
