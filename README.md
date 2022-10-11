# Youtube-dwn-rs
Simple youtube video downloader on rust with curl

# Usage
This program can do 2 things:
>1. Print meta info about video *(Title, description, author and views count)*
>2. Download video in variable formats

## Args structure
Exactly on this, program *have 2 sub commands*: **download** and **meta**

*Before* sub commands goes **URL** of the video. Currently this **URL** can`t be ***youtu.be***, and should be like ***youtube.com/watch***...

## Examples
### Downloading video
```bash
yt-down 'https://www.youtube.com/watch?v=8klLh8vp2ew' download 720p
```
### Getting meta about video
```bash
yt-down 'https://www.youtube.com/watch?v=8klLh8vp2ew' meta
```
