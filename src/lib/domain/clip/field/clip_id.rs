
use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize, Constructor)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }

}

// creates a clip_id from DbId 
impl From<DbId> for ClipId {
    fn from(id: DbId) -> ClipId {
        Self(id)
    }
} 

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}