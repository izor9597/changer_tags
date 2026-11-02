use crate::service_tag::TagContainer;
use id3::{Tag, TagLike};
use std::path::PathBuf;
use yore::code_pages::{CP1251, CP1252};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeTag {
    Album,
    AlbumArist,
    Artist,
    // DateRecorded,
    // DateReleased,
    Disc,
    Genre,
    // OriginalDateReleased,
    Text,
    TextValues,
    Title,
    TotalDiscs,
    TotalTracks,
    Track,
    Year,
}

impl std::fmt::Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Album => "Альбом",
            Self::AlbumArist => "Исполнитель альбома",
            Self::Artist => "Исполнитель",
            // Self::DateRecorded => "Дата записи",
            // Self::DateReleased => "Дата создания",
            Self::Disc => "Диск",
            Self::Genre => "Жанр",
            // Self::OriginalDateReleased => "Изначальная дата создания",
            Self::Text => "Одиночный текст",
            Self::TextValues => "Список текстовых значений",
            Self::Title => "Название",
            Self::TotalDiscs => "Номер диска",
            Self::TotalTracks => "Количество треков",
            Self::Track => "Нумерация треков",
            Self::Year => "Год",
        })
    }
}

pub struct CreatorTag;

impl CreatorTag {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_tag(&mut self, tags: &TagContainer, path: PathBuf) -> Tag {
        let mut tag: Tag = match self.read_file_tag(path) {
            Some(tag) => tag,
            None => Tag::new(),
        };

        if let Some(album) = &tags.album {
            tag.set_album(album);
        }
        if let Some(album_artist) = &tags.album_artist {
            tag.set_album_artist(album_artist);
        }
        if let Some(artist) = &tags.artist {
            tag.set_artist(artist);
        }
        // if let Some(date_recorded) = date_recorded {
        //     tag.set_date_recorded(Timestamp::(&date_recorded.value));
        // }
        // if let Some(date_released) = date_released {
        //     let mut time_stamp = Timestamp::default();
        //     time_stamp.
        //     tag.set_date_released(&date_released.value);
        // }
        if let Some(disc) = &tags.disc {
            let disc: u32 = disc.parse().expect("Error disc");
            tag.set_disc(disc);
        }
        if let Some(genre) = &tags.genre {
            tag.set_genre(genre);
        }
        // if let Some(original_date_released) = original_date_released {
        //     tag.set_original_date_released(&original_date_released.value);
        // }
        if let Some(text) = &tags.text {
            tag.set_text(&text.0, &text.1);
        }
        if let Some(text_values) = &tags.text_values {
            tag.set_text_values(&text_values.0, text_values.1.split(","));
        }
        if let Some(title) = &tags.title {
            tag.set_title(title);
        }
        if let Some(total_discs) = &tags.total_discs {
            let total_discs: u32 = total_discs.parse().expect("Error total discs");
            tag.set_total_discs(total_discs);
        }
        if let Some(total_tracks) = &tags.total_tracks {
            let total_tracks: u32 = total_tracks.parse().expect("Error total tracks");
            tag.set_total_tracks(total_tracks);
        }
        if let Some(track) = &tags.track {
            tag.set_track(*track);
        }
        if let Some(year) = &tags.year {
            let year: i32 = year.parse().expect("Error year");
            tag.set_year(year);
        }

        tag
    }

    fn read_file_tag(&self, path: PathBuf) -> Option<id3::Tag> {
        if let Ok(readed_tag) = Tag::read_from_path(path) {
            let mut new_tag = Tag::new();

            if let Some(album) = readed_tag.album() {
                new_tag.set_album(self.decode(album));
            }
            if let Some(album_artist) = readed_tag.album_artist() {
                new_tag.set_album_artist(self.decode(album_artist));
            }
            if let Some(artist) = readed_tag.artist() {
                new_tag.set_artist(self.decode(artist));
            }
            if let Some(date_recorded) = readed_tag.date_recorded() {
                new_tag.set_date_recorded(date_recorded);
            }
            if let Some(date_realesed) = readed_tag.date_released() {
                new_tag.set_date_released(date_realesed);
            }
            if let Some(disc) = readed_tag.disc() {
                new_tag.set_disc(disc);
            }
            if let Some(duration) = readed_tag.duration() {
                new_tag.set_duration(duration);
            }
            if let Some(genre) = readed_tag.genre() {
                new_tag.set_genre(self.decode(genre));
            }
            if let Some(original_date_released) = readed_tag.original_date_released() {
                new_tag.set_original_date_released(original_date_released);
            }
            if let Some(title) = readed_tag.title() {
                new_tag.set_title(self.decode(title));
            }
            if let Some(total_discs) = readed_tag.total_discs() {
                new_tag.set_total_discs(total_discs);
            }
            if let Some(total_tracks) = readed_tag.total_tracks() {
                new_tag.set_total_tracks(total_tracks);
            }
            if let Some(track) = readed_tag.track() {
                new_tag.set_track(track);
            }
            if let Some(year) = readed_tag.year() {
                new_tag.set_year(year);
            }

            Some(new_tag)
        } else {
            None
        }
    }

    fn decode(&self, wrong_string: &str) -> String {
        match &CP1252.encode(wrong_string) {
            Ok(encode_string) => CP1251.decode(encode_string).to_string(),
            Err(_) => wrong_string.to_string(),
        }
    }
}
