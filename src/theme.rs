use iced::Color;

// Paleta de cores Catppuccin
pub enum Colors {
    WidgetBackground,
    WidgedBackgroundHighlight,
    ButtonColor,
    SecondaryButtonColor,
    TextColor,
    GrayTextColor,
    Peach,
    Red,
    Green,
    Blue,
    Mauve
}
impl Colors {
    pub fn get(&self) -> Color {
        match self {
            Colors::WidgetBackground => Color::from_rgb8(54, 58, 79),
            Colors::WidgedBackgroundHighlight => Color::from_rgb8(73, 77, 100),
            Colors::ButtonColor => Color::from_rgb8(30, 102, 245),
            Colors::SecondaryButtonColor => Color::from_rgb8(64, 160, 43),
            Colors::TextColor => Color::from_rgb8(255, 255, 255),
            Colors::GrayTextColor => Color::from_rgb8(200, 200, 200),
            Colors::Peach => Color::from_rgb8(245, 169, 127),
            Colors::Red => Color::from_rgb8(237, 135, 150),
            Colors::Green => Color::from_rgb8(166, 218, 149),
            Colors::Blue => Color::from_rgb8(138, 173, 244),
            Colors::Mauve => Color::from_rgb8(198, 160, 246),
        }
    }
}