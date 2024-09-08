use ansi_term::Colour::RGB;

use super::bundle::{Card, CardBundle};

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            bevy::render::color::Color::Rgba {
                red,
                green,
                blue,
                alpha,
            } => {
                // massage the Bevy::Color into a ansi_term::Colour
                let color = RGB(
                    (red * alpha * 255.0) as u8,
                    (green * alpha * 255.0) as u8,
                    (blue * alpha * 255.0) as u8,
                );
                write!(f, "{}", color.paint(format!("{}", self.value)))
            }
            _ => write!(f, "card with undefined color {}", self.value),
        }
    }
}

impl std::fmt::Debug for CardBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardBundle")
            .field("{:?}", &self.card)
            .finish()
    }
}
