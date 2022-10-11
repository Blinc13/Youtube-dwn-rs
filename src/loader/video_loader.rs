//Idea: add cache to minimize ram usage
use std::fs::File;
use std::io::Write;

use std::thread;
use crate::{
    loader::http_getter::StreamRequest,
    parser::Format
};


pub struct Loader<'a> {
    format: &'a Format<'a>,

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
            format,
            part_size,
            parts_count
        }
    }

    pub fn download_by_workers_count(&self, workers_count: usize, pred: &mut dyn FnMut(Vec<u8>)) {
        let parts_for_worker = self.parts_count / workers_count;
        let missing_parts = self.parts_count % workers_count;

        let mut workers: Vec<_> = (0..workers_count)
            .map(| id | {
                let first_part_number = parts_for_worker * id;
                let last_part_number = parts_for_worker * (id + 1);

                self.construct_and_spawn(first_part_number, last_part_number)
            }).collect();

        workers.push(
            self.construct_and_spawn(
                self.parts_count - missing_parts,
                self.parts_count
            )
        );

        for handle in workers {
            pred(handle.join().unwrap());
        }
    }


    fn construct_and_spawn(&self, first_part_number: usize, last_part_number: usize) -> thread::JoinHandle<Vec<u8>> {
        thread::spawn(self.construct_worker(first_part_number, last_part_number))
    }

    fn construct_worker(&self, first_part_number: usize, last_part_number: usize) -> impl Fn() -> Vec<u8> {
        let part_size = self.part_size;
        let url = self.format.url.to_string();

        move || {
            let mut out = Vec::new();
            let mut stream = StreamRequest::new();

            for part_number in first_part_number..last_part_number {
                stream.set_url(&format_url(&url, calculate_file_range(part_size, part_number)));

                stream.execute_with_custom_pred(| buf | {
                    out.extend_from_slice(buf);

                    Ok(buf.len())
                }).unwrap();
            }

            out
        }
    }
}

#[inline]
fn format_url(url: &str, range: (usize, usize)) -> String {
    format!("{}&range={}-{}", url, range.0, range.1 - 1)
}

#[inline]
fn calculate_file_range(part_size: usize, part_number: usize) -> (usize, usize) {
    (
        part_number * part_size,
        (part_number + 1) * part_size
    )
}