//TODO: Add structure for all needed parameters in format and support for adaptiveFormats
use json::JsonValue;

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

    pub fn get_available_qualities(&self) -> Vec<&str> {
        let formats = &self.json["streamingData"]["formats"];

        formats.members()
            .map(| format | {
                format["quality"].as_str().unwrap()
            }).collect()
    }

    pub fn get_url_by_quality(&self, quality: &str) -> Option<&str> {
        let formats = &self.json["streamingData"]["formats"];

        formats.members()
            .find(| &format| {
                format["quality"].as_str().unwrap() == quality
            }).map( | format| {
                format["url"].as_str().unwrap()
            })
    }

    pub fn get_file_format_by_quality(&self, quality: &str) -> Option<&str> {
        let formats = &self.json["streamingData"]["formats"];

        formats.members()
            .find(| &format | {
                format["quality"].as_str().unwrap() == quality
            }).map( | format | {
                let mime_type = format["mimeType"].as_str().unwrap();

                let beg = mime_type.find('/').unwrap() + 1;
                let end = mime_type.find(';').unwrap();

                &mime_type[beg..end]
            })
    }
}

fn extract_context_settings_json_from_string(string: &str) -> &str {
    let beg = string.find("responseContext").unwrap() - 2;
    let end = beg + (string[beg..].find("};")).unwrap() + 1;

    &string[beg..end]
}