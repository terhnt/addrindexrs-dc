// Rust Bitcoin Library
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Blockdata constants
//!
//! This module provides various constants relating to the blockchain and
//! consensus code. In particular, it defines the genesis block and its
//! single transaction
//!

use std::default::Default;

use blockdata::opcodes;
use blockdata::script;
use blockdata::transaction::{OutPoint, Transaction, TxOut, TxIn};
use blockdata::block::{Block, BlockHeader};
use network::constants::Network;
use util::misc::hex_bytes;
use util::uint::Uint256;

/// The maximum allowable sequence number
pub const MAX_SEQUENCE: u32 = 0xFFFFFFFF;
/// How many satoshis are in "one bitcoin"
pub const COIN_VALUE: u64 = 100_000_000;
/// How many seconds between blocks we expect on average
pub const TARGET_BLOCK_SPACING: u32 = 60;
/// How many blocks between diffchanges
/// KGW will change this
pub const DIFFCHANGE_INTERVAL: u32 = 3;
/// How much time on average should occur between diffchanges
pub const DIFFCHANGE_TIMESPAN: u32 = 60 * 3;
/// The maximum allowed weight for a block, see BIP 141 (network rule)
pub const MAX_BLOCK_WEIGHT: u32 = 4_000_000;
/// The minimum transaction weight for a valid serialized transaction
pub const MIN_TRANSACTION_WEIGHT: u32 = 4 * 60;


/// In Unobtaniumd this is insanely described as ~((u256)0 >> 32)
pub fn max_target(_: Network) -> Uint256 {
    Uint256::from_u64(0xFFFF).unwrap() << 208
}

/// The maximum value allowed in an output (useful for sanity checking,
/// since keeping everything below this value should prevent overflows
/// if you are doing anything remotely sane with monetary values).
pub fn max_money(_: Network) -> u64 {
    250_000 * COIN_VALUE
}

/// Constructs and returns the coinbase (and only) transaction of the Bitcoin genesis block
fn bitcoin_genesis_tx() -> Transaction {
    // Base
    let mut ret = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![],
        output: vec![],
    };

    // Inputs
    let in_script = script::Builder::new().push_scriptint(504365055)
                                          .push_scriptint(4)
                                          .push_slice(b"San Francisco plaza evacuated after suspicious package is found")
                                          .into_script();
    ret.input.push(TxIn {
        previous_output: OutPoint::null(),
        script_sig: in_script,
        sequence: MAX_SEQUENCE,
        witness: vec![],
    });

    // Outputs
    let out_script = script::Builder::new()
        .push_slice(&hex_bytes("04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f").unwrap())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();
    ret.output.push(TxOut {
        value: 1 * COIN_VALUE,
        script_pubkey: out_script
    });

    // end
    ret
}

/// Constructs and returns the genesis block
pub fn genesis_block(network: Network) -> Block {
    match network {
        Network::Bitcoin => {
            let txdata = vec![bitcoin_genesis_tx()];
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Default::default(),
                    merkle_root: txdata[0].txid(),
                    time: 1375548986,
                    bits: 0x1e0fffff,
                    nonce: 1211565
                },
                txdata: txdata
            }
        }
        Network::Testnet => {
            let txdata = vec![bitcoin_genesis_tx()];
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Default::default(),
                    merkle_root: txdata[0].txid(),
                    time: 1531487000,
                    bits: 0x1e0fffff,
                    nonce: 1746468
                },
                txdata: txdata
            }
        }
        Network::Regtest => {
            let txdata = vec![bitcoin_genesis_tx()];
            Block {
                header: BlockHeader {
                    version: 1,
                    prev_blockhash: Default::default(),
                    merkle_root: txdata[0].txid(),
                    time: 1375548985,
                    bits: 0x207fffff,
                    nonce: 1
                },
                txdata: txdata
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use hex::decode as hex_decode;

    use network::constants::Network;
    use consensus::encode::serialize;
    use blockdata::constants::{genesis_block, bitcoin_genesis_tx};
    use blockdata::constants::{MAX_SEQUENCE, COIN_VALUE};
    use util::hash::BitcoinHash;

    #[test]
    fn bitcoin_genesis_first_transaction() {
        let gen = bitcoin_genesis_tx();

        assert_eq!(gen.version, 1);
        assert_eq!(gen.input.len(), 1);
        assert_eq!(gen.input[0].previous_output.txid, Default::default());
        assert_eq!(gen.input[0].previous_output.vout, 0xFFFFFFFF);
        assert_eq!(serialize(&gen.input[0].script_sig),
                   hex_decode("4d04ffff001d01043f53616e204672616e636973636f20706c617a612065766163756174656420616674657220737573706963696f7573207061636b61676520697320666f756e64").unwrap());

        assert_eq!(gen.input[0].sequence, MAX_SEQUENCE);
        assert_eq!(gen.output.len(), 1);
        assert_eq!(serialize(&gen.output[0].script_pubkey),
                   hex_decode("04ffff001d01043f53616e204672616e636973636f20706c617a612065766163756174656420616674657220737573706963696f7573207061636b61676520697320666f756e64").unwrap());
        assert_eq!(gen.output[0].value, 50 * COIN_VALUE);
        assert_eq!(gen.lock_time, 0);

        assert_eq!(format!("{:x}", gen.bitcoin_hash()),
                   "36a192e90f70131a884fe541a1e8a5643a28ba4cb24cbb2924bd0ee483f7f484".to_string());
    }

    #[test]
    fn bitcoin_genesis_full_block() {
        let gen = genesis_block(Network::Bitcoin);

        assert_eq!(gen.header.version, 1);
        assert_eq!(gen.header.prev_blockhash, Default::default());
        assert_eq!(format!("{:x}", gen.header.merkle_root),
                   "36a192e90f70131a884fe541a1e8a5643a28ba4cb24cbb2924bd0ee483f7f484".to_string());
        assert_eq!(gen.header.time, 1375548986);
        assert_eq!(gen.header.bits, 0x1e0fffff);
        assert_eq!(gen.header.nonce, 1211565);
        assert_eq!(format!("{:x}", gen.header.bitcoin_hash()),
                   "000004c2fc5fffb810dccc197d603690099a68305232e552d96ccbe8e2c52b75".to_string());
    }

    #[test]
    fn testnet_genesis_full_block() {
        let gen = genesis_block(Network::Testnet);
        assert_eq!(gen.header.version, 1);
        assert_eq!(gen.header.prev_blockhash, Default::default());
        assert_eq!(format!("{:x}", gen.header.merkle_root),
                  "36a192e90f70131a884fe541a1e8a5643a28ba4cb24cbb2924bd0ee483f7f484".to_string());
        assert_eq!(gen.header.time, 1531487000);
        assert_eq!(gen.header.bits, 0x1e0fffff);
        assert_eq!(gen.header.nonce, 1746468);
        assert_eq!(format!("{:x}", gen.header.bitcoin_hash()),
                   "000007b02afb00ae826d948d88f4973c00073425f965917f6298b6d280bde021".to_string());
    }
}

