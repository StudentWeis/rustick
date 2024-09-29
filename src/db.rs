pub mod db {
    use chrono::{ Utc, TimeZone };
    use chrono_tz::Asia::Shanghai;
    use polodb_core::{ bson::doc, CollectionT, Database };
    use serde::{ Deserialize, Serialize };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Log {
        pub datetime: String,
        pub message: String,
        pub ticktime: String,
    }

    pub fn insert_log(message: String, ticktime: String) {
        let db = Database::open_path("Rustick-db").unwrap();
        let collection = db.collection("logs");
        collection
            .insert_one(Log {
                datetime: Shanghai.from_utc_datetime(&Utc::now().naive_utc()).to_string(),
                message,
                ticktime,
            })
            .unwrap();
    }

    pub fn get_all_logs() -> Vec<Log> {
        let db = Database::open_path("Rustick-db").unwrap();
        let collection = db.collection("logs");
        let logs: Vec<Log> = collection
            .find(doc! {})
            .run()
            .unwrap()
            .filter_map(|res| {
                match res {
                    Ok(log) => Some(log),
                    Err(_) => None,
                }
            })
            .collect();
        logs
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insert_log() {
            insert_log("test message".to_string(), "1秒322毫秒".to_string());
        }

        #[test]
        fn test_get_logs() {
            let logs = get_all_logs();
            println!("{:?}", logs);
        }
    }
}
