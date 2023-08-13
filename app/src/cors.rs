use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let request_origin = request.headers().get_one("Origin").unwrap_or_default().to_lowercase();
        let allowed_origins = [
            "localhost"
        ];

        if request_origin != "" {
            let _result = allowed_origins.iter().find(|&orign| request_origin.starts_with(orign));
            
            if _result.is_none() {
                println!("\nCors blocked {}\n", request_origin);
                return;
            }
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", request_origin));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}