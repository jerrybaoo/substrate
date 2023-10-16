rm -rf ./tmp
mkdir ./tmp
killall node-template
cargo build -p node-template --release

nohup ./target/release/node-template \
--base-path ./tmp/alice/ \
--chain local \
--alice \
--port 30333 \
--unsafe-rpc-external \
--rpc-port 9944 \
--pool-limit 200000 \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
> ./tmp/log.alice 2>&1 &

nohup ./target/release/node-template \
--base-path ./tmp/bob/ \
--chain local \
--bob \
--port 30334 \
--unsafe-rpc-external \
--rpc-port 9945 \
--pool-limit 200000 \
--validator \
> ./tmp/log.bob 2>&1 &


nohup ./target/release/node-template \
--base-path ./tmp/charlie/ \
--chain local \
--charlie \
--port 30335 \
--unsafe-rpc-external \
--rpc-port 9946 \
--pool-limit 200000 \
--validator \
> ./tmp/log.charlie 2>&1 &


nohup ./target/release/node-template \
--base-path ./tmp/dave/ \
--chain local \
--dave \
--port 30336 \
--unsafe-rpc-external \
--rpc-port 9947 \
--pool-limit 200000 \
--validator \
> ./tmp/log.dave 2>&1 &


nohup ./target/release/node-template \
--base-path ./tmp/eve/ \
--chain local \
--eve \
--port 30337 \
--unsafe-rpc-external \
--pool-limit 200000 \
--rpc-port 9948 \
> ./tmp/log.eve 2>&1 &


nohup ./target/release/node-template \
--base-path ./tmp/ferdie/ \
--chain local \
--ferdie \
--port 30338 \
--rpc-port 9949 \
--unsafe-rpc-external \
--pool-limit 200000 \
--validator \
> ./tmp/log.ferdie 2>&1 &