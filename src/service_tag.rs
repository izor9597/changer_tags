use std::{path::PathBuf};

use crate::{
    application::TableRow,
    creator_tag::{CreatorTag, TypeTag},
};

#[derive(Default)]
pub struct TagContainer {
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    // pub date_recorded: Option<Timestamp>,
    // pub date_released: Option<Timestamp>,
    pub disc: Option<String>,
    pub genre: Option<String>,
    // pub original_date_released: Option<Timestamp>,
    pub text: Option<(String, String)>,
    pub text_values: Option<(String, String)>,
    pub title: Option<String>,
    pub total_discs: Option<String>,
    pub total_tracks: Option<String>,
    pub track: Option<u32>,
    pub year: Option<String>,
}

pub struct ServiceTag {
    pub rows: Vec<TableRow>,
    creator_tag: CreatorTag,
}

impl ServiceTag {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            creator_tag: CreatorTag::new(),
        }
    }

    pub fn add_tag(&mut self, tag: TypeTag) -> Result<(), String> {
        for row in &self.rows {
            if row.tag == tag {
                return Err(format!("Тег {tag} уже выбран"));
            }
        }
        self.rows.push(TableRow {
            tag,
            value: String::new(),
        });

        Ok(())
    }

    pub fn delete_tag(&mut self, tag: TypeTag) {
        for counter in 0..self.rows.len() {
            if let Some(_rows) = self.rows.get(counter) {
                if _rows.tag == tag {
                    self.rows.remove(counter);
                }
            }
        }
    }

    pub fn set_tag(&mut self, tags: &TagContainer, path: PathBuf) -> Result<(), String> {
        let tag = self.creator_tag.create_tag(&tags, path.clone());

        match tag.write_to_path(path, id3::Version::Id3v24) {
            Result::Ok(_) => Ok(()),
            Result::Err(error) => Err(error.to_string()),
        }
    }

    // pub fn parse_timestamp(&self, string_timestamp: &String) -> Timestamp {
    //     todo!()
    // }

    pub fn prepare_tag(&self) -> TagContainer {
        let mut tags = TagContainer::default();

        for row in &self.rows {
            let value = row.value.clone();
            match row.tag {
                TypeTag::Album => tags.album = Some(value),
                TypeTag::AlbumArist => tags.album_artist = Some(value),
                TypeTag::Artist => tags.artist = Some(value),
                // TypeTag::DateRecorded => tags.date_recorded = Some(self.parse_timestamp()),
                // TypeTag::DateReleased => tags.date_released = Some(self.parse_timestamp()),
                TypeTag::Disc => tags.disc = Some(value),
                TypeTag::Genre => tags.genre = Some(value),
                // TypeTag::OriginalDateReleased => tags.original_date_released = Some(self.parse_timestamp()),
                TypeTag::Text => tags.text = Some((value, String::new())),
                TypeTag::TextValues => tags.text_values = Some((value, String::new())),
                TypeTag::Title => tags.title = Some(value),
                TypeTag::TotalDiscs => tags.total_discs = Some(value),
                TypeTag::TotalTracks => tags.total_tracks = Some(value),
                TypeTag::Track => tags.track = Some(0),
                TypeTag::Year => tags.year = Some(value),
            }
        }

        tags
    }

}
