// Note: A real application would use stream_to_client since
// we'd want to send the Early Hints response before we
// have the content from the origin.

use fastly::http::StatusCode;
use fastly::{ Error, Request, Response };

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    if _req.get_path() == "/" {
        let mut ok_response = Response::from_status(StatusCode::OK);
        ok_response.set_body("Hello, world!");
        Ok(ok_response)
    } else {
        let mut early_hints_response = Response::from_status(StatusCode::from_u16(103).unwrap());
        early_hints_response.append_header("Link", "</style.css>; rel=preload; as=style");

        let body = early_hints_response.get_body_mut();
        body.write_str("HTTP/1.1 200 OK\r\n");
        body.write_str("Content-Type: text/html\r\n");
        body.write_str("\r\n");
        body.write_str("<html><body><h1>Hello, world!</h1></body></html>");

        Ok(early_hints_response)
    }
}