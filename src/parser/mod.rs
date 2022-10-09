pub use youtube::YoutubeHtmlParser;

pub mod youtube;


use derive_builder::Builder;

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct VideoMeta<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub duration: usize,
    pub keywords: Vec<&'a str>,
    pub description: &'a str,
    pub author: &'a str,
    pub views_count: usize
}

impl<'a> VideoMeta<'a> {
    pub fn new(name: &'a str, description: &'a str, author: &'a str, keywords: Vec<&'a str>, id: &'a str, duration: usize, views_count: usize) -> Self {
        Self {
            name,
            description,
            author,
            keywords,
            id,
            duration,
            views_count
        }
    }
}


#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Format<'a> {
    pub r#type: &'a str,
    pub file_format: &'a str,

    pub url: &'a str,
    pub quality: &'a str,  // Perhaps in the future it will be replaced by enum
    pub quality_on_page_label: &'a str,

    pub duration_ms: usize,
    pub bitrate: usize,
    pub fps: usize,

    pub size: (u16, u16)
}

impl<'a> Format<'a> {
    pub fn new(r#type: &'a str, file_format: &'a str, url: &'a str, quality: &'a str, quality_on_page_label: &'a str, duration_ms: usize, bitrate: usize, fps: usize, size: (u16, u16)) -> Self {
        Self {
            r#type,
            file_format,
            url,
            quality,
            quality_on_page_label,
            duration_ms,
            bitrate,
            fps,
            size
        }
    }
}