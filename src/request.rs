pub enum RequestType {
    Get,
    Post,
}

pub struct Request {
    pub request_type: RequestType,
    pub path: String,
}

pub fn parse_request(r: String) -> Request {
    let r = r.split(" ");
    let request_type = match r[0] {
        "GET" => RequestType::Get,
        "POST" => RequestType::Post,
        _ => panic!("Invalid Request Type"),
    };

    Request {
        request_type,
        path: r[1],
    }
}
