use mongodb::{options::ClientOptions, Client, Database};
pub async fn fetch_db_connection()->Database{

    let mongo_url = "mongodb://localhost:27017";
    let database_name = "algo_trading";

    let client_options = ClientOptions::parse(mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let database = client.database(database_name);
    return database
}