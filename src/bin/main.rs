use std::fs::File;
use std::io::Write;
use youtube_downloader::loader::{
    SingleRequest,
    ResponseAs
};
use youtube_downloader::parser::youtube::YoutubeHtmlParser;

fn main() {
    let youtube_watch = SingleRequest::get("https://www.youtube.com/watch?v=WuGcoPOT94c").unwrap();
    let video_formats = YoutubeHtmlParser::new(&youtube_watch.response_as_str()).unwrap();


    let available_qualities = video_formats.get_available_qualities();


    let quality = *available_qualities.first().unwrap();

    let file_format = video_formats.get_file_format_by_quality(quality).unwrap();
    let file_url = video_formats.get_url_by_quality(quality).unwrap();

    let mut file = File::create(format!("video.{}", file_format)).unwrap();
    let video_data = SingleRequest::get(file_url).unwrap();

    file.write_all(video_data.response_as_u8()).unwrap();
}
