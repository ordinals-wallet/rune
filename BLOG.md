# Casey Rodamor's Blog

I'm not sure creating a new fungible token protocol for Bitcoin is a good idea. Fungible tokens are 99.9% scams and memes. However, they don't appear to be going away any time soon, similar to the way in which casinos don't appear to be going away any time soon. Creating a good fungible token protocol for Bitcoin might bring significant transaction fee revenue, developer mindshare, and users to Bitcoin. Additionally, if this protocol had a small on-chain footprint and encouraged responsible UTXO management, it might serve as harm reduction compared to existing protocols. At least one of which, BRC-20, is already quite popular, and has the undesirable consequence of UTXO proliferation.

When comparing existing fungible token protocols, there are a few important ways in which they differ:

Complexity: How complex is the protocol? Is it easy to implement? Is it easy to adopt?
User experience: Are there any implementation details which have a negative effect on the user experience? In particular, protocols that rely on off-chain data have a lighter on-chain footprint, but introduce a great deal of complexity, and require users to either run their own servers, or discover and interact with existing servers.
State model: Protocols that are UTXO-based fit more naturally into Bitcoin and promote UTXO set minimization by avoiding the creation of "junk" UTXOs.
Native token: Protocols with a native token which is required for protocol operations are cumbersome, extractive, and naturally less widely adopted.
Comparing existing fungible token protocols for Bitcoin:

BRC-20: Not UTXO-based and rather complex, since it requires use of ordinal theory for some operations.
RGB: Very complicated, relies on off-chain data, has been in development for a long time with no adoption.
Counterparty: Has a native token required for some operations, not UTXO-based.
Omni Layer: Has a native token required for some operations, not UTXO-based.
Taproot Assets: Somewhat complicated, relies on off-chain data.
What would a simple, UTXO-based fungible token protocol with a good user experience for Bitcoin look like? Here's one, called "runes", because it sounds cool.

Overview
Rune balances are held by UTXOs. A UTXO can contain any amount of any number of runes.

A transaction contains a protocol message if it contains an output whose script pubkey contains an OP_RETURN followed by a data push of the ASCII uppercase letter R. The protocol message is all data pushes after the first.

Runes input to a transaction with an invalid protocol message are burned. This allows for future upgrades that change how runes are assigned or created from creating situations where old clients erroneously assign rune balances.

Integers are encoded as prefix varints, where the number of leading ones in a varint determines its length in bytes.

Transfer
The first data push in a protocol message is decoded as a sequence integers.

These integers are interpreted as a sequence of (ID, OUTPUT, AMOUNT) tuples. If the number of decoded integers is not a multiple of three, the protocol message message is invalid.

ID is the numeric ID of the run to assign
OUTPUT is the index of the output to assign it to
AMOUNT is the amount of the run to assign
ID is encoded as a delta. This allows multiple assignments of the same rune to avoid repeating the full rune ID. For example, the tuples:

[(100, 1, 20), (0, 2 10), (20, 1, 5)]
Make the following assignments:

ID 100, output 1, 20 runes
ID 100, output 2, 10 runes
ID 120, output 1, 5 runes
The AMOUNT 0 is shorthand for "all remaining runes".

After processing all tuple assignments, any unassigned runes are assigned to the first non-OP_RETURN output, if any.

Excess assignments are ignored.

Runes may be burned by assigning them to the OP_RETURN output containing the protocol message.

Issuance
If the protocol message has a second data push, it is an issuance transaction. The second data push is decoded as two integers, SYMBOL, DECIMALS. If additional integers remain, the protocol message is invalid.

An issuance transaction may create any amount, up to 2^128 - 1 of the issued rune, using the ID 0 in assignment tuples.

SYMBOL is a base 26-encoded human readable symbol, similar to that used in ordinal number sat names. The only valid characters are A through Z.

DECIMALS is the number of digits after the decimal point that should be used when displaying the issued rune.

If SYMBOL has not already been assigned, it is assigned to the issued rune, and the issued rune receives the next available numeric rune ID, starting at one.

If SYMBOL has already been assigned, or is BITCOIN, BTC, or XBT, then no new rune is created. Issuance transaction assignments using the 0 rune ID are ignored, but other assignments are still processed.

Notes
When displaying UTXO balances, the native bitcoin balance of a UTXO can be displayed with rune ID zero and the symbol BITCOIN, BTC, or XBT.

No attempt is made to avoid symbol squatting, to keep the protocol simple. One possible, but still simple, technique to avoid symbols squatting would be to only allow assignment of symbols above a certain length, with that length decreasing over time, before eventually reaching zero and allowing all symbols. This would avoid short, desirable symbols being assigned in the early days of the protocol, and encourage competition for desirable symbols later on, when such competition might be meaningful.

Hand Wringing
Should such a thing exist? I don't know. It's about as simple as possible, does not rely on off-chain data, does not have a native token, and fits nicely into Bitcoin's native UTXO model. Such a scheme might draw users from other schemes with worse on-chain footprints, and bring developer and user mindshare to Bitcoin, encouraging them to adopt Bitcoin itself.

On the other hand, the world of fungible tokens is a near totally irredeemable pit of deceit and avarice, so it might be a wash.
