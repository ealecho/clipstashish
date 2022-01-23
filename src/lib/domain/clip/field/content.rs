use super::super::ClipError;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

// this ensures that the content that we get is always valid
impl Content {
    pub fn new(content: &str) ->  Result<Self, ClipError> {
      if !content.trim().is_empty(){
          Ok(Self(content.to_owned()))
      } else {
           Err(ClipError::EmptyContent)
      }
    }

// we are moving self, so the content struct wont exist anymore and we are converting the struct into a string
// this helps us access the string content from the tuple Content struct using indexing
    pub fn into_inner(self) -> String {
        self.0
    }

// this is useful for logging and transforming the data when we still want to keep a copy of the content around
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

