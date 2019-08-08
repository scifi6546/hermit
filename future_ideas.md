#Ideas:

## Persistant indexing of all data
	Path: Store al video names+metadata (stuff like video rating etc..) in a config file.
	On startup compare database in config file with files in video path. Do this by sorting
	the file list and the database. If a file exists but is not present in the database add 
	it. If a entry exists in the database but does not exist in the file system, then remove
	the entry in the database. I will probably roll my own databse because lightweight systems
	are not mature yet.
## More Fine grained ssl

## playlists
* Note would probably require persistant indexing of data

## Work on deploy precudure to non docker enviroments
	right now hermit requiers ffmpegthumbnailer, maby integerate it into the installed pacakge?
