use std::fs::File;
use std::io::Write;
use youtube_downloader::{
    Args,
    parser::{
        Parser,
        youtube::YoutubeHtmlParser
    },
    SingleRequest,
    loader::video_loader::Loader,
    ResponseAs,
    generate_file_name
};
use clap::Parser as ClapParser;

fn main() {
    let args = Args::parse();

    let page = SingleRequest::get(&args.url).expect("Failed to execute request");
    let parser = YoutubeHtmlParser::new(&page.response_as_str()).expect("Failed to parse page");

    let formats = parser.get_video_formats();
    let meta = parser.get_video_meta();

    if args.show_formats {
        println!("Formats for \"{}\":\n", meta.name);

        for format in formats {
            println!("  {}", format.quality_on_page_label);
        }

        return;
    }

    let format = formats.iter().find(| format | {
        format.quality_on_page_label == args.format
    }).expect("Format not found");


    let mut file = File::create(generate_file_name(&meta, format)).unwrap();

    Loader::new(format)
        .download_by_workers_count(9, &mut move | buf | {
            file.write_all(&buf).unwrap();
        });
}
