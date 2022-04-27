#[cfg(test)]
mod postgres_test_happy_path_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn success_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/js-happy-path1")
            .arg("--db-type=mysql")
            .arg("--db-host=localhost")
            .arg("--db-port=33306")
            .arg("--db-user=root");
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }

    /// should not be using any arg to provide credential for DB connection
    #[test]
    fn success_with_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.env("DB_HOST", "127.0.0.1")
            .env("DB_PORT", "33306")
            .env("DB_USER", "root");
        cmd.arg("samples/generic/js-happy-path1")
            .arg("--db-type=mysql");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }

    #[test]
    fn success_with_partial_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.env("DB_HOST", "127.0.0.1")
            .env("DB_PORT", "333abc06")
            .env("DB_USER", "root");

        cmd.arg("samples/generic/js-happy-path1")
            .arg("--db-port=33306");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No SQL errors detected!"));

        Ok(())
    }
}