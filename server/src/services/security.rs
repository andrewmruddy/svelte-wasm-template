extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use shared::models::user::User;
use regex::Regex;

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