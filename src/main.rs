#![windows_subsystem = "windows"]

use application::*;

mod application;
mod creator_tag;
mod creator_view;
mod service_file;
mod service_tag;

const WINDOW_SIZE: (f32, f32) = (500.0, 500.0);

fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view)
        .title("ChangerTags")
        .window_size(WINDOW_SIZE)
        .theme(Application::theme)
        .run()
}
