use iced::{
    Background,
    Color,
    button
};

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from([0.7, 0.7, 0.7]))),
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            ..button::Style::default()
        }
    }
}
