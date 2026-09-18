#[cfg(test)]
mod test {
    use crate::services::deployment::update::new_builder;
    use crate::types::deployment::UpdateDeployment;
    use crate::types::definition::DataType;
    use crate::types::deployment::connection::junction::BlockJunction;
    use crate::types::deployment::connection::BlockConnection;
    use crate::types::deployment::sink::{Sink, SinkId};
    use crate::types::deployment::source::{Source, SourceId};
    use crate::types::deployment::DeployedBlock;

    fn empty_update(id: i32) -> UpdateDeployment {
        UpdateDeployment {
            id,
            name: None,
            version: None,
            connections: None,
            sources: None,
            sinks: None,
            blocks: None,
        }
    }

    #[test]
    fn no_fields_set_produces_no_set_clause() {
        let update = empty_update(1);
        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE deployments SET WHERE id = $1 RETURNING *"
        );
    }

    #[test]
    fn single_field_is_bound_after_set() {
        let mut update = empty_update(2);
        update.name = Some("flow".to_string());
        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE deployments SET name = $1 WHERE id = $2 RETURNING *"
        );
    }

    #[test]
    fn multiple_fields_are_comma_separated() {
        let mut update = empty_update(3);
        update.name = Some("flow".to_string());
        update.version = Some("2.0.0".to_string());
        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE deployments SET name = $1, version = $2 WHERE id = $3 RETURNING *"
        );
    }

    #[test]
    fn all_fields_set_in_declaration_order() {
        let mut update = empty_update(4);
        update.name = Some("flow".to_string());
        update.version = Some("2.0.0".to_string());
        update.connections = Some(vec![BlockConnection {
            from: BlockJunction::from_source_id(SourceId::from("src"), DataType::Text),
            to: BlockJunction::from_sink_id(SinkId::from("snk"), DataType::Text),
        }]);
        update.sources = Some(vec![Source::new(SourceId::from("src"), DataType::Text)]);
        update.sinks = Some(vec![Sink::new(SinkId::from("snk"), DataType::Text)]);
        update.blocks = Some(vec![DeployedBlock::new(1, 1)]);

        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE deployments SET name = $1, version = $2, connections = $3, sources = $4, sinks = $5, blocks = $6 WHERE id = $7 RETURNING *"
        );
    }

    #[test]
    fn id_placeholder_always_matches_the_field_count() {
        let mut update = empty_update(42);
        update.blocks = Some(vec![DeployedBlock::new(1, 1), DeployedBlock::new(1, 2)]);
        let builder = new_builder(&update);
        assert!(builder
            .sql()
            .as_str()
            .ends_with("WHERE id = $2 RETURNING *"));
    }
}
