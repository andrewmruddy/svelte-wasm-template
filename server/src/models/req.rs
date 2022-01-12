use crate::shared::nodes::user::User;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UserReq{
    pub token: String,
    pub user: User
}

impl UserReq{
    fn sign() -> &'static str{
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("sub", "someone");

        let token_str = claims.sign_with_key(&key).unwrap();
        return "hey";
    }
}