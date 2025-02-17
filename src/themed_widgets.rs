use std::{borrow::Borrow, ops::RangeInclusive};

use iced::{
    border::Radius,
    widget::{
        slider::{Handle, HandleShape, Rail},
        PickList, TextInput,
    },
    Background, Border, Color, Renderer, Shadow, Theme,
};

use crate::Message;

enum Colors {
    WidgetBackground,
    WidgedBackgroundHighlight,
    ButtonColor,
    SecondaryButtonColor,
    TextColor,
    GrayTextColor,
}
impl Colors {
    fn get(&self) -> Color {
        match self {
            Colors::WidgetBackground => Color::from_rgb8(54, 58, 79),
            Colors::WidgedBackgroundHighlight => Color::from_rgb8(73, 77, 100),
            Colors::ButtonColor => Color::from_rgb8(30, 102, 245),
            Colors::SecondaryButtonColor => Color::from_rgb8(64, 160, 43),
            Colors::TextColor => Color::from_rgb8(255, 255, 255),
            Colors::GrayTextColor => Color::from_rgb8(200, 200, 200),
        }
    }
}
pub fn button<'a>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::widget::Button<'a, Message, Theme, Renderer> {
    iced::widget::button(content).style(move |_: &Theme, _| iced::widget::button::Style {
        background: Some(Background::Color(Color::from_rgb8(30, 102, 245))),
        text_color: Color::from_rgb8(255, 255, 255),
        border: Border {
            color: Color::from_rgb8(30, 102, 245),
            width: 0.,
            radius: Radius::new(10),
        },
        shadow: Shadow::default(),
    })
}

pub fn secondary_button<'a>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::widget::Button<'a, Message, Theme, Renderer> {
    iced::widget::button(content).style(move |_: &Theme, _| iced::widget::button::Style {
        background: Some(Background::Color(Colors::SecondaryButtonColor.get())),
        text_color: Color::from_rgb8(255, 255, 255),
        border: Border {
            color: Color::from_rgb8(64, 160, 43),
            width: 0.,
            radius: Radius::new(10),
        },
        shadow: Shadow::default(),
    })
}

pub fn red_button<'a>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::widget::Button<'a, Message, Theme, Renderer> {
    iced::widget::button(content).style(move |_: &Theme, _| iced::widget::button::Style {
        background: Some(Background::Color(Color::from_rgb8(230, 69, 83))),
        text_color: Color::from_rgb8(255, 255, 255),
        border: Border {
            color: Color::from_rgb8(230, 69, 83),
            width: 0.,
            radius: Radius::new(10),
        },
        shadow: Shadow::default(),
    })
}

pub fn button_with_color<'a>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
    color: Color,
) -> iced::widget::Button<'a, Message, Theme, Renderer> {
    iced::widget::button(content).style(move |_: &Theme, _| iced::widget::button::Style {
        background: Some(Background::Color(color)),
        text_color: Color::from_rgb8(255, 255, 255),
        border: Border {
            color: color,
            width: 0.,
            radius: Radius::new(10),
        },
        shadow: Shadow::default(),
    })
}

pub fn pick_list<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
{
    iced::widget::pick_list(options, selected, on_selected)
        .style(move |_: &Theme, _| iced::widget::pick_list::Style {
            text_color: Color::from_rgb8(255, 255, 255),
            placeholder_color: Color::from_rgb8(255, 255, 255),
            handle_color: Color::from_rgb8(30, 102, 245),
            background: Background::Color(Colors::WidgetBackground.get()),
            border: Border {
                color: Colors::WidgetBackground.get(),
                width: 0.,
                radius: Radius::new(10),
            },
        })
        .menu_style(|_| iced::overlay::menu::Style {
            background: Background::Color(Colors::WidgetBackground.get()),
            border: Border {
                color: Colors::WidgetBackground.get(),
                width: 0.,
                radius: Radius::new(10),
            },
            text_color: Color::from_rgb8(255, 255, 255),
            selected_text_color: Color::from_rgb8(255, 255, 255),
            selected_background: Background::Color(Colors::WidgedBackgroundHighlight.get()),
        })
}

pub fn text_input<'a, Message>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    iced::widget::text_input(placeholder, value).style(|_, _| iced::widget::text_input::Style {
        background: Background::Color(Colors::WidgetBackground.get()),
        border: Border {
            color: Colors::WidgetBackground.get(),
            width: 0.,
            radius: Radius::new(10),
        },
        icon: Colors::GrayTextColor.get(),
        placeholder: Colors::GrayTextColor.get(),
        value: Colors::TextColor.get(),
        selection: Colors::ButtonColor.get(),
    })
}

pub fn toggler<'a>(is_checked: bool) -> iced::widget::Toggler<'a, Message, Theme, Renderer> {
    iced::widget::toggler(is_checked).style(|_, _| iced::widget::toggler::Style {
        background: Colors::WidgetBackground.get(),
        background_border_width: 0.,
        background_border_color: Colors::ButtonColor.get(),
        foreground: Colors::ButtonColor.get(),
        foreground_border_width: 0.,
        foreground_border_color: Colors::ButtonColor.get(),
    })
}

pub fn slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> iced::widget::Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
    iced::widget::slider(range, value, on_change).style(|_, _| iced::widget::slider::Style {
        rail: Rail {
            backgrounds: (
                Background::Color(Colors::ButtonColor.get()),
                Background::Color(Colors::WidgetBackground.get()),
            ),
            width: 5.,
            border: Border {
                color: Colors::TextColor.get(),
                width: 0.,
                radius: Radius::new(10),
            },
        },
        handle: Handle {
            shape: HandleShape::Rectangle {
                width: 10,
                border_radius: Radius::new(10.),
            },
            background: Background::Color(Colors::ButtonColor.get()),
            border_width: 0.,
            border_color: Colors::ButtonColor.get(),
        },
    })
}
