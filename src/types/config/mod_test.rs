#[cfg(test)]
mod test {
    use crate::types::config::{Database, Kafka, Topics};

    fn database() -> Database {
        Database {
            host: "db-host".to_string(),
            port: 5432,
            user: "mcep".to_string(),
            password: "secret".to_string(),
            name: "mcep".to_string(),
        }
    }

    fn kafka() -> Kafka {
        Kafka {
            hosts: "kafka-1:9092,kafka-2:9092".to_string(),
            topics: Topics {
                input: "input".to_string(),
                output: "output".to_string(),
            },
            client_id: "mcep-kafka".to_string(),
        }
    }

    #[test]
    fn database_url_is_assembled_from_fields() {
        assert_eq!(
            database().url(),
            "postgres://mcep:secret@db-host:5432/mcep"
        );
    }

    #[test]
    fn kafka_sink_id_appends_suffix_to_client_id() {
        assert_eq!(kafka().sink_id().value, "mcep-kafka-sink");
    }

    #[test]
    fn kafka_source_id_appends_suffix_to_client_id() {
        assert_eq!(kafka().source_id().value, "mcep-kafka-source");
    }

    #[test]
    fn kafka_host_list_splits_on_comma() {
        assert_eq!(
            kafka().host_list(),
            vec!["kafka-1:9092".to_string(), "kafka-2:9092".to_string()]
        );
    }

    #[test]
    fn kafka_host_list_single_host() {
        let mut k = kafka();
        k.hosts = "localhost:9094".to_string();
        assert_eq!(k.host_list(), vec!["localhost:9094".to_string()]);
    }
}
