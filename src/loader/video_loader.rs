//Idea: add cache to minimize ram usage
use std::fs::File;
use std::io::Write;
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

        let range = calculate_file_range(self.part_size, part_number);

        self.request.set_url(&format_url(self.format.url, range));
        self.request.execute().unwrap();

        self.request.response_as_u8()
    }

    pub fn start(mut self, workers_count: usize) {
        let parts_for_worker = self.parts_count / workers_count;

        let workers: Vec<_> = (0..workers_count).map(| id | {
            let part_size = self.part_size;

            let first_part_id = parts_for_worker * id;
            let last_part_id = parts_for_worker * (id + 1);

            let url = self.format.url.to_string();

            move || {
                let mut out = Vec::new();
                let mut stream = StreamRequest::new();

                for part_number in first_part_id..last_part_id {
                    stream.set_url(&format_url(&url, calculate_file_range(part_size, part_number)));

                    stream.execute().unwrap();

                    out.extend_from_slice(stream.response_as_u8());

                    println!("Id: {id} Fragment {part_number} downloaded"); // Debug
                }

                out
            }
        }).map(| worker | thread::spawn(worker)).collect();

        let mut file = File::create("video.mp4").unwrap();

        for x in workers {
            let data = x.join().unwrap();

            file.write_all(&data).unwrap();
        }

        /*
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
        }*/
    }
}

fn format_url(url: &str, range: (usize, usize)) -> String {
    format!("{}&range={}-{}", url, range.0, range.1 - 1)
}

fn calculate_file_range(part_size: usize, part_number: usize) -> (usize, usize) {
    (
        part_number * part_size,
        (part_number + 1) * part_size
    )
}