use std::sync::Arc;
use std::thread;
use crate::{loader::http_getter::{
    StreamRequest,
    ResponseOwned,
    ResponseAs
}, parser::Format, SingleRequest};


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

        let range = self.calculate_file_range(part_number);

        self.request.set_url(&self.format_url(range));
        self.request.execute().unwrap();

        self.request.response_as_u8()
    }

    pub fn start(mut self) {
        let format = Arc::new(self.format);

        for part_number in (0..self.parts_count).step_by(2) {
            println!("Downloading fragment number {} from {}", part_number + 1, self.parts_count); // Debug

            let first_url = self.format_url(self.calculate_file_range(part_number));
            let second_url = self.format_url(self.calculate_file_range(part_number + 1));

            let first_part = thread::spawn(move || {
                SingleRequest::get(&first_url).unwrap().response()
            });
            let second_part = thread::spawn(move || {
                SingleRequest::get(&second_url).unwrap().response()
            });

            let first_part = first_part.join().unwrap();
            let second_part = second_part.join().unwrap();

            println!("{}{}",
                     String::from_utf8_lossy(&first_part),
                    String::from_utf8_lossy(&second_part)
            ); // Debug
        }
    }


    fn format_url(&self, range: (usize, usize)) -> String {
        format!("{}&range={}-{}", self.format.url, range.0, range.1)
    }

    fn calculate_file_range(&self, part_number: usize) -> (usize, usize) {
        (
            part_number * self.part_size,
            (part_number + 1) * self.part_size
        )
    }
}