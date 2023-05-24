use tungstenite::connect;
use url::Url;

//Get the connection to the binance websocket

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
fn main() {
    //Get top 5 bids and asks for ETHBTC every 100ms
    let binance_url = format!("{}/stream?streams={}", BINANCE_WS_API, "ethbtc@depth@100ms");
    
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

        let parsed_data: serde_json::Value = serde_json::from_str(&msg).expect("Unable to parse the message");
        println!("{:?}", parsed_data);
        // println!("best ask: {}, ask size: {}", parsed_data["asks"][0][0], parsed_data["asks"][0][1]);
        // println!("best bid: {}, bid size: {}", parsed_data["bids"][0][0], parsed_data["bids"][0][1]);
        
    }
}
