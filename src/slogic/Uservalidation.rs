use rocket::{outcome::IntoOutcome, request::{FromRequest, Request}};
use super::TokenInvalid;
use super::{User, Admin, verify_user};

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenInvalid;
     
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        verify_user(req.cookies()).await.map_err(|_| {TokenInvalid{}}).or_forward(())
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = TokenInvalid;
     
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let user = verify_user(req.cookies()).await;
        match user {
            Err(_e) => Err(TokenInvalid{}),
            Ok(u) => if u.is_admin {
                Ok(Admin {user: u})
            } else {
                Err(TokenInvalid{})
            }
        }.or_forward(())
    }
}