use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    SearchMatch,
}

impl Type {
    pub const fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::SearchMatch => color::Rgb(38, 139, 210),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}