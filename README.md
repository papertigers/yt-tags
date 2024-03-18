# yt-tags

yt-tags is a tool to add id3 tag information to your tracks based upon file structure.
It operates under the assumption that you have organized your tracks in a Artist/Album/Track directory structure. I also assumes that tracks are organized by "track number - track title". Note that this is additive and that existing id3 tags will not be overwritten unless they are artist, album_artist, album, track number, or title.

For example `Hearts that Hate/Laptop EP/1 - Singing Emo.mp3` will get tagged as follows:
```
Artist: Hearts that Hate
Album Artist: Hearts that Hate
Album: Laptop EP
Tack: Singing Emo
Track No: 1
```


## Example of grabbing and tagging something with yt-dlp
```bash
#!/usr/bin/env bash

set -x
set -e

uploader=$(yt-dlp --print "%(uploader)s" "$1")

yt-dlp \
	-x \
	--embed-thumb \
	--audio-quality 0 \
	--split-chapters \
	-P "~/Music" \
	-o "%(uploader)s/%(title)s/%(title)s.%(ext)s" \
	-o "chapter:%(uploader)s/%(title)s/%(section_number)s - %(section_title)s.%(ext)s" \
	--audio-format mp3 \
	--exec rm \
	"$@"

yt-tags ~/Music/"$uploader"
```
