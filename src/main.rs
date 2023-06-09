use tungstenite::connect;
use url::Url;

mod models;

//Get the connection to the binance websocket

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
fn main() {
    let binance_url =
        format!("{}/stream?streams=ethbtc@depth5@100ms/bnbeth@depth5@100ms", BINANCE_WS_API);

    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect");

    println!("Connected to binance stream");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}: {:?}", header, _value);
    }

    //Process messages in an infinite loop using serde
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        //Deserialize the message into a DepthStreamData struct

        let parsed: models::DepthStreamDataWrapper = serde_json::from_str(&msg).expect("Can't parse the message");
        for i in 0..parsed.data.asks.len() {
            println!(
                "{}: {}. ask: {}, size: {}",
                parsed.stream,
                i,
                parsed.data.asks[i].price,
                parsed.data.asks[i].size
            );
        }
    }
}
