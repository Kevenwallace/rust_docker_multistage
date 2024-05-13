use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Defina as opções de conexão para o PostgreSQL
    let connect_options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("usr_env")
        .password("pass_env")
        .database("db_env");

    // Conectar ao banco de dados PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    
    // Executar uma consulta SQL
    let query = sqlx::query("SELECT id, nome FROM usuarios WHERE email = $1")
        .bind("kevenwallace@yahoo.com")
        .fetch_all(&pool)
        .await?;

    // Iterar sobre as linhas e imprimir os resultados
    for row in query {
        let id: i32 = row.try_get("id")?;
        let nome: String = row.try_get("nome")?;
        println!("ID: {}, Nome: {}", id, nome);
    }

    Ok(())
}