use std::fmt::{Display, Formatter};
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

impl Display for VideoMeta<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n\n", self.name)?;
        writeln!(f, "{}\n\n", self.description)?;
        writeln!(f, "{}\t\t\t{} views", self.author, self.views_count)
    }
}

pub trait Parser {
    fn get_video_meta(&self) -> VideoMeta;
    fn get_video_formats(&self) -> Vec<Format>;
}