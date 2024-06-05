use zombienet_sdk::{NetworkConfigBuilder, NetworkConfigExt};

#[subxt::subxt(runtime_metadata_path = "statemint.scale")]
pub mod statemint {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("rococo-local")
                .with_default_command("polkadot")
                .with_default_args(vec!["-lparachain=debug".into()])
                .with_node(|node| {
                    node.with_name("polkadot")
                })
                .with_node(|node| {
                    node.with_name("polkadot2")
                })
        })
        .with_parachain(|p| {
            p.with_id(100)
                .with_chain("asset-hub-rococo-local")
                .with_collator(|n| {
                    n.with_name("asset-hub")
                        .with_args(vec!["-lruntime=debug,parachain=trace".into()])
                        .with_ws_port(42068)
                        .with_command("polkadot-parachain")
                })
                .with_collator(|n| {
                    n.with_name("asset-hub2")
                        .with_args(vec!["-lruntime=debug,parachain=trace".into()])
                        .with_ws_port(42069)
                        .with_command("polkadot-parachain")
                })
        })
        .with_global_settings(|s| {
            s.with_node_spawn_timeout(1000)
                .with_network_spawn_timeout(1000)
        })
        .build()
        .unwrap()
        .spawn_native()
        .await.unwrap();

        let mut client = None;
        if let Ok(node) = config.get_node("asset-hub") {
            while let Err(_) = node.client::<subxt::PolkadotConfig>().await {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }

            client = node.client::<subxt::PolkadotConfig>().await.ok();
        }
        let client = client.unwrap();

        client
            .tx()
            .sign_and_submit_then_watch_default(
                &statemint::tx()
                .uniques()
                .create(1, subxt_signer::sr25519::dev::alice().public_key().into()),
                &subxt_signer::sr25519::dev::alice())
            .await.unwrap()
            .wait_for_finalized_success()
            .await.unwrap();

        println!("Transaction finalized");
        loop {}
}