use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use regex::{self, Regex};
use std::borrow::BorrowMut;

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL. Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to `generate_types = true`
pub fn prepare(
    db_conn: &DBConn,
    sql: &SQL,
    should_generate_types: &bool,
    handler: &Handler,
) -> Result<(bool, Option<TsQuery>)> {
    let mut failed = false;

    let conn = match &db_conn {
        DBConn::PostgresConn(conn) => conn,
        _ => panic!("Invalid connection type"),
    };
    let span = sql.span.to_owned();

    // Match the $/name/ pattern and replace it with $1, $2, etc.
    let re = Regex::new(r"\$\/[A-Za-z0-9_]+(:[A-Za-z]+)?\/").unwrap();
    let mut index = 0;
    let replaced = re.replace_all(&sql.query, |_caps: &regex::Captures| {
        index += 1;
        format!("${}", index)
    });

    let query = replaced.to_string();
    let prepare_query = format!("PREPARE sqlx_stmt AS {}", query);

    let result = conn.lock().unwrap().borrow_mut().query(prepare_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    } else {
        // We should only deallocate if the prepare statement was executed successfully
        let _ = &conn
            .lock()
            .unwrap()
            .borrow_mut()
            .query("DEALLOCATE sqlx_stmt", &[])
            .unwrap();
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(sql, db_conn)?);
    }

    Ok((failed, ts_query))
}
