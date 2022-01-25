use crate::domain::clip::ClipError;
 use serde::{Deserialize, Serialize};
 use std::str::FromStr;

 #[derive(Clone,  Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
 pub struct Password(Option<String>);

 impl Password {
     //We are using the into trait because we want to accept either an option of a password or a string as password
     //and the into fn will convert it into the appropriate type
     pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None))
        }
     }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

 }

/*  impl Default for Password {
      fn default() -> Self {
          Self(None)
      }
 } */

 // helps us to convert form a &str into to the password type that we want
 impl FromStr for Password{
     type Err = ClipError;
     fn from_str(s : &str) -> Result<Self, Self::Err> {
          Self::new(s.to_string())
     }
 }

 