use iced::widget::svg::Handle;
use iced::widget::text::LineHeight;
use iced::widget::{Column, Row};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, svg, table, text, text_input, tooltip,
};

use iced::{Length, Pixels, Theme};

use crate::application::{Message, TableRow};
use crate::creator_tag::TypeTag;

const SVG_FOLDER_BYTES: &[u8] = include_bytes!("..\\asset\\folder.svg");
const SVG_RUN_BYTES: &[u8] = include_bytes!("..\\asset\\run.svg");
const SVG_CANCEL_BYTES: &[u8] = include_bytes!("..\\asset\\cancel.svg");
const SIZE: f32 = 50.0;
const HEIGHT: f32 = SIZE / 2.0;

pub struct CreatorView {
    handle_folder: Handle,
    handle_run: Handle,
    handle_cancel: Handle,
}

impl CreatorView {
    pub fn new() -> Self {
        Self {
            handle_folder: Handle::from_memory(SVG_FOLDER_BYTES),
            handle_run: Handle::from_memory(SVG_RUN_BYTES),
            handle_cancel: Handle::from_memory(SVG_CANCEL_BYTES),
        }
    }

    pub fn create_top_panel(
        &self,
        path: &String,
        tags: Vec<TypeTag>,
        tag: Option<TypeTag>,
    ) -> Row<'_, Message> {
        let folder_icon = svg(self.handle_folder.clone())
            .width(Length::Fixed(HEIGHT))
            .height(Length::Fixed(HEIGHT));
        let run_icon = svg(self.handle_run.clone())
            .width(Length::Fixed(HEIGHT))
            .height(Length::Fixed(HEIGHT));

        let left_inputs = column![
            text_input("Путь к папке", path)
                .line_height(LineHeight::Absolute(Pixels::from(HEIGHT)))
                .width(Length::Fixed(SIZE * 6.0)),
            pick_list(tags, tag, Message::SelectedTag)
                .placeholder("Выбор тега")
                .text_line_height(LineHeight::Absolute(Pixels::from(HEIGHT)))
                .width(Length::Fixed(SIZE * 6.0))
        ]
        .spacing(10);

        row![
            left_inputs,
            button(folder_icon)
                .on_press(Message::SelectedPath)
                .padding(5),
            button(run_icon)
                .on_press(Message::SetTags)
                .height(Length::Fixed(SIZE * 1.5))
                .width(Length::Fixed(SIZE * 1.5))
                .padding(15)
        ]
        .spacing(10)
        .align_y(iced::alignment::Vertical::Top)
    }

    pub fn create_main_content<'a>(
        &self,
        row: Row<'a, Message, Theme>,
        tags: Vec<TableRow>,
    ) -> Column<'a, Message> {
        column![
            row,
            table(
                vec![
                    table::column("Теги", |row: TableRow| { text(row.tag.to_string()) }),
                    table::column("Значение", |row: TableRow| {
                        self.create_value(row)
                    })
                ],
                tags
            )
        ]
        .padding(10)
        .spacing(15)
    }

    fn create_value<'a>(&self, row: TableRow) -> Row<'a, Message> {
        if [TypeTag::Track, TypeTag::Title].contains(&row.tag) {
            return row![
                checkbox(true).size(Pixels::from(HEIGHT * 1.5)),
                tooltip(
                    "?",
                    container("Формат: Num. Artist - Name.mp3 или Num. Name")
                        .padding(10)
                        .style(container::rounded_box),
                    tooltip::Position::FollowCursor
                ),
                {
                    let cancel_icon = svg(self.handle_cancel.clone())
                        .width(Length::Fixed(HEIGHT))
                        .height(Length::Fixed(HEIGHT));
                    button(cancel_icon).on_press(Message::DeleteTag(row.tag))
                }
            ]
            .spacing(5);
        }

        if row.tag == TypeTag::Artist {
            return row![
                text_input("Значение", &row.value)
                    .on_input(move |v| Message::InputedValue(row.tag, v))
                    .width(Length::Fixed(SIZE * 5.0)),
                tooltip(
                    "?",
                    container(
                        "Оставьте пустоту, если хотите чтобы программа сама нашла исполнителя"
                    )
                    .padding(10)
                    .style(container::rounded_box),
                    tooltip::Position::FollowCursor
                ),
                {
                    let cancel_icon = svg(self.handle_cancel.clone())
                        .width(Length::Fixed(HEIGHT))
                        .height(Length::Fixed(HEIGHT));
                    button(cancel_icon).on_press(Message::DeleteTag(row.tag))
                }
            ]
            .spacing(5);
        }

        if [TypeTag::Text, TypeTag::TextValues].contains(&row.tag) {
            return row![
                text_input("Значение", &row.value)
                    .on_input(move |v| Message::InputedValue(row.tag, v))
                    .width(Length::Fixed(SIZE * 5.0)),
                tooltip(
                    "?",
                    container("Формат: Id:Значение, ...,")
                        .padding(10)
                        .style(container::rounded_box),
                    tooltip::Position::FollowCursor
                ),
                {
                    let cancel_icon = svg(self.handle_cancel.clone())
                        .width(Length::Fixed(HEIGHT))
                        .height(Length::Fixed(HEIGHT));
                    button(cancel_icon).on_press(Message::DeleteTag(row.tag))
                }
            ];
        }

        // if [TypeTag::DateRecorded, TypeTag::DateReleased, TypeTag::OriginalDateReleased].contains(&row.tag) {
        //     return row![
        //         text_input("Значение", &row.value)
        //             .on_input(move |v| Message::InputedValue(row.tag, v))
        //             .width(Length::Fixed(SIZE * 5.0)),
        //         tooltip(
        //             "?",
        //             container("Формат: YYYY:MM:DD:HH:MM:SS\nИспользуйте точку для пропуска")
        //                 .padding(10)
        //                 .style(container::rounded_box),
        //             tooltip::Position::FollowCursor
        //         ),
        //         {
        //             let cancel_icon = svg(self.handle_cancel.clone())
        //                 .width(Length::Fixed(HEIGHT))
        //                 .height(Length::Fixed(HEIGHT));
        //             button(cancel_icon).on_press(Message::DeleteTag(row.tag))
        //         }
        //     ];
        // }

        row![
            text_input("Значение", &row.value)
                .on_input(move |v| Message::InputedValue(row.tag, v))
                .width(Length::Fixed(SIZE * 5.0)),
            {
                let cancel_icon = svg(self.handle_cancel.clone())
                    .width(Length::Fixed(HEIGHT))
                    .height(Length::Fixed(HEIGHT));
                button(cancel_icon).on_press(Message::DeleteTag(row.tag))
            }
        ]
    }
}
