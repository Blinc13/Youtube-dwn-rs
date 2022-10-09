//TODO: Add support for adaptiveFormats
use json::JsonValue;
use crate::parser::{Format, VideoMeta, VideoMetaBuilder, FormatBuilder};

///# Description
///This structure parses json with formats
///## What is this json ?
///This json located in one of scripts in html of Youtube watch.
///
///When you want to receive the page, youtube generates new page for every video
///and past in this json all info about formats
pub struct YoutubeHtmlParser {
    json: JsonValue
}

impl YoutubeHtmlParser {
    pub fn new(content: &str) -> Result<Self, ()> {
        let json = extract_context_settings_json_from_string(content);

        Ok (
            Self {
                json: match json::parse(json) {
                    Ok(json) => json,
                    Err(_) => return Err(())
                }
            }
        )
    }

    pub fn get_video_meta(&self) -> VideoMeta {
        let video_details = &self.json["videoDetails"];

        let length = video_details["lengthSeconds"].as_str().unwrap();
        let views_count = video_details["viewCount"].as_str().unwrap();

        VideoMetaBuilder::default()
            .name( video_details["title"].as_str().unwrap() )
            .description( video_details["shortDescription"].as_str().unwrap() )
            .author( video_details["author"].as_str().unwrap() )
            .id( video_details["videoId"].as_str().unwrap() )
            .duration( length.parse::<usize>().unwrap() )
            .views_count( views_count.parse::<usize>().unwrap() )
            .keywords(
                video_details["keywords"].members()
                    .map(| json_list_member |
                        json_list_member.as_str().unwrap()
                    ).collect::<Vec<&str>>()
            ).build().unwrap()
    }


    pub fn get_video_formats(&self) -> Vec<Format> {
        let formats = &self.json["streamingData"]["formats"];

        formats.members().map(| format_json | {
            let duration_ms = format_json["approxDurationMs"].as_str().unwrap();
            let bitrate = format_json["bitrate"].as_usize().unwrap();
            let fps = format_json["fps"].as_usize().unwrap();

            let size_width = format_json["width"].as_u16().unwrap();
            let size_height = format_json["height"].as_u16().unwrap();


            let r#type;
            let file_format;

            {
                let mime_info = format_json["mimeType"].as_str().unwrap();

                let mid = mime_info.find('/').unwrap();
                let end = mid + mime_info[mid..].find(';').unwrap();

                r#type = &mime_info[..mid];
                file_format = &mime_info[mid + 1..end];
            }


            FormatBuilder::default()
                .r#type( r#type )
                .file_format( file_format )
                .url( format_json["url"].as_str().unwrap() )
                .quality( format_json["quality"].as_str().unwrap() )
                .quality_on_page_label( format_json["qualityLabel"].as_str().unwrap() )
                .duration_ms( duration_ms.parse::<usize>().unwrap() )
                .bitrate( bitrate )
                .fps( fps )
                .size( (size_width, size_height) )
                .build().unwrap()
        }).collect()
    }
}

fn extract_context_settings_json_from_string(string: &str) -> &str {
    let beg = string.find("responseContext").unwrap() - 2;
    let end = beg + (string[beg..].find("};")).unwrap() + 1;

    &string[beg..end]
}