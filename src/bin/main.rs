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
    let format = formats.first().unwrap();

    println!("{:?}", video_info.get_video_meta());  // Debug
    println!("{:?}", format);                       //

    Loader::new(format).start();
}
