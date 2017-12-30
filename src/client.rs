use hex::ToHex;
use errors::*;
use reqwest;
use reqwest::{StatusCode, Response};
use reqwest::header::{Headers, UserAgent, ContentType};
use std::io::Read;
use rand::{OsRng, Rng};
use ring::{digest, hmac};

static API1_HOST : &'static str = "https://api.bitfinex.com/v2/";
static API_SIGNATURE_PATH : &'static str = "/api/v2/auth/r/";

#[derive(Clone)]
pub struct Client {
    api_key: String, 
    secret_key: String
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key : api_key.unwrap_or("".into()),
            secret_key : secret_key.unwrap_or("".into())
        }
    }

    pub fn get(&self, endpoint: String, request: String) -> Result<(String)> {
        let mut url: String = format!("{}{}", API1_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }
 
        let response = reqwest::get(url.as_str())?;

        self.handler(response)
    }

    pub fn post_signed(&self, endpoint: String, request: String) -> Result<(String)> {
        let url: String = format!("{}{}", API1_HOST, endpoint);

        let client = reqwest::Client::new();
        let response = client.post(url.as_str())
            .headers(self.build_headers(request))
            .send()?;

        self.handler(response)            
    } 

    fn build_headers(&self, request: String) -> Headers {
        let nonce: String = self.generate_nonce();
        let signature_path: String = format!("{}{}{}{}", API_SIGNATURE_PATH, request, nonce, "{}");

        let signed_key = hmac::SigningKey::new(&digest::SHA384, self.secret_key.as_bytes());
        let signature = hmac::sign(&signed_key, signature_path.as_bytes()).as_ref().to_hex().to_string();        

        let mut custon_headers = Headers::new();  
        custon_headers.set(UserAgent::new("bitfinex-rs"));
        custon_headers.set_raw("bfx-nonce", nonce.as_str());
        custon_headers.set_raw("bfx-apikey", self.api_key.as_str());
        custon_headers.set_raw("bfx-signature", signature.as_str());
        custon_headers.set(ContentType::json());

        custon_headers
    } 

    fn generate_nonce(&self) -> String {
        OsRng::new().unwrap()
                    .gen_ascii_chars()
                    .take(40)
                    .collect()
    }

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                return Ok(body);
            },
            StatusCode::InternalServerError => {
                bail!("Internal Server Error");
            }
            StatusCode::ServiceUnavailable => {
                bail!("Service Unavailable");
            }
            StatusCode::Unauthorized => {
                bail!("Unauthorized");
            }            
            StatusCode::BadRequest => {
                bail!(format!("Bad Request: {:?}", response));
            }                        
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        };
    }

}