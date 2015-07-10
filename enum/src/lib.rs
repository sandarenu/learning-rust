//Explains how to use simple enum in Rust

#[derive(Debug)]
enum StatusCode {
    Success,
    Failure
}

#[derive(Debug)]
struct Response {
    status: StatusCode,
    description: String
}

impl Response {
    pub fn new(code: StatusCode, desc: &str) -> Response {
        Response {
            status: code,
            description: desc.to_string()
        }
    }
}

#[test]
fn test_enum() {

    let a = Response {
        status: StatusCode::Success,
        description: "Success".to_string()
    };

    println!("{:?}", a);

    let b = Response::new(StatusCode::Success, "Success");
    println!("{:?}", b);
}
