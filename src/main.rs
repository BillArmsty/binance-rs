use tungstenite::connect;
use url::Url;

//Get the connection to the binance websocket

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
fn main() {
    let binance_url = format!("{}/ws", BINANCE_WS_API);
    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect");

    println!("Connected to binance stream");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}: {:?}", header, _value);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
