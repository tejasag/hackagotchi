use tokio_postgres::{Client, NoTls, Statement};

struct Database;

impl Database {
    pub async fn postgres_db() -> Client {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
    }
}
