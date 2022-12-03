use crate::api::*;
use serde::{Serialize, Serializer};
pub enum SerializeEnum {
    Record(Record),
    SerEnum(SerEnum),
}
impl Serialize for SerializeEnum {
    fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SerializeEnum::Record(ref __field0) => __field0.serialize(__serializer),
            SerializeEnum::SerEnum(ref __field0) => __field0.serialize(__serializer),
        }
    }
}
