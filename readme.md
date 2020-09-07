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
const cryptids = new (require('crypt-ids').Cryptids)(require('fs').readFileSync('./16byteSecretKey'));
let original = 5;
let encrypted = cryptids.i2s(original);
let decrypted = cryptids.s2i(encrypted);
console.log(original, encrypted, decrypted);
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