use std::fs::File;
use std::io::Write;
use crate::{
    loader::http_getter::{
        StreamRequest,
        ResponseAs
    },
    parser::Format
};

pub struct Loader<'a> {
    request: StreamRequest,
    format: &'a Format<'a>
}

impl<'a> Loader<'a> {
    pub fn new(format: &'a Format<'a>) -> Self {
        Self {
            request: StreamRequest::new(),
            format
        }
    }

    pub fn start(mut self, parts_count: usize) {
        if parts_count == 0 {
            panic!("Invalid parts_count value");
        }

        let duration_sec = (self.format.duration_ms as f64) / 1000.0;

        let part_len = duration_sec / parts_count as f64;
        let part_size = (part_len * self.format.bitrate as f64).trunc() as usize;

        println!("Part len - {part_len}, part size - {part_size}, duration - {duration_sec}"); // Debug

        for x in 0..parts_count+1 {
            println!("Downloading {x} part"); // Debug

            let current_pos = x * part_size;
            let next_pos = (x + 1) * part_size;

            println!("{current_pos}-{next_pos}"); // Debug

            self.request.set_url(&format!("{}&range={current_pos}-{next_pos}", self.format.url)); // Needs ti be rewritten

            let mut len = 0;

            while len == 0 {
                self.request.execute().unwrap();

                len = self.request.response_as_u8().len();
            }

            println!("{}", self.request.response_as_str());
        }
    }
}