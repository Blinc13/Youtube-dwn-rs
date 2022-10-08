use std::borrow::Cow;
use curl::{easy::Easy, Error};

const CURL_EXPECT_MESSAGE: &str = "Failed to set curl parameters";

///# Description
///On construction this struct **executes request to given url**
///and **saves response** in allocated memory
///
///# Example
///```
///use youtube_downloader::loader::http_getter::{SingleRequest, ResponseAs};
///
///let request = SingleRequest::get("https://www.google.com").unwrap();
///
///println!("{}", request.response_as_str());
/// ```
#[derive(Debug)]
pub struct SingleRequest {
    response: Vec<u8>
}

impl SingleRequest {
    pub fn get(url: &str) -> Result<Self, ()> {
        let mut easy = Easy::new();
        let mut response = Vec::new();

        easy.url(url).expect(CURL_EXPECT_MESSAGE);

        {
            let mut transfer = easy.transfer();

            transfer.write_function(| buf | {
                response.extend_from_slice(buf);

                Ok(buf.len())
            }).expect(CURL_EXPECT_MESSAGE);

            match transfer.perform() {
                Ok(_) => {}
                Err(_) => return Err(())
            };
        }

        Ok (
            Self {
                response
            }
        )
    }
}

///# Description
///On construction this struct does nothing, but you can set url
///and execute request
///
///*PS* - this struct **more optimised for multiple requests**
///
///# Example
///```
///use youtube_downloader::loader::http_getter::{StreamRequest, ResponseAs};
///
///let mut request = StreamRequest::new();
///
///request.set_url("https://www.google.com");
///request.execute().unwrap();
///
///println!("{}", request.response_as_str());
///```
#[derive(Debug)]
pub struct StreamRequest{
    easy: Easy,
    response: Option<Vec<u8>>
}

impl StreamRequest {
    pub fn new() -> Self {
        Self {
            easy: Easy::new(),
            response: None
        }
    }

    pub fn contains_response(&self) -> bool {
        match self.response {
            None => false,
            Some(_) => true
        }
    }

    pub fn set_url(&mut self, url: &str) {
        self.easy.url(url).expect(CURL_EXPECT_MESSAGE);
    }

    pub fn execute(&mut self) -> Result<(), ()> {
        let mut response = Vec::new();

        {
            let mut transfer = self.easy.transfer();

            transfer.write_function(| buf | {
                response.extend_from_slice(buf);

                Ok(buf.len())
            }).expect(CURL_EXPECT_MESSAGE);

            match transfer.perform() {
                Ok(_) => {}
                Err(_) => return Err(())
            }
        }

        self.response = Some(response);

        Ok(())
    }
}


pub trait ResponseAs {
    fn response_as_str(&self) -> Cow<str>;
    fn response_as_u8(&self) -> &[u8];
}

impl ResponseAs for SingleRequest {
    fn response_as_str(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.response)
    }

    fn response_as_u8(&self) -> &[u8] {
        &self.response
    }
}

impl ResponseAs for StreamRequest {
    fn response_as_str(&self) -> Cow<str> {
        String::from_utf8_lossy(self.response.as_ref().unwrap())
    }

    fn response_as_u8(&self) -> &[u8] {
        self.response.as_ref().unwrap()
    }
}