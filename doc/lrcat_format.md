The Lightroom catalog is in a sqlite database: file *.lrcat.


This document is for Lightroom 4.

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
    0300025 for Lightroom 3
    0400020 for Lightroom 4.4.1
* AgLibraryKeyword_rootTagID: the root keyword local_id (int as string)

## Keywords

AgLibraryKeyword: keyword definitions.

* id_local: local id
* id_global: uuid
* dateCreated: creation date timestamp
* genealogy: the hierarchy
* lc_name: the (localized?) tag name
* name: the tag nane
* parent: the parent (local_id)

## Images

Adobe_Images: image. This doesn't represent physical files.

* id_local: local id
* id_global: uuid
* fileFormat: string representing the format. Possible values: RAW,
* pick: not 1 if picked, -1 if rejected, 0 if unpicked.
* rating: rating value or NULL
* rootFile: the id of the physical file (in `AgLibraryFile`)
* orientation: text marking the orientation. ex. AB, DA
* captureTime: date capture time (likely from Exif originally)
* masterImage: id of master if this is a copy. NULL otherwise.
* copyName: the name of the virtual copy. masterImage not NULL.

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
