use num_bigint::BigUint;
use ton_abi::{Int, Token, TokenValue, Uint};
use ton_token_abi::UnpackAbi;
use ton_token_unpacker::UnpackToken;
use ton_types::UInt256;

#[derive(UnpackAbi)]
#[abi(plain)]
struct Data {
    #[abi(int8)]
    data_i8: i8,
    #[abi(uint8)]
    data_u8: u8,
    #[abi(uint16)]
    data_u16: u16,
    #[abi(uint32)]
    data_u32: u32,
    #[abi(uint64)]
    data_u64: u64,
    #[abi(uint128)]
    data_u128: u128,
    #[abi(uint160)]
    data_u160: BigUint,
    #[abi(uint256)]
    data_u256: UInt256,
    #[abi(biguint128)]
    data_biguint128: BigUint,
    #[abi(bool)]
    data_bool: bool,
}

fn test() -> Data {
    let data_i8 = Token::new("data_i8", TokenValue::Int(Int::new(8, 8)));
    let data_u8 = Token::new("data_u8", TokenValue::Uint(Uint::new(8, 8)));
    let data_u16 = Token::new("data_u16", TokenValue::Uint(Uint::new(16, 16)));
    let data_u32 = Token::new("data_u32", TokenValue::Uint(Uint::new(32, 32)));
    let data_u64 = Token::new("data_u64", TokenValue::Uint(Uint::new(64, 64)));
    let data_u128 = Token::new("data_u128", TokenValue::Uint(Uint::new(128, 128)));
    let data_u160 = Token::new("data_u160", TokenValue::Uint(Uint::new(160, 160)));
    let data_u256 = Token::new("data_u256", TokenValue::Uint(Uint::new(256, 256)));
    let data_biguint128 = Token::new("data_biguint128", TokenValue::Uint(Uint::new(128, 128)));
    let data_bool = Token::new("data_bool", TokenValue::Bool(true));

    let tokens = vec![
        data_i8,
        data_u8,
        data_u16,
        data_u32,
        data_u64,
        data_u128,
        data_u160,
        data_u256,
        data_biguint128,
        data_bool,
    ];
    let parsed: Data = tokens.unpack().unwrap();

    parsed
}

fn main() {
    let data = test();

    assert_eq!(data.data_i8, 8);
    assert_eq!(data.data_u8, 8);
    assert_eq!(data.data_u16, 16);
    assert_eq!(data.data_u32, 32);
    assert_eq!(data.data_u64, 64);
    assert_eq!(data.data_u128, 128);
    assert_eq!(
        data.data_u256.to_hex_string(),
        "0000000000000000000000000000000000000000000000000000000000000100"
    );
    assert_eq!(data.data_bool, true);

    {
        let bytes = data.data_biguint128.to_bytes_be();
        assert!(bytes.len() <= 16);

        let mut padded_data = [0u8; 16];
        let offset = padded_data.len() - bytes.len();
        padded_data[offset..16].copy_from_slice(&bytes);

        assert_eq!(hex::encode(padded_data), "00000000000000000000000000000080");
    }

    {
        let bytes = data.data_u160.to_bytes_be();
        assert!(bytes.len() <= 20);

        let mut padded_data = [0u8; 20];
        let offset = padded_data.len() - bytes.len();
        padded_data[offset..20].copy_from_slice(&bytes);

        assert_eq!(
            hex::encode(padded_data),
            "00000000000000000000000000000000000000a0"
        );
    }
}
