use tungstenite::connect;
use url::Url;
use crate::binance::models;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms";

pub async fn process() {
    let binance_url: String = String::from(BINANCE_WS_API);
    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect to binance stream.");

    log::info!("Connected to binance stream. {}", response.status());
    log::info!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        log::info!("- {}: {:?}", header, header_value);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let _parsed: models::DepthStreamData = serde_json::from_str(&msg).expect("Can't parse");
        // for i in 0..parsed.asks.len() {
        //     println!(
        //         "{}: ask: {}, size: {}",
        //         i, parsed.asks[i].price, parsed.asks[i].size
        //     );
        // }
    }
}