extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use shared::nodes::user::User;
use regex::Regex;
// let hashed = hash("hunter2", DEFAULT_COST)?;
// let valid = verify("hunter2", &hashed)?;

// let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
// let mut claims = BTreeMap::new();
// claims.insert("sub", "someone");

// let token_str = claims.sign_with_key(&key).unwrap();

// assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");

pub fn hash_password(password: &str) -> Option<String>{
    let hashed = hash(password, DEFAULT_COST);
    match hashed {
        Ok(hashed_password) => return Some(hashed_password),
        Err(_) => return None
    }
}

pub fn validate_password(password: &str, hashed_password: &str) -> bool{

    match verify(password, hashed_password){
        Ok(valid) => return valid,
        Err(_) => return false
    }
}

pub fn validate_sign_up(user: &User) -> bool{
    //validate password
    let check_for_capital = Regex::new(r"").unwrap();
    let check_for_special_character = Regex::new(r"").unwrap();
    let check_for_number = Regex::new(r"").unwrap();
    let check_for_length = Regex::new(r"").unwrap();
    if check_for_capital.is_match("")
        && check_for_special_character.is_match("")
        && check_for_number.is_match("")
        && check_for_length.is_match(""){
            return true
    }else{
        return true;
    }
}