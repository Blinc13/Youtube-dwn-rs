use crate::{
    loader::http_getter::{
        StreamRequest,
        ResponseAs
    },
    parser::Format
};

pub struct Loader<'a> {
    request: StreamRequest,
    format: &'a Format<'a>,

    part_len: f64,
    part_size: usize,
    parts_count: usize
}

impl<'a> Loader<'a> {
    pub fn new(format: &'a Format<'a>) -> Self {
        Self::new_with_parts_count(format, format.duration_ms / 1000 / 5)
    }

    pub fn new_with_parts_count(format: &'a Format<'a>, parts_count: usize) -> Self {
        if parts_count == 0 {
            panic!("Invalid parts_count value");
        }

        let duration_sec = (format.duration_ms as f64) / 1000.0;

        let part_len = duration_sec / parts_count as f64;
        let part_size = (part_len * format.bitrate as f64).trunc() as usize;

        Self {
            request: StreamRequest::new(),
            format,
            part_len,
            part_size,
            parts_count
        }
    }

    pub fn get_fragment(&mut self, fragment_number: usize) -> &[u8] {
        if fragment_number >= self.parts_count {
            panic!("Invalid frament number!");
        }

        let fragment_beg = fragment_number * self.part_size;
        let fragment_end = (fragment_number + 1) * self.part_size;

        println!("{fragment_beg} - {fragment_end}");

        self.request.set_url(&format!("{}&range={fragment_beg}-{fragment_end}", self.format.url));
        self.request.execute().unwrap();

        self.request.response_as_u8()
    }

    pub fn start(mut self) {
        for fragment_number in 0..self.parts_count {
            let fragment = self.get_fragment(fragment_number);

            println!("Fragment - {}", String::from_utf8_lossy(fragment)); // Debug
        }
    }
}