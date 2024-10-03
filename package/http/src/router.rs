use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use crate::http_response::HttpResponse;
use crate::{http_request, http_request::HttpRequest};
use std::io::prelude::*;

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }

                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
