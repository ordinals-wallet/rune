# Rune

A reference implementation of the rune protocol. The goal of this project is to find consensus around the rune protocol and implement an index. 

## Testing

```
cargo test
```

## Example 

Let's examine the hex of the first ordinalswallet rune tx. See https://mempool.space/api/tx/1aa98283f61cea9125aea58441067baca2533e2bbf8218b5e4f9ef7b8c0d8c30/hex  

#### ScriptPubKey
The rune data is provided within the ScriptPubKey section, introduced by the OP_RETURN opcode, 0x6a. It is divided into substrings by OP_PUSHBYTES opcodes, here 0x01, 0x0b, 0x0a: 
01520b0001ff00752b7d000000000aff987806010000000012
= OP_RETURN 01 52 0b 0001ff00752b7d00000000 0a ff987806010000000012
-> datapush R, datapush transfer, datapush issuance

All rune tx start their ScriptPubKey with 1 pushbyte encoding the letter R in hex:  
0x 01 52  
= OP_PUSHBYTES_1 52  
= R  

This is followed by a _transfer_ data push:  
- 0x 0b 0001ff00752b7d00000000  
= OP_PUSHBYTES_11 00 01 ff 00 75 2b 7d 00 00 00 00  
= 00, 01, 00 00 00 00 7d 2b 75 00  
= _ID (hex)_ 0, _OUTPUT (hex)_ 1, _AMOUNT (varint)_ 21000000  

In a mint tx this is then followed by an additional _issuance_ data push:  
- 0x 0a ff987806010000000012  
= OP_PUSHBYTES_10 ff 98 78 06 01 00 00 00 00 12  
= 00 00 00 00 01 06 78 98, 12  
= _SYMBOL (base26)_ RUNE, _DECIMALS (hex)_ 18  

Note that the ordinalswallet implementation is encoding the _Symbol_ through Base64 and varint into little endian!  
How to decode the _Symbol_ pushstring: ff 98 78 06 01 00 00 00 00  
- 0xff tells us the next 8 bytes are little endian, which means we need to swap them around:  
00 00 00 00 01 06 78 98  
- This hex decodes to a decimal:  
17201304  
- Which in turn decodes to a character string via base26 with 00=A:  
17 20 13 04  
= RUNE   

## Other Implementations

- https://github.com/Anarkoic/runes
- https://gist.github.com/revofusion/ba74dc11e0b007feba84b7b492e5ee87
- https://github.com/rot13maxi/ruin
