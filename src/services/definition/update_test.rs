#[cfg(test)]
mod test {
    use crate::services::definition::update::new_builder;
    use crate::types::definition::UpdateDefinition;

    fn empty_update(id: i32) -> UpdateDefinition {
        UpdateDefinition {
            id,
            version: None,
            name: None,
            body: None,
            body_type: None,
            description: None,
            help: None,
        }
    }

    #[test]
    fn no_fields_set_produces_no_set_clause() {
        let update = empty_update(1);
        let builder = new_builder(&update);
        assert_eq!(builder.sql().as_str(), "UPDATE definitions SET WHERE id = $1 RETURNING *");
    }

    #[test]
    fn single_field_is_bound_after_set() {
        let mut update = empty_update(2);
        update.name = Some("my_definition".to_string());
        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE definitions SET name = $1 WHERE id = $2 RETURNING *"
        );
    }

    #[test]
    fn multiple_fields_are_comma_separated() {
        let mut update = empty_update(3);
        update.name = Some("my_definition".to_string());
        update.version = Some("2.0.0".to_string());
        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE definitions SET version = $1, name = $2 WHERE id = $3 RETURNING *"
        );
    }

    #[test]
    fn all_fields_set_in_declaration_order() {
        let mut update = empty_update(4);
        update.version = Some("2.0.0".to_string());
        update.name = Some("my_definition".to_string());
        update.body = Some("{}".to_string());
        update.body_type = Some("CodeBlock".to_string());
        update.description = Some("desc".to_string());
        update.help = Some("help".to_string());

        let builder = new_builder(&update);
        assert_eq!(
            builder.sql().as_str(),
            "UPDATE definitions SET version = $1, name = $2, body = $3, body_type = $4, description = $5, help = $6 WHERE id = $7 RETURNING *"
        );
    }
}
