use ton_abi::Uint;
use ton_token_abi::{PackAbi, UnpackAbi};
use ton_token_packer::BuildTokenValue;
use ton_token_unpacker::UnpackToken;

#[derive(PackAbi, UnpackAbi, PartialEq, Debug)]
enum EventType {
    ETH = 0,
    TON = 1,
}

fn test() -> EventType {
    let event = EventType::TON;
    let token = event.token_value();
    let parsed: EventType = token.unpack().unwrap();
    parsed
}

fn test_vec() -> Vec<EventType> {
    let eth_token = ton_abi::TokenValue::Uint(Uint::new(0, 8));
    let ton_token = ton_abi::TokenValue::Uint(Uint::new(1, 8));

    let tokens = ton_abi::Token::new(
        "types",
        ton_abi::TokenValue::Array(vec![eth_token, ton_token]),
    );

    let parsed: Vec<EventType> = tokens.unpack().unwrap();
    parsed
}

fn main() {
    let event = test();
    assert_eq!(event, EventType::TON);

    let vec = test_vec();
    assert_eq!(vec[0], EventType::ETH);
    assert_eq!(vec[1], EventType::TON);
}
