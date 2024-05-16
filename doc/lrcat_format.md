The Lightroom catalog is in a sqlite database: file *.lrcat.


This document is for Lightroom 4 and Lightroom 6.
Unless mentionned, info applies to both versions.

Tables
------

Common concepts:

Lots of tables contain id_local and id_global.

id_local is an integer that is supposed to be uniquely incremented
across the database.
id_global is a UUID (string) that is supposed to be unique.

Genealogy is also often found. It is a string denoting the path in the
hierarchy. Each component is the local_id, but prefix with the lenght
of the id. They are separated by '/' and starts with a '/'.

dateCreated: a time stamp of some sort. Not UNIX epoch.

## Settings

Adobe_variablesTable

This table contain settings for Lightroom. Most of it is irrelevant.
A few exceptions:

* Adobe_DBVersion:
    0200022 for Lightroom 2
    0300025 for Lightroom 3 (currently not supported)
    0400020 for Lightroom 4.4.1
    0600008 for Lightroom 6.0 - 6.13
* AgLibraryKeyword_rootTagID: the root keyword local_id (int as string)

## Keywords

AgLibraryKeyword: keyword definitions.

* id_local: local id
* id_global: uuid
* dateCreated: creation date timestamp
* genealogy: the hierarchy
* includeOnExport: whether to include on export
* includeParents: whether to include parents (on export)
* includeSynonyms: whether to include synonyms (on export)
* (Lr6) keywordType: "person" is for tag that are `Faces`.
* lc_name: the lowercase tag name
* name: the tag nane
* parent: the parent (local_id)

AgLibraryKeywordImage: keyword relation with images

* id_local: local id
* image: associated image `Adobe_images`
* tag: associated keyword `AgLibraryKeyword`

## Folders

Two kinds. Root and Folders. Root are top level folders and don't have
a parent.

AgLibraryRootFolder: root folder.

* id_local: local id
* id_global: uuid
* absolutePath: Absolute path to the root folder.
* name: name of the folder.
* relativePathFromCatalog: may be NULL if on a different volume.

AgLibraryFolder: folder. Attached to a root folder.

* id_local: local id
* id_global: uuid
* pathFromRoot: path from the root folder. Can be `NULL`.
* rootFolder: id of the root folder.

There is always a folder with an empty `pathFromRoot` for a root
folder (does this mean an AgLibraryFile is attached to only folders?)

## Content

Collections and folder have a "content" defined to declare what's
inside. They is a per containe type table.

This is done by defining key/values of properties. `owningValue` is
the key. `content` is a value.

common columns:
* id_local: local id
* owningModule: the key.
* content: value of the property

AgFolderContent:
* id_global: uuid
* containingFolder: the folder this content applies to
* name: ????

AgLibraryCollectionContent:
* collection: the collection this content applies to.

## Images

Adobe_images: image. This doesn't represent physical files.

* id_local: local id
* id_global: uuid
* fileFormat: string representing the format.
              Possible values: RAW, JPG, VIDEO, DNG
* pick: (it's a float in the database) not 1 if picked, -1 if rejected, 0 if unpicked.
* rating: rating value or NULL
* rootFile: the id of the physical file (in `AgLibraryFile`)
* orientation: text marking the orientation. ex. AB, DA. May be NULL
               for video.
   Mapping to Exif values
   * AB -> 1
   * DA -> 8
   * BC -> 6
   * CD -> 3
   * Not sure if the "mirrored" orientation follow the same scheme.
* captureTime: date capture time (likely from Exif originally or as reajusted in Lr)
* masterImage: id of master if this is a copy. NULL otherwise.
* copyName: the name of the virtual copy. masterImage not NULL.

Adobe_imageProperties: other properties for images.

* id_local: local id
* id_global: uuid
* image: id_local in the `Adobe_images` table.
* propertiesString: some properties in lron format. Seems to cover:
  - loupe focus point
    * `loupeFocusPoint`
      * `_ag_className = "AgPoint"
      * `x` and `y`: floating point 0..1.0
  - crop aspect ratio:
    * `cropAspectH`
    * `cropAspectW`
  - default crop (floating point values) :
    * `defaultCropBottom`
    * `defaultCropLeft`
    * `defaultCropRight`
    * `defaultCropTop`

AgLibraryFile: physical files.

* id_local: local id
* id_global: uuid
* baseName: name without extension
* extension: extension
* idx_filename: index entry
* importHash: hash at import time
* md5: md5 digest
* originalFilename: filename before renaming at import time
* sidecarExtensions: extensions of sidecars. Comma separated strings.
   JPG,xmp => when RAW + JPEG with xmp sidecar.
   Can be empty

Adobe_AdditionalMetadata: extra metadata for images

* id_local: local id
* id_global: uuid
* additionalInfoSet
* emeddedXmp: XMP is stored in file (vs in sidecar)
* externalXmpIsDirty: 0 or 1
* image: local id in the `Adobe_images` table.
* incrementalWhiteBalance
* internalXmpDigest: (md5 or sha1 of XMP in DB?)
* isRawFile: 1 if RAW, or 0 (likely using sidecar).
* lastSynchronizedHash: (sha1?)
* lastSyncrhonizedTimestamp
* metadataPresetID: UUID of the metadata preset applied (?)
* metadataVersion: seems to be 4.0 in Lr2, Lr4 and Lr6.
* monochrome: 1 if monochrome?
* xmp: the XMP packet text

## Collections

AgLibraryCollection (Lr3, Lr4 and Lr6) - collections definitions

* id_local: local id
* creationId: "type" of collection. Some of the possible values:
  - com.adobe.ag.library.group: Group of collection
  - com.adobe.ag.library.collection: regular collection
  - com.adobe.ag.library.smart_collection: user collection
  - com.adobe.ag.webGallery: web gallery
  - com.adobe.ag.print.unsaved: last print (internal state)
  - com.adobe.ag.webGallery.unsaved: last web gallery (not saved)
  - com.adboe.ag.slidshow.unsaved: last slideshow (not saved)
* genealogy: the hierarchy
* imageCount: ???? (NULL)
* name: String name of the collection
* parent: NULL is id_local of parent
* systemOnly: (seems to apply to the quick collection and *.unsaved)

AgLibraryCollectionImage (Lr3, Lr4 and Lr6) - image to collection relation

* id_local: local id
* collection: local id of the collection
* image: local id of the image
* pick
* positionInCollection

AgLibraryCollectionCoverImage (Lr6) - The cover image for collections

* collection: local id for `AgLibraryCollection`
* collectionImage: local id for `AgLibraryCollectionImage`

### Lr2

In Lr2, there is no collection table. Instead it seems to use a
catch-all `AgLibraryTag` table, that include collections and other
attributes.

Collections are selected with `kindName == "AgCollectionTagKind"`.
The quick collection is `kindName == "AgQuickCollectionTagKind"`.

AgLibraryTag (Lr2) - Tag definitions,

* id_local: local id
* id_glboal: global UUID
* dateCreated:
* genealogy: the hierarchy
* imageCountCache: cache of count.
* lc_name: Normalized name
* name: Display name
* parent: local id of parent. Reflected in genealogy.
* kindName: (string)
  * AgCollectionTagKind: collection
  * AgQuickCollectionTagKind: the quick collection
  * AgImportTagKind: import reference
  * AgCopyrightTagKind
  * AgCaptionTagKind
  * AgMissingFileTagKind
  * AgEntireLibraryContentOwnerTagKind
  * AgLastCatalogExportTagKind
  * AgPreviousImportContentOwnerTagKind
  * AgTempImagesTagKind
  * AgUpdatedPhotoTagKind
* sortDirection: (NULL?)
* sordOrder: (NULL?)

AgLibraryTagImage (Lr2) - Tag mapping to images.

* id_local: local id of relation
* image: local id of image (table `Adobe_images`
* tag: local id of tag
* tagKind: kind of tag. Probably related to `AgLibraryTag.kindName`.

## Faces (Lr 6 only)

AgLibraryFace - Define face detected.

* id_local: local id
* cluster: id of the cluster `AgLibraryFaceCluster`
* image: id of the image

AgLibraryFaceCluster - Group faces together

* id_local: local id of the cluster.
* keyFace: (NULL?)

AgLibraryFaceData - Data of each face

* id_local: local id
* face: local id of the face.
* data: blob. Apparently JP2.

AgLibraryKeywordFace - Face to keyword equivalence

* id_local: local id
* face: local id of the face.
* tag: local id of the keyword tag.
* userPick: user said yes.
* userReject: (rejected by user?)
* keyFace: (NULL?)
* rankOrder: ?
