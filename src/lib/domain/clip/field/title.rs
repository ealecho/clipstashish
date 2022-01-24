use super::super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    //We are using the into trait because we want to accept either an option of a title or a string as title
    //and the into fn will convert it into the appropriate type
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
       let title: Option<String> = title.into();
       match title {
           Some(title) => {
               if !title.trim().is_empty() {
                   Self(Some(title))
               } else {
                   Self(None)
               }
           }
           None => Self(None)
       }
    }

   pub fn into_inner(self) -> Option<String> {
       self.0
   }

   pub fn has_title(&self) -> bool {
       self.0.is_some()
   }

}

impl Default for Title {
     fn default() -> Self {
         Self::new(None)
     }
}

// helps us to convert form a &str into to the title type that we want
impl FromStr for Title{
    type Err = ClipError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
         Ok(Self::new(s.to_string()))
    }
}
