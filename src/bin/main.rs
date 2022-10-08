use youtube_downloader::loader::{
    SingleRequest,
    ResponseAs
};
use youtube_downloader::parser::youtube::YoutubeHtmlParser;

fn main() {
    let youtube_watch = SingleRequest::get("https://www.youtube.com/watch?v=WuGcoPOT94c").unwrap();

    let x = YoutubeHtmlParser::new(&youtube_watch.response_as_str());
}
