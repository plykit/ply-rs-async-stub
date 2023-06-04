use crate::{Operation, PlyError};

#[derive(Debug)]
pub struct Msg {
    pub op: Operation,
    pub id: String,
    // TODO Rename to type (this is entity type name)
    pub kind: String,
    pub bytes: Vec<u8>,
}

// TODO rename from/to msg to 'entity' and combine
pub trait ToMsg {
    fn kind() -> &'static str;
    fn id(&self) -> String;
    fn to_msg(&self) -> Vec<u8>;
}

pub trait FromMsg: Sized {
    fn kind() -> &'static str;
    // TDO make this try from
    fn from_msg(msg: Vec<u8>) -> Result<Self,PlyError>;
}

// TODO explain why we do not use tryFrom, Into
// TODO or maybe we shouyld?  but ho wto constrain all this then?
