/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use rusqlite::Row;

use crate::catalog::CatalogVersion;
use crate::fromdb::FromDb;
use crate::lrobject::{LrId, LrObject};
use crate::lron;
use crate::{AspectRatio, Point, Rect};

/// Some misc properties of the image specific to Lr
#[derive(Default, Debug)]
pub struct Properties {
    /// Where the loupe is focused
    loupe_focus: Option<Point>,
    /// Aspect ration of the crop
    crop_aspect_ratio: Option<AspectRatio>,
    /// Default crop
    default_crop: Option<Rect>,
}

impl Properties {
    #[allow(clippy::unnecessary_unwrap)]
    fn loupe_focus(value: &[lron::Object]) -> Option<Point> {
        use crate::lron::{Object, Value};

        let mut x: Option<f64> = None;
        let mut y: Option<f64> = None;
        let mut is_point = false;
        value.iter().for_each(|o| {
            if let Object::Pair(p) = o {
                match p.key.as_str() {
                    "_ag_className" => is_point = p.value == Value::Str("AgPoint".to_owned()),
                    "y" => y = p.value.to_number(),
                    "x" => x = p.value.to_number(),
                    _ => {}
                }
            }
        });
        // This triggers clippy::unnecessary_unwrap
        if is_point && x.is_some() && y.is_some() {
            Some(Point {
                x: x.unwrap(),
                y: y.unwrap(),
            })
        } else {
            None
        }
    }

    #[allow(clippy::unnecessary_unwrap)]
    fn properties(value: &[lron::Object]) -> Self {
        use crate::lron::{Object, Value};

        let mut props = Properties::default();
        let mut crop_aspect_h: Option<i32> = None;
        let mut crop_aspect_w: Option<i32> = None;

        let mut top: Option<f64> = None;
        let mut bottom: Option<f64> = None;
        let mut left: Option<f64> = None;
        let mut right: Option<f64> = None;
        value.iter().for_each(|o| {
            if let Object::Pair(p) = o {
                match p.key.as_str() {
                    "loupeFocusPoint" => {
                        if let Value::Dict(ref v) = p.value {
                            props.loupe_focus = Self::loupe_focus(v);
                        }
                    }
                    "cropAspectH" => {
                        if let Value::Int(i) = p.value {
                            crop_aspect_h = Some(i);
                        }
                    }
                    "cropAspectW" => {
                        if let Value::Int(i) = p.value {
                            crop_aspect_w = Some(i);
                        }
                    }
                    "defaultCropBottom" => {
                        bottom = p.value.to_number();
                    }
                    "defaultCropLeft" => {
                        left = p.value.to_number();
                    }
                    "defaultCropRight" => {
                        right = p.value.to_number();
                    }
                    "defaultCropTop" => {
                        top = p.value.to_number();
                    }
                    _ => {}
                }
            }
        });

        // This triggers clippy::unnecessary_unwrap
        if crop_aspect_h.is_some() && crop_aspect_w.is_some() {
            props.crop_aspect_ratio = Some(AspectRatio {
                width: crop_aspect_w.unwrap(),
                height: crop_aspect_h.unwrap(),
            });
        }
        // This triggers clippy::unnecessary_unwrap
        if top.is_some() && bottom.is_some() && left.is_some() && right.is_some() {
            props.default_crop = Some(Rect {
                top: top.unwrap(),
                bottom: bottom.unwrap(),
                left: left.unwrap(),
                right: right.unwrap(),
            });
        }
        props
    }
}

