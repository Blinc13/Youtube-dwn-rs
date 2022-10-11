use clap::Parser as ClapParser;

use std::{
    fs::File,
    io::Write,
    process::exit
};

use yt_down::{
    args::{
        Args,
        Command
    },
    parser::{
        Parser,
        youtube::YoutubeHtmlParser,
        Format
    },
    SingleRequest,
    loader::video_loader::Loader,
    ResponseAs,
    generate_file_name
};


fn main() {
    let args = Args::parse();

    let page = SingleRequest::get(&args.url).expect("Invalid url");
    let parser = YoutubeHtmlParser::new(&page.response_as_str()).unwrap();


    let formats = parser.get_video_formats();
    let meta = parser.get_video_meta();


    match args.command {
        Command::Download{format, workers_count} => {
            let format = match find_format(&formats, &format) {
                Some(f) => f,
                None => {
                    println!("No format found!\nAvailable formats:");

                    for format in formats {
                        println!("\t{}", format.quality_on_page_label);
                    }

                    exit(1);
                }
            };

            let loader = Loader::new(format);

            let workers_count = workers_count.unwrap_or(loader.get_parts_count());


            let mut file = File::create(generate_file_name(&meta, format)).unwrap();

            loader.download_by_workers_count(workers_count, &mut | buf |
                file.write_all(&buf).unwrap()
            )
        }
        Command::Meta => {
            println!("{}", meta);
        }
    }
}

fn find_format<'a>(formats: &'a [Format], name: &'a str) -> Option<&'a Format<'a>> {
    formats.iter()
        .find(| format | {
            format.quality_on_page_label == name
        })
}