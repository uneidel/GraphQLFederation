pub mod error;

use crate::auth::error::{handle_rejection};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

type Result<T> = std::result::Result<T, error::Error>;
const PK: &[u8] = b"-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAv5HKO8pWcjjhu4As0rBcGrKRrzUUBqR3/vPrSRnlrqYQhwrDuhnf3LBPXwR/6iugsDaLK84Rw/R+3l291nA7iIOf6zFAT2fFZLMDTSuLGcwbvhrtGXK3OK/UG3jIxiU3h1xg0w+Ze5UbukqaMoTUMicNs+bSZF/e8/8T2Y0H7VncKRNIkL6Uz7QmjVGoFEoCKRsq60T16pcXgYHWSa7ROWKnsuHrh5sEUQg/uxk12teSykUTpY0tzOsiISw/vCq9FX5DvbHBheVOGMU851lC9wGTMX+p62k0cvnqSH4+DqQNZSPQ5Sq/b4WO28G4jTkv5HHux3UPC21Riaw26XElTQIDAQAB\n-----END CERTIFICATE-----";
const BEARER: &str = "Bearer ";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    groups: Vec<String>,
    Company: String,
    exp: usize,
}

pub fn with_auth(role: String, key: String) -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

async fn authorize(
    (role, headers): (String, HeaderMap<HeaderValue>),
) -> std::result::Result<Claims, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let dc = &DecodingKey::from_rsa_pem(PK).unwrap();
            let decoded = decode::<Claims>(&jwt, dc, &Validation::new(Algorithm::RS256));
            let decryptedToken = match decoded {
                Ok(c) => c,
                Err(err) => match *err.kind() {
                    ErrorKind::InvalidToken => {
                        return Err(reject::custom(error::Error::JWTTokenError))
                    }
                    ErrorKind::InvalidSignature => {
                        return Err(reject::custom(error::Error::InvalidAuthHeaderError))
                    }
                    ErrorKind::ExpiredSignature => {
                        return Err(reject::custom(error::Error::ExpiredSignature))
                    }
                    _ => return Err(reject::custom(error::Error::JWTTokenError)),
                },
            };

            
            let roles:  Vec<&str>= role.split(',').collect();
            if !roles.iter().all( | role | decryptedToken.claims.groups.contains(&role.to_string())){
                println!("Not Authorized.");
                return Err(reject::custom(error::Error::NoPermissionError));
            }
            
            Ok(decryptedToken.claims)
        }

        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(error::Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(error::Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(error::Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
