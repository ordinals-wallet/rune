use super::*;

use bitcoin::{
    blockdata::{opcodes, script},
    Address, Transaction, TxOut,
};

#[derive(Debug, PartialEq)]
pub enum RuneOp {
    Issuance,
    Transfer,
}

#[derive(Debug)]
pub struct Rune {
    pub op: RuneOp,
    pub amount: u64,
    pub output_index: u64,
    pub id: u64,
    pub decimals: Option<u64>,
    pub symbol: Option<u64>,
}

impl Rune {
    pub fn name(self) -> String {
        let base26_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
        let name = self
            .symbol
            .unwrap()
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|e| {
                let value = e.iter().collect::<String>();
                let num = value.parse::<u64>().unwrap();
                base26_chars[num as usize]
            })
            .collect::<String>();

        name
    }

    pub fn from_tx(tx: Transaction) -> Result<Rune> {
        let data_out = &tx.output[0];
        let chunks: Vec<_> = data_out
            .script_pubkey
            .instructions()
            .map(|e| e.unwrap())
            .collect();

        let op = match chunks.len() {
            3 => RuneOp::Transfer,
            4 => RuneOp::Issuance,
            _ => anyhow::bail!("Invalid Rune"),
        };

        if script::Instruction::Op(opcodes::all::OP_RETURN) != chunks[0] {
            anyhow::bail!("Invalid Rune");
        }

        if script::Instruction::PushBytes(b"R") != chunks[1] {
            anyhow::bail!("Invalid Rune");
        }

        let transfer_bytes: Vec<u8>;
        if let script::Instruction::PushBytes(bytes) = chunks[2] {
            transfer_bytes = bytes.try_into().unwrap();
        } else {
            anyhow::bail!("Invalid Rune");
        }

        let mut offset;
        let (id, id_offset) = VarInt::read_bytes(transfer_bytes.clone());
        offset = id_offset as usize;
        let (output_index, output_index_offset) =
            VarInt::read_bytes((&transfer_bytes[offset..]).try_into().unwrap());
        offset = offset + output_index_offset as usize;
        let (amount, _) = VarInt::read_bytes(transfer_bytes[offset..].try_into().unwrap());

        let mut symbol = None;
        let mut decimals = None;

        if op == RuneOp::Issuance {
            let issuance_bytes: Vec<u8>;
            if let script::Instruction::PushBytes(bytes) = chunks[3] {
                issuance_bytes = bytes.try_into().unwrap();
            } else {
                anyhow::bail!("Invalid Rune");
            }

            let offset;
            let (symbol_int, symbol_offset) = VarInt::read_bytes(issuance_bytes.clone());
            offset = symbol_offset as usize;
            let (decimals_int, _) =
                VarInt::read_bytes(issuance_bytes[offset..].try_into().unwrap());

            symbol = Some(symbol_int);
            decimals = Some(decimals_int);
        }

        Ok(Rune {
            op,
            id,
            output_index,
            amount,
            symbol,
            decimals,
        })
    }

    pub fn transfer_script(id: u64, output_index: u64, amount: u64) -> Vec<u8> {
        let mut data = VarInt::get_bytes(id);
        data.append(&mut VarInt::get_bytes(output_index));
        data.append(&mut VarInt::get_bytes(amount));
        data
    }

    pub fn issuance_script(symbol: u64, decimals: u64) -> Vec<u8> {
        let mut data = VarInt::get_bytes(symbol);
        data.append(&mut VarInt::get_bytes(decimals));
        data
    }

    pub async fn issuance_outputs(
        symbol: u64,
        decimals: u64,
        amount: u64,
        to: Address,
    ) -> Result<Vec<TxOut>> {
        let outputs = vec![
            TxOut {
                value: 0,
                script_pubkey: script::Builder::new()
                    .push_opcode(opcodes::all::OP_RETURN)
                    .push_slice(b"R")
                    .push_slice(&Rune::transfer_script(0, 1, amount))
                    .push_slice(&Rune::issuance_script(symbol, decimals))
                    .into_script(),
            },
            TxOut {
                value: 546,
                script_pubkey: to.script_pubkey(),
            },
        ];

        Ok(outputs)
    }

    pub async fn transfer_outputs(amount: u64, to: Address) -> Result<Vec<TxOut>> {
        let outputs = vec![
            TxOut {
                value: 0,
                script_pubkey: script::Builder::new()
                    .push_opcode(opcodes::all::OP_RETURN)
                    .push_slice(b"R")
                    .push_slice(&Rune::transfer_script(0, 1, amount))
                    .into_script(),
            },
            TxOut {
                value: 546,
                script_pubkey: to.script_pubkey(),
            },
        ];

        Ok(outputs)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_1aa98283f61cea9125aea58441067baca2533e2bbf8218b5e4f9ef7b8c0d8c30() {
        // https://mempool.space/tx/1aa98283f61cea9125aea58441067baca2533e2bbf8218b5e4f9ef7b8c0d8c30
        // https://twitter.com/hbeckeri/status/1706537670778703967
        let rawtx = "01000000000101c305297cf18619a64dc0778d709a056184aa8760a4e95885585fd0ce3eff469e0800000000ffffffff0200000000000000001a6a01520b0001ff00752b7d000000000aff9878060100000000122d271d00000000002251208b9feef297e14e85f192e8f900efaf8852bf78542898c2a7af9550d4e137026101406e124084850794e370013ee6cfb5db762a16108df9ac8f7951140a56c3f143dc367bb9df0b424eba627e4887043140a80aab9571349a869a024a4095cb95928000000000";
        let tx: Transaction =
            bitcoin::consensus::encode::deserialize(&hex::decode(rawtx).unwrap()).unwrap();

        let rune = Rune::from_tx(tx).unwrap();

        assert_eq!(rune.op, RuneOp::Issuance);
        assert_eq!(rune.amount, 2_100_000_000);
        assert_eq!(rune.output_index, 1);
        assert_eq!(rune.id, 0);
        assert_eq!(rune.symbol, Some(17201304));
        assert_eq!(rune.decimals, Some(18));
        assert_eq!(rune.name(), "RUNE".to_string());
    }

    #[test]
    fn test_804c299bad4457daeab28c5227d36c3920d92b98dc73e4f37fe1497956d91469() {
        // https://mempool.space/tx/804c299bad4457daeab28c5227d36c3920d92b98dc73e4f37fe1497956d91469
        let rawtx = "01000000000101a616ebcf5b79993bd58fdf5830663bbb792d99c292350492857ba3d293887ba70300000000ffffffff040000000000000000126a0152030001450affe083e50000000000122202000000000000225120faae2eb2f5a9baa41050f8e2799115722cd3c9b683ea6f33e0a11caf764faa21f824010000000000225120f667578b85bed256c7fcb9f2cda488d5281e52ca42e7dd4bc21e95149562f09f9083030000000000225120045f0270cab3fd9be9c1e7b78686244ccc5b001db145ae3d9678ee254a576a0a0140ca3ed829cf459dd4fec05945eb2b215caab468105cd081b46c0f7061a005f727c5b50407d8c10494bda121b95c9097534bc3b201d93cec3c75515eef24d6623300000000";
        let tx: Transaction =
            bitcoin::consensus::encode::deserialize(&hex::decode(rawtx).unwrap()).unwrap();

        let rune = Rune::from_tx(tx).unwrap();

        assert_eq!(rune.op, RuneOp::Issuance);
        assert_eq!(rune.amount, 69);
        assert_eq!(rune.output_index, 1);
        assert_eq!(rune.id, 0);
        assert_eq!(rune.symbol, Some(15041504));
        assert_eq!(rune.decimals, Some(18));
        assert_eq!(rune.name(), "PEPE".to_string());
    }

    #[test]
    fn test_2aefe2887654b3e4e7addd8f7c6496c26110833342830c19babda8d3875072ea() {
        // https://twitter.com/revofusion/status/1706533725230792974
        // https://mempool.space/tx/2aefe2887654b3e4e7addd8f7c6496c26110833342830c19babda8d3875072ea
        let rawtx = "01000000000101112d49b15384c61a2411036a7f73e8229e5a99da33147fd00ffc3b91526732430100000000ffffffff02e803000000000000146a0152090100010104406f40010603b50c0501002682980000000000225120b96b3913b8a1fdc6c223f884a779dd66c92ed6e4465e290ae3395306f98e91b20140ae0aba07e6543256a6c0659165e3a4b91574949d9987255bb0feca619b1ffdf02e8a932b6bad668b23f36ac7bc3f15a845e76b2d62319dc7d0f9b9c6c76eaedd00000000";
        let tx: Transaction =
            bitcoin::consensus::encode::deserialize(&hex::decode(rawtx).unwrap()).unwrap();

        let rune = Rune::from_tx(tx).unwrap();

        assert_eq!(rune.op, RuneOp::Issuance);
        assert_eq!(rune.amount, 1);
        assert_eq!(rune.output_index, 0);
        assert_eq!(rune.id, 1);
        assert_eq!(rune.symbol, Some(3));
        assert_eq!(rune.decimals, Some(181));
        assert_eq!(rune.name(), "D".to_string());
    }
}
