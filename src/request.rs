pub enum RequestType {
    Get,
    Post,
}

pub struct Request {
    pub request_type: RequestType,
    pub path: String,
}

pub fn parse_request(r: String) -> Request {
    let request= r.split("\r\n").collect::<Vec<_>>();
    let request_line = request[0].split(" ").collect::<Vec<_>>();
    let request_type = match request_line[0] {
        "GET" => RequestType::Get,
        "POST" => RequestType::Post,
        _ => panic!("Invalid Request Type"),
    };
    let path = request_line[1].to_string();

    Request {
        request_type,
        path,
    }
}
