all: install run

install:
	subxt metadata  --url wss://polkadot-asset-hub-rpc.polkadot.io:443 > statemint.scale

run:
	zombienet_sdk=debug cargo run