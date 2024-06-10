pub enum StatusCode {
    Ok,
    NotFound,
}

pub struct Response {
    pub code: StatusCode,
    pub body: String,
}

impl Response {
    pub fn into_string(&self) -> String{
        let response_status = match self.code {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        };
        let content_length = self.body.len();

        format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", response_status, content_length.to_string(), self.body)
    }
}