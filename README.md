# Album Attendance

Reports missing tracks in my music library.

Written in Rust 🦀

## Library structure

This is only intended for libraries that represent:

+ Artists as directories in the root
+ Albums as subdirectories
+ Songs as normal files in albums, with the track number in the filename's first two characters
  + eg: `"02 - Oneohtrix Point Never - Describing Bodies".mp3`

## Usage

Provide the program the path to the root of your music library.

```bash
$ album-attendance <PATH>
```
