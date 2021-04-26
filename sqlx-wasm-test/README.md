# Setup
1. Make sure postgres is installed and listening at the 5432 port.
2. Start a websocket-tcp proxy using [websocat](https://github.com/vi/websocat)
   `$ websocat --binary ws-l:127.0.0.1:8080 tcp:127.0.0.1:5432`

# Running
From the root folder of this crate:
1. `wasm-pack test --firefox -- --test`
2. Launch Firefox and navigate to [http://127.0.0.1:8000](http://127.0.0.1:8000)
