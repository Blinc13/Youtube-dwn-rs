use json::JsonValue;

pub struct YoutubeHtmlParser {

}

impl YoutubeHtmlParser {
    pub fn new(content: &str) {
        let json = extract_context_settings_json_from_string(content);
        let json = json::parse(json).unwrap();

        print_members(&json);
    }
}

fn print_members(val: &JsonValue) {
    for x in val.members() {
        print_members(x);
    }

    for x in val.entries() {
        println!("{}: {}", x.0, x.1);
    }
}

fn extract_context_settings_json_from_string(string: &str) -> &str {
    let beg = string.find("responseContext").unwrap() - 2;
    let end = beg + (string[beg..].find("};")).unwrap() + 1;

    &string[beg..end]
}