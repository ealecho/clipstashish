use serde::{Deserialize, Serialize};
use crate::domain::time::Time;
use derive_more::Constructor;
#[derive(Clone, Debug, Serialize, Deserialize, Constructor)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time{
        self.0
    }

}