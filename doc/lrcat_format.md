The Lightroom catalog is in a sqlite database: file *.lrcat.


This document is for Lightroom 4 and seems to be valid for Lightroom 6.

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
* lc_name: the (localized?) tag name
* name: the tag nane
* parent: the parent (local_id)

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
* pathFromRoot: path from the root folder.
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

Adobe_Images: image. This doesn't represent physical files.

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

## Collections

AgLibraryCollection

* id_local: local id
* creationId: "type" of collection. Possible values:
  - com.adobe.ag.library.group
  - com.adobe.ag.library.collection
  - com.adobe.ag.library.smart_collection
* genealogy: the hierarchy
* imageCount: ???? (NULL)
* name: String name of the collection
* parent: NULL is id_local of parent
* systemOnly: (seems to only apply to the quick collection)