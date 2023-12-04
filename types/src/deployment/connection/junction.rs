use crate::definition::{DataType, DefinitionField, DefinitionId};
use crate::deployment::sink::SinkId;
use crate::deployment::source::SourceId;
use crate::deployment::BlockId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DefinitionJunction {
    pub block: DefinitionId,
    pub name: DefinitionField,
    pub data_type: DataType,
}

impl DefinitionJunction {}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BlockJunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sink: Option<SinkId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceId>,
    pub data_type: DataType,
}

impl BlockJunction {
    pub fn from_block_id(block_id: BlockId, data_type: DataType) -> BlockJunction {
        BlockJunction {
            block: Some(block_id),
            sink: None,
            source: None,
            data_type,
        }
    }

    pub fn from_sink_id(sink_id: SinkId, data_type: DataType) -> BlockJunction {
        BlockJunction {
            block: None,
            sink: Some(sink_id),
            source: None,
            data_type,
        }
    }

    pub fn from_source_id(source_id: SourceId, data_type: DataType) -> BlockJunction {
        BlockJunction {
            block: None,
            sink: None,
            source: Some(source_id),
            data_type,
        }
    }

    pub fn definition_id_opt(&self) -> Option<DefinitionId> {
        self.block.clone().map(|id| id.definition_id)
    }
}
