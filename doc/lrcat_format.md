The Lightroom catalog is in a sqlite database: file *.lrcat.


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

Dates: a float stored as string.... (which encoding?)

## Settings

Adobe_variablesTable

This table contain settings for Lightroom. Most of it is irrelevant. 
A few exceptions:

* Adobe_DBVersion: 0400020 (for Lightroom 4.4.1)
* AgLibraryKeyword_rootTagID: the root keyword local_id (int as string)

## Keywords

AgLibraryKeyword: keyword definitions.

* id_local: local id
* id_global: uuid
* dateCreated: creation date
* genealogy: the hierarchy
* lc_name: the (localized?) tag name
* name: the tag nane
* parent: the parent (local_id)

