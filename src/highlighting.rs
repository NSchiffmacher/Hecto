use termion::color;

#[derive(PartialEq)]
pub enum Type {
    Number,
    SearchMatch,
    String,
    Character,
    Comment,
    PrimaryKeyword,
    SecondaryKeyword,
    None,
}

impl Type {
    pub const fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::SearchMatch => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment => color::Rgb(133, 153, 0),
            Type::PrimaryKeyword => color::Rgb(181, 137, 0),
            Type::SecondaryKeyword => color::Rgb(42, 161, 152),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
