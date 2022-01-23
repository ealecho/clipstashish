use serde::{Serialize, Deserialize};

//we use this to create a new constructor function for us
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}