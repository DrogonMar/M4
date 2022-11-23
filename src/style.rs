use iced::{Background, Color, text_input, Vector};
use iced::button::{Style, StyleSheet};
use crate::constants;

pub struct M4ButtonStyleSheet;

impl StyleSheet for M4ButtonStyleSheet {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Vector::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: *constants::M4BLUE,
        }
    }
}

pub struct M4InputStyleSheet {
    pub show_validity: bool,
    pub is_valid: bool,
}

// A sleek and simple stylesheet for the text input,
// a slight grey border and a semi-transparent background
// Border color is M4Blue but toned down a tad.
impl text_input::StyleSheet for M4InputStyleSheet {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            border_radius: 5.0,
            border_width: 1.3,
            //Make a darker gray than the background
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgb(1., 1., 1.)),
            border_radius: 5.0,
            border_width: 1.3,
            //Make a darker gray than the background
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.6, 0.6, 0.6)
    }

    fn value_color(&self) -> Color {
        return if self.show_validity {
            if self.is_valid {
                Color::from_rgb(0., 0.6, 0.)
            } else {
                Color::from_rgb(0.8, 0., 0.)
            }
        } else {
            Color::BLACK
        };
    }

    fn selection_color(&self) -> Color {
        Color::from_rgba(0., 0., 1.0, 0.55)
    }
}
