use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}, error::Error};
use serde::{Serialize, Deserialize};
use crate::helpers::constants::{DEFAULT_EXPIRATION, DEFAULT_SECRET};
use colored::*;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Token{
    pub token: String
}

impl Token{
    pub fn new(_claims: Vec<(&str,&str)>)->Token{
        return Token{
            token:Token::create(_claims)
        }
    }

    pub fn from_str(token:&str)->Token{
        return Token{
            token:token.to_string()
        }
    }

    fn create(_claims: Vec<(&str,&str)>)->String{
        let key: Hmac<Sha256> = Hmac::new_from_slice(DEFAULT_SECRET.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        for claim in _claims{
            claims.insert(claim.0, claim.1);
        }
        let timestamp:String;
        match get_timestamp_offset(DEFAULT_EXPIRATION){
            Ok(time)=>timestamp=time.to_string(),
            Err(_)=>timestamp=0.to_string()
        }
        claims.insert("exp", &timestamp);
        return claims.sign_with_key(&key).unwrap();
    }
    
    pub fn validate(token: &str)->bool{
        let key: Hmac<Sha256> = Hmac::new_from_slice(DEFAULT_SECRET.as_bytes()).unwrap();
        let claims: Result<BTreeMap<String, String>,jwt::Error> = token.verify_with_key(&key);
        match claims{
            Ok(claim) => {
                if check_jwt_expiration(&claim){
                    return true;
                }else{
                    println!("JWT Expired");
                    return false;
                }
            },
            Err(_) => return false
        }
    }
    
    pub fn get_claim(&self, property: &str)->Option<String>{
        let key: Hmac<Sha256> = Hmac::new_from_slice(DEFAULT_SECRET.as_bytes()).unwrap();
        let claims: Result<BTreeMap<String, String>,jwt::Error> = self.token.verify_with_key(&key);
        match claims{
            Ok(claim) => {
                if check_jwt_expiration(&claim){
                    return Some(claim[property].to_owned());
                }else{
                    println!("JWT Expired");
                    return None;
                }
            },
            Err(_) => return None
        }
    }
}

fn get_timestamp_offset(offset:u64)->Result<u64,Box<dyn Error>>{
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(time.as_secs()+(offset*60*60))
}

fn get_expiration_offset(offset:u64)->Result<u64,Box<dyn Error>>{
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(time.as_secs()-(offset*60*60))
}

fn check_jwt_expiration(claim: &BTreeMap<String, String>)->bool{
    let actual_expiration: u64;
    match claim["exp"].parse::<u64>(){
        Ok(exp)=>actual_expiration=exp,
        Err(_)=>actual_expiration=0
    }
    if actual_expiration==0{
        return false
    }
    match get_expiration_offset(DEFAULT_EXPIRATION){
        Ok(max_token_age)=>{
            if actual_expiration <= max_token_age{
                return false
            }else{
                return true
            }
        }
        Err(_)=>return false
    }
}