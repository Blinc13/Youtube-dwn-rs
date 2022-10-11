pub use loader::{SingleRequest, StreamRequest, ResponseAs};
pub use parser::YoutubeHtmlParser;
pub use args::Args;

pub mod loader;
pub mod parser;
pub mod args;


use parser::{VideoMeta, Format};

pub fn generate_file_name(meta: &VideoMeta, format: &Format) -> String {
    format!("{}-{}.{}", meta.name, meta.id, format.file_format)
}