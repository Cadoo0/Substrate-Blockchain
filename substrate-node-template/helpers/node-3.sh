../target/release/node-template --base-path $(pwd)/../../node-03 --chain ../customSpecRaw.json --port 30334 --ws-port 9945 --rpc-port 9934 --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --rpc-methods=Unsafe --name MyNode03 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$1