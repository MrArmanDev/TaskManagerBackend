pub async fn start_database_scheduler(pool: sqlx::PgPool) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30*60)).await;

            let result = sqlx::query("UPDATE users SET attempts = 0 WHERE attempts > 0")
                .execute(&pool)
                .await;

            match result {
                Ok(res) => {
                    if res.rows_affected() > 0 {
                        println!(
                            "Clean up complete: {} users' attempts reset to 0.",
                            res.rows_affected()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Scheduler Database Error: {:?}", e);
                }
            }
        }
    });
}
