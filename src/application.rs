use crate::creator_tag::{TypeTag};
use crate::creator_view::CreatorView;
use crate::service_file::{ServiceFile};
use crate::service_tag::ServiceTag;
use iced::widget::{container, space, stack, text};
use iced::{Element, Length, Theme};

#[derive(Clone, Debug)]
pub enum Message {
    InputedValue(TypeTag, String),
    SelectedTag(TypeTag),
    SelectedPath,
    SetTags,
    DeleteTag(TypeTag),
}

enum TagResult {
    None,
    Error(String),
    Ok,
}

#[derive(Clone)]
pub struct TableRow {
    pub tag: TypeTag,
    pub value: String,
}

pub struct Application {
    tag: Option<TypeTag>,
    path: String,
    result_programm: TagResult,
    creator_view: CreatorView,
    service_tag: ServiceTag,
}

impl Application {
    pub fn new() -> Self {
        Self {
            tag: None,
            path: String::new(),
            result_programm: TagResult::None,
            creator_view: CreatorView::new(),
            service_tag: ServiceTag::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputedValue(tag, text) => {
                for row in &mut self.service_tag.rows {
                    if tag == row.tag {
                        row.value = text;
                        break;
                    }
                }
            }
            Message::SelectedTag(tag) => {
                if let Err(error) = self.service_tag.add_tag(tag) {
                    self.result_programm = TagResult::Error(error);
                }
            }
            Message::SelectedPath => {
                let folder = rfd::FileDialog::new().set_directory("/").pick_folder();
                if let Some(folder) = folder {
                    self.path = String::from(folder.to_str().unwrap());
                }
            }
            Message::SetTags => {
                let service_file: ServiceFile;

                if let Ok(service) = ServiceFile::new(&self.path) {
                    service_file = service;
                } else {
                    self.result_programm = TagResult::Error("Некорректный путь".to_string());
                    return;
                }

                let mut tag_container = self.service_tag.prepare_tag();
                let tags: Vec<TypeTag> = self.service_tag.rows.iter().map(|row| row.tag).collect();

                for (file, file_name) in service_file {
                    if tags.contains(&TypeTag::Track) {
                        match ServiceFile::get_number(&file_name) {
                            Some(number) => tag_container.track = Some(number),
                            None => {
                                self.result_programm = TagResult::Error(
                                    "Имя файла не соответствует шаблону".to_string(),
                                )
                            }
                        }
                    }

                    if tags.contains(&TypeTag::Artist) {
                        match ServiceFile::get_artist(&file_name) {
                            Some(artist) => tag_container.artist = Some(artist),
                            None => {
                                self.result_programm = TagResult::Error(
                                    "Имя файла не соответствует шаблону".to_string(),
                                )
                            }
                        }
                    }

                    if tags.contains(&TypeTag::Title) {
                        match ServiceFile::get_title(&file_name) {
                            Some(title) => tag_container.title = Some(title),
                            None => {
                                self.result_programm = TagResult::Error(
                                    "Имя файла не соответствует шаблону".to_string(),
                                )
                            }
                        }
                    }

                    if tags.contains(&TypeTag::TextValues) {
                        if let Some(text_values) = &tag_container.text_values {
                            if let Some(index) = text_values.0.find(":") {
                                tag_container.text_values = Some((String::from(&text_values.0[0..index-1]), 
                                                                String::from(&text_values.1[index+1..])));
                            }
                            else {
                                self.result_programm = TagResult::Error("Неправильный формат TextValues".to_string());
                            }
                        }
                    }

                    if tags.contains(&TypeTag::Text) {
                        if let Some(text_values) = &tag_container.text_values {
                            if let Some(index) = text_values.0.find(":") {
                                tag_container.text_values = Some((String::from(&text_values.0[0..index-1]), 
                                                                String::from(&text_values.1[index+1..])));
                            }
                            else {
                                self.result_programm = TagResult::Error("Неправильный формат TextValues".to_string());
                            }
                        }
                    }

                    match self.service_tag.set_tag(&tag_container, file.path()) {
                        Result::Ok(_) => self.result_programm = TagResult::Ok,
                        Result::Err(error) => {
                            self.result_programm = TagResult::Error(error.to_string())
                        }
                    }
                }
            }
            Message::DeleteTag(tag) => {
                self.service_tag.delete_tag(tag);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tags = vec![
            TypeTag::Album,
            TypeTag::AlbumArist,
            TypeTag::Artist,
            // TypeTag::DateRecorded,
            // TypeTag::DateReleased,
            TypeTag::Disc,
            TypeTag::Genre,
            // TypeTag::OriginalDateReleased,
            TypeTag::Text,
            TypeTag::TextValues,
            TypeTag::Title,
            TypeTag::TotalDiscs,
            TypeTag::TotalTracks,
            TypeTag::Track,
            TypeTag::Year,
        ];

        let top_panel = self
            .creator_view
            .create_top_panel(&self.path, tags, self.tag);

        let main_content = self
            .creator_view
            .create_main_content(top_panel, self.service_tag.rows.clone());

        let result_programm = match &self.result_programm {
            TagResult::None => container(space()),
            TagResult::Ok => container(text("Выполнено"))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Bottom)
                .align_x(iced::alignment::Horizontal::Center)
                .padding(20),
            TagResult::Error(err) => container(text(err.to_string()))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(iced::alignment::Vertical::Bottom)
                .align_x(iced::alignment::Horizontal::Center)
                .padding(20),
        };

        stack![main_content, result_programm]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Oxocarbon
    }
}
