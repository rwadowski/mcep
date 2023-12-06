#[cfg(test)]
mod tests {
    use crate::engine::Data;
    use crate::{DataFrame, Name, Origin};
    use chrono::{DateTime, Utc};
    use std::time;
    use std::time::{Instant, SystemTime, UNIX_EPOCH};
    use types::definition::DefinitionId;
    use types::deployment::{BlockId, BlockInstanceId};

    #[test]
    fn test_serialize_data_frame() {
        let definition_id: DefinitionId = 1;
        let instance_id: BlockInstanceId = 1;
        let origin = Origin::from(BlockId::new(definition_id, instance_id));
        let ts = DateTime::from_timestamp(1701879374, 0).unwrap();
        let name = Name::from("value_name");
        let value = Data::Text("text".to_string());
        let frame = DataFrame::new(origin, ts, name, value);
        let expected = r#"{"origin":{"block":"1.1","source":null},"ts":1701879374000,"name":"value_name","value":{"Text":"text"}}"#
        .to_string();
        let result = serde_json::to_string(&frame).unwrap();
        assert_eq!(expected, result);
    }
}
