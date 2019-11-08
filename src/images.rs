/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use rusqlite::Row;

use fromdb::FromDb;
use lrobject::{LrId, LrObject};

/// An image in the `Catalog`. Requires a `LibraryFile` backing it
pub struct Image {
    id: LrId,
    uuid: String,
    /// If this a copy, id of the `Image` it is a copy of
    pub master_image: Option<LrId>,
    /// Name of copy.
    pub copy_name: Option<String>,
    /// Star rating
    pub rating: Option<i64>,
    /// Backing `LibraryFile` id.
    pub root_file: LrId,
    /// File format
    pub file_format: String,
    /// Pick. -1, 0, 1
    pub pick: i64,
    /// Orientation
    pub orientation: Option<String>,
    /// Capture date.
    pub capture_time: String,
}

impl Image {
    /// Return the Exif value for the image orientation
    /// No orientation = 0.
    /// Error = -1 or unknown value
    /// Otherwise the Exif value for `orientation`
    pub fn exif_orientation(&self) -> i32 {
        self.orientation.as_ref().map_or(0, |s| match s.as_ref() {
            "AB" => 1,
            "DA" => 8,
            "BC" => 6,
            "CD" => 3,
            _ => -1,
        })
    }
}

impl LrObject for Image {
    fn id(&self) -> LrId {
        self.id
    }
    fn uuid(&self) -> &str {
        &self.uuid
    }
}

impl FromDb for Image {
    fn read_from(row: &Row) -> Option<Self> {
        Some(Image {
            id: row.get(0),
            uuid: row.get(1),
            master_image: row.get_checked(2).ok(),
            rating: row.get_checked(3).ok(),
            root_file: row.get(4),
            file_format: row.get(5),
            pick: row.get(6),
            orientation: row.get_checked(7).ok(),
            capture_time: row.get(8),
            copy_name: row.get_checked(9).ok(),
        })
    }
    fn read_db_tables() -> &'static str {
        "Adobe_images"
    }
    fn read_db_columns() -> &'static str {
        "id_local,id_global,masterImage,rating,rootFile,fileFormat,cast(pick as integer),orientation,captureTime,copyName"
    }
}

#[cfg(test)]
#[test]
fn test_exif_orientation() {
    let mut image = Image {
        id: 1,
        uuid: String::new(),
        master_image: None,
        rating: None,
        root_file: 2,
        file_format: String::from("RAW"),
        pick: 0,
        orientation: None,
        capture_time: String::new(),
        copy_name: None,
    };

    assert_eq!(image.exif_orientation(), 0);
    image.orientation = Some(String::from("ZZ"));
    assert_eq!(image.exif_orientation(), -1);

    image.orientation = Some(String::from("AB"));
    assert_eq!(image.exif_orientation(), 1);
    image.orientation = Some(String::from("DA"));
    assert_eq!(image.exif_orientation(), 8);
    image.orientation = Some(String::from("BC"));
    assert_eq!(image.exif_orientation(), 6);
    image.orientation = Some(String::from("CD"));
    assert_eq!(image.exif_orientation(), 3);
}
