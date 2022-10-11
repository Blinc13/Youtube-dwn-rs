use std::fs::File;
use std::io::Write;
use youtube_downloader::loader::{
    SingleRequest,
    ResponseAs,
    video_loader::Loader
};
use youtube_downloader::parser::{
    youtube::YoutubeHtmlParser,
    Parser
};

fn main() {
    let youtube_watch = SingleRequest::get("https://www.youtube.com/watch?v=starRhGZa6k").unwrap();
    let video_info = YoutubeHtmlParser::new(&youtube_watch.response_as_str()).unwrap();


    let formats = video_info.get_video_formats();
    let format = &formats[0];

    println!("{:?}", video_info.get_video_meta());  // Debug
    println!("{:?}", format);                       //

    let mut file = File::create("video.mp4").unwrap();

    Loader::new(format)
        .download_by_workers_count(9, &mut move | buf |
            file.write_all(&buf).unwrap()
        );
}
