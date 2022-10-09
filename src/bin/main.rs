use std::fs::File;
use std::io::Write;
use youtube_downloader::loader::{
    SingleRequest,
    ResponseAs
};
use youtube_downloader::parser::youtube::YoutubeHtmlParser;

fn main() {
    let youtube_watch = SingleRequest::get("https://www.youtube.com/watch?v=WuGcoPOT94c").unwrap();
    let video_info = YoutubeHtmlParser::new(&youtube_watch.response_as_str()).unwrap();


    let formats = video_info.get_video_formats();
    let format = formats.first().unwrap();

    println!("{:?}", video_info.get_video_meta());  // Debug
    println!("{:?}", format);                       //

    let file_name = format!("video.{}", format.file_format);
    let mut file = File::create(file_name).unwrap();

    let response = SingleRequest::get(format.url).unwrap();

    file.write_all(response.response_as_u8()).unwrap();
}
