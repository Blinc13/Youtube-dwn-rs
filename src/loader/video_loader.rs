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
        let part_size = ((part_len * format.bitrate as f64) / 8.0) as usize;

        Self {
            request: StreamRequest::new(),
            format,
            part_len,
            part_size,
            parts_count
        }
    }

    pub fn get_fragment(&mut self, part_number: usize) -> &[u8] {
        if part_number >= self.parts_count {
            panic!("Invalid part number!");
        }

        let part_beg = part_number * self.part_size;
        let part_end = (part_number + 1) * self.part_size;

        self.request.set_url(&format!("{}&range={part_beg}-{part_end}", self.format.url));
        self.request.execute().unwrap();

        self.request.response_as_u8()
    }

    pub fn start(mut self) {
        for part_number in 0..self.parts_count {
            println!("Downloading fragment number {} from {}", part_number + 1, self.parts_count); // Debug

            let part = self.get_fragment(part_number);

            println!("{}", String::from_utf8_lossy(part)); // Debug
        }
    }
}