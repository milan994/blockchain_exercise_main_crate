use blockchain_simulator;

#[tokio::main]
async fn main() {
    println!("Starting program...\n");

    blockchain_simulator::fn_blockchain_simulator_main().await;

    println!("Main - Test\n");
}
