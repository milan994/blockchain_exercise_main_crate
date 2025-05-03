use blockchain_simulator;
use blockchain_client_server;

#[tokio::main]
async fn main() {

    // Initialize logging
    tracing_subscriber::fmt::init();

    tracing::info!("Starting both programs: blockchain_simulator and blockchain_client_server\n");

    let jh_sim = blockchain_simulator::fn_blockchain_simulator_main();

    let jh_client_server = blockchain_client_server::blockchain_client_http_server_main();

    let _ = tokio::join!(jh_sim, jh_client_server);
}
