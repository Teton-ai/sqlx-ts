use std::env::var;

pub struct Config {
    db_host: String,
    db_port: i32,
    db_user: String,
    db_pass: String,
}

impl Config {
    pub fn new() -> Config {
        return Config {
            db_host: match var("DB_HOST") {
                Ok(s) => s,
                _ => panic!("DB_HOST environment variable is missing!"),
            },
            db_port: match var("DB_PORT") {
                Ok(s) => s.parse::<i32>().unwrap(),
                _ => panic!("DB_PORT environment variable is missing!"),
            },
            db_user: match var("DB_USER") {
                Ok(s) => s,
                _ => panic!("DB_USER environment variable is missing!"),
            },
            db_pass: match var("DB_PASS") {
                Ok(s) => s,
                _ => panic!("DB_PASS environment variable is missing!"),
            },
        };
    }

    pub fn get_postgres_cred(&self) -> String {
        format!(
            "host={} user={} password={} port={:?}",
            self.db_host, self.db_user, self.db_pass, self.db_port
        )
    }
}