#![windows_subsystem = "windows"]

use iced::application;
use iced::window::Settings;

mod date_manager;
mod view;

use date_manager::DateManager;

fn main() -> iced::Result {
    let settings = match iced::window::icon::from_file("assets/icon.ico") {
        Ok(icon) => Settings {
            icon: Some(icon),
            ..Settings::default()
        },
        Err(_) => 
            Settings::default()
    };
    application("Calendar", update, view::view)
    .window(settings)
    .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    NextMonth,
    PreviousMonth,
}

fn update(state: &mut DateManager, message: Message) {
    match message {
        Message::NextMonth => {
            state.next_month();
        }
        Message::PreviousMonth => {
            state.previous_month();
        }
    }
}