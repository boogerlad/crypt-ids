# crypt-ids
Given a 128 bit secret, create functions to encrypt a 32 bit signed integer to a base58 encoded string and vice versa for things such as coupon codes, order ids, short URLs, serial numbers, etc.
## usage
```shell
npm install crypt-ids
cat /dev/urandom | head -c 16 > 16byteSecretKey
# or https://www.random.org/bytes/
# or /dev/null for testing purposes
```
```js
const b = require('fs').readFileSync('./16byteSecretKey');
const cryptids = new (require('crypt-ids').Cryptids)(new Uint8Array(b.buffer, b.byteOffset, b.byteLength));
let original = 5;
let encoded_encrypted = cryptids.i2s(original);
let decrypted_decoded = cryptids.s2i(encoded_encrypted);
console.log(original, encoded_encrypted, decrypted_decoded);
```
```
5 meGJU 5
```
## why
The type of database identity columns are commonly an integer or a UUID.

The problems with exposing an auto-incremented integer publicly is it's easy to guess another valid id and figure out how many entities there are. Why not randomize the integer? This works fine until you start getting collisions which the probability increases exponentially as the number of records approaches the maximum value of the integer. Your inserts would grind to a halt looking for an usused but random integer. What about pregenerating a randomized permutation of the sequence? Sure, if you have the entropy available to generate 2^32 random numbers and 16gb to shuffle and store the ids (assuming 32 bit integer and [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle))

What about UUIDs? While the probability that a UUID will be duplicated is not zero, it is close enough to zero to be negligible. The biggest issue with them is size. Databases work faster with smaller columns, and humans checking such a long identifier for equality is very error prone.

An auto-incremented integer is the most convenient type for a databases identity column. A linear-feedback shift register or a linear congruential generator can be used to map to a seemingly random number within the column's limits, but it's not cryptographically secure. [This](https://stackoverflow.com/a/3156231/1201238) recommends using a 32 bit block cipher, but those are rare. You could [roll your own](https://security.stackexchange.com/q/18197/123140), or use `skip32`, a derivative of `skipjack`, but the NIST does not recommend its use after 2010.

What if a proven encryption algorithm such as AES could be used? Unfortunately, its block size is too big (128 bits). [Format preserving encryption](https://en.wikipedia.org/wiki/Format-preserving_encryption) to the rescue! Using the FF1 algorithm approved by the NIST, essentially AES can be used with any block size! 
## development
I didn't actually do anything other than bundle things together...
### dependencies
* [rust](https://rustup.rs/)
* `cargo install wasm-pack`
* [nodejs](https://nodejs.org/)

### notes
this package was generated with `cargo new --lib crypt-ids`

to build the npm package from the rust source, `wasm-pack build --target nodejs`

## FAQ
### Isn't format preserving encryption slow?
With this benchmark code on a 3950x
```js
const l = 1000000;
let decrypted_decoded = new Array(l);
let encoded_encrypted = new Array(l);
const start = process.hrtime.bigint();
for(let i = 0; i < l; ++i) {
    encoded_encrypted[i] = cryptids.i2s(i);
}
const e = process.hrtime.bigint();
for(let i = 0; i < l; ++i) {
    decrypted_decoded[i] = cryptids.s2i(encrypted[i]);
}
const d = process.hrtime.bigint();
console.log((e - start) / 1000000000n, (d - e) / 1000000000n);
```
it outputs
```
33n 33n
```
which translates to 30303 encodes or decodes per second.
### What is the encoded string length?
4 to 6. In base58, leading zero bytes are encoded as '1'. If 0 is the result after encryption, in binary it's `00000000 00000000 00000000 00000000` since it's an `i32`. After encoding is `1111`. The length after the leading zero bytes is dependent on how many times it takes to integer divide the result after encryption by 58 to reach 0. (larger number => lengthier)

For example, 255 => `00000000 00000000 00000000 11111111` => "1115Q". There are three leading zero bytes.

255 % 58 = 23 => 'Q'

255 \ 58 = 4

4 % 58 = 4 => '5'

4 \ 58 = 0. Done

Another example: 65535 => `00000000 00000000 11111111 11111111` => "11LUv". There are two leading zero bytes.

65535 % 58 = 53 => 'v'

65535 \ 58 = 1129

1129 % 58 = 27 => 'U'

1129 \ 58 = 19

19 % 58 = 19 => 'L'

19 \ 58 = 0. Done