impl From<lron::Object> for Properties {
    fn from(object: lron::Object) -> Self {
        use crate::lron::{Object, Value};

        match object {
            Object::Pair(ref s) => {
                if &s.key == "properties" {
                    match s.value {
                        Value::Dict(ref dict) => Self::properties(dict),
                        _ => Properties::default(),
                    }
                } else {
                    Properties::default()
                }
            }
            _ => Properties::default(),
        }
    }
}

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
    /// Orientation string (set Lr format documentation)
    /// Convert to EXIF orientation with `self.exif_orientation()`.
    pub orientation: Option<String>,
    /// Capture date.
    pub capture_time: String,
    /// XMP block as stored in the database. If len() == 0,
    /// there is no XMP.
    pub xmp: String,
    /// XMP is embedded: whether the XMP packet in the file
    /// like a JPEG, or in a sidecar like in a RAW (non DNG)
    /// file, regardless of `xmp`.
    pub xmp_embedded: bool,
    /// The external XMP (ie not in the database) is different.
    pub xmp_external_dirty: bool,
    /// Misc properties from the Adobe_imageProperties table
    pub properties: Option<Properties>,
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
    fn read_from(_version: CatalogVersion, row: &Row) -> crate::Result<Self> {
        let properties = row
            .get::<&str, String>("propertiesString")
            .ok()
            .and_then(|v| lron::Object::from_string(&v).ok())
            .map(Properties::from);
        Ok(Image {
            id: row.get("id_local")?,
            uuid: row.get("id_global")?,
            master_image: row.get("masterImage").ok(),
            rating: row.get("rating").ok(),
            root_file: row.get("rootFile")?,
            file_format: row.get("fileFormat")?,
            pick: row.get("pick")?,
            orientation: row.get("orientation").ok(),
            capture_time: row.get("captureTime")?,
            copy_name: row.get("copyName").ok(),
            xmp: row.get("xmp")?,
            xmp_embedded: row.get("embeddedXmp")?,
            xmp_external_dirty: row.get("externalXmpIsDirty")?,
            properties,
        })
    }
    fn read_db_tables(_version: CatalogVersion) -> &'static str {
        "Adobe_images as img,Adobe_AdditionalMetadata as meta,Adobe_imageProperties as props"
    }
    fn read_db_columns(_version: CatalogVersion) -> &'static str {
        "img.id_local,img.id_global,img.masterImage,img.rating,img.rootFile,img.fileFormat,cast(img.pick as integer) as pick,img.orientation,img.captureTime,img.copyName,meta.xmp,meta.embeddedXmp,meta.externalXmpIsDirty,props.propertiesString"
    }
    fn read_join_where(_version: CatalogVersion) -> &'static str {
        "meta.image = img.id_local and props.image = img.id_local"
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use super::Properties;
    use crate::lron;

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
            xmp: String::new(),
            xmp_embedded: false,
            xmp_external_dirty: false,
            properties: None,
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

    #[test]
    fn test_properties_loading() {
        const LRON1: &str = "properties = { \
	cropAspectH = 9, \
	cropAspectW = 16, \
	defaultCropBottom = 0.92105263157895, \
	defaultCropLeft = 0, \
	defaultCropRight = 1, \
	defaultCropTop = 0.078947368421053, \
	loupeFocusPoint = { \
		_ag_className = \"AgPoint\", \
		x = 0.6377015605549, \
		y = 0.70538265910057, \
	}, \
        }";

        let object = lron::Object::from_string(LRON1);

        assert!(object.is_ok());
        let object = object.unwrap();
        let properties = Properties::from(object);

        assert!(properties.loupe_focus.is_some());
        if let Some(ref loupe_focus) = properties.loupe_focus {
            assert_eq!(loupe_focus.x, 0.6377015605549);
            assert_eq!(loupe_focus.y, 0.70538265910057);
        }

        assert!(properties.crop_aspect_ratio.is_some());
        if let Some(ref ar) = properties.crop_aspect_ratio {
            assert_eq!(ar.height, 9);
            assert_eq!(ar.width, 16);
        }

        assert!(properties.default_crop.is_some());
        if let Some(ref crop) = properties.default_crop {
            assert_eq!(crop.top, 0.078947368421053);
            assert_eq!(crop.bottom, 0.92105263157895);
            assert_eq!(crop.left, 0.0);
            assert_eq!(crop.right, 1.0);
        }
    }
}
