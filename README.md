<!-- Version -->
<a href="https://crates.io/crates/solid">
<img src="https://img.shields.io/crates/v/solid.svg?style=flat-square"
alt="Crates.io version" />
</a>
<!-- Downloads -->
<a href="https://crates.io/crates/solid">
<img src="https://img.shields.io/crates/d/solid.svg?style=flat-square"
    alt="Download" />
</a>
<!-- Docs -->
<a href="https://docs.rs/solid">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
    alt="docs.rs docs" />
</a>

### Solidity

#### A Solidity encoding and decoding framework for Rust.

```rust
// Basic usage using the built in `Encode` derive macro.
// (Requires the `derive` feature.)
#[derive(Encode)]
struct ContractCallEncode<'a> {
    pub name: &'a str,
    pub number: u128,
    pub bytes10: Bytes10,
    pub bytes: Bytes<'a>,
}

// Basic usage using serde. (Requires the `serde` feature).
// Note: Serde only supports a subset of the types that Solidity supports.
// If you need to support more types you'll have to use the `Encode` derive
// macro, or use the `solid::Builder` manually.
#[derive(Serialize)]
pub struct ContractCallSerde<'a> {
    // String is also supported, but it's recommened you use &str when possible.
    // pub name: String,
    pub name: &'a str,
    pub number: u128,
    pub bytes: Bytes<'a>,
    // Bytes10 cannot be serialized correctly using serde.
    // pub bytes: Bytes10,
}

// Use the `#[solid(constructor)]` attribute to declare a struct as a constructor.
// This is important because constructors do not have the function name prefix,
// unlike all other functions. Usually the struct name is used as the function
// name. To rename the function use the `#[solid(name = "<function_name>")]`
// where `<function_name>` is the name of your function.
// ie. `#[solid(name = "transfer")]`.
#[derive(Encode)]
struct ContractConstructorEncode<'a> {
    pub value: u128,
    pub string: &'a str,
}

// Basic usage with the built in `Decode` derive macro.
// (Requires the `derive` feature.)
// Note: `Uint256` and all other `Int`/`Uint` types are simple
// wrappers around `[u8; 32]`. The point of them is to support all
// `int`/`uint` Solidity types.
#[derive(Decode)]
#[solid(error)]
struct ContractCallResponse<'a> {
    int: Uint256,
    // Note: &'a [u8] is *not* the same as `Bytes<'a>`. The former is is `uint8[]` in solidity
    // while the latter is `bytes`. The two types are encoded very differently so decoding
    // `bytes` as `uint8[]` array will give you invalid data if not fail outright.
    bytes: Bytes<'a>,
    memo: &'a str,
    address: Address,
}

// Basic usage with serde's `Deserialize` derive macro.
// (Requires the `serde` feature.)
// Note: Serde only supports a subset of the types that Solidity supports.
// If you need to support more types you'll have to use the `Encode` derive
// macro, or use the `solid::Builder` manually.
#[derive(Deserialize)]
struct ContractCallResponseSerde<'a> {
    int: u128,
    bytes: &'a [u8],
    memo: &'a str,
    // There is no way to read `Address` with serde.
    // address: Address
}

// Support for composite types and `Vec`
#[derive(Encode)]
struct ContractCallComposite<'a> {
    to: (&'a str, u128),
    memos: &'a [&'a str],
    matrix: &'a [&'a [&'a [u8]]],
}

// If you want to manually build the contract you can use the provided `Builder`
let function = Builder::new()
    .name("transfer")
    .push("daniel")
    .push(10u128)
    .push(Bytes10([1u8; 10]))
    .build();
```

### [num_bigint](https://docs.rs/num-bigint/0.2.6/num_bigint/) Support

If you'd like support for `num_bigint` enable the `bigint` feature.

``` rust
// Note: BigInt is variable sized and encodes to `int256`.
// To encode to `uint256` use the `BigUint` struct.
// Also, BigInt supports numbers larger than the max value a uint256 can store, so the value
// will be truncated to 32 bytes before it's encoded.
#[derive(Encode)]
#[solid(rename = "transfer")]
struct ContractTransfer<'a> {
    amount: BigInt,
    to: &'a str
}
```

### [ethereum_types](https://docs.rs/ethereum-types/0.9.0/ethereum_types/index.html) Support

If you'd like support for `ethereum_types` enable the `ethereum_types` feature.

``` rust
// Support for Address, U256, U128 from `ethereum_types` crate.
#[derive(Encode)]
#[solid(rename = "transfer")]
struct ContractTransfer<'a> {
    amount: ethereum_types::U256,
    to: ethereum_types::Address,
}
```

### Install

```toml
# Cargo.toml

# Default features which includes `derive`, and `serde`
solid = "0.1.4"

# num_bigint support
solid = { version = "0.1.4", default-features = false, features = [ "derive", "serde", "bigint" ] }

# ethereum_types support
solid = { version = "0.1.4", default-features = false, features = [ "derive", "serde", "ethereum_types" ] }
```

#### Using [cargo-edit](https://github.com/killercup/cargo-edit)
```bash
cargo add solid
```

#### Features
 - derive: Add support for the `Encode` and `Decode` derive macros. (Recommended)
 - derse: Add support for `serde`s `Serialize` and `Deserialize` derive macros, and `to_bytes` function.
 - bigint: Add suport for `num_bigint` crate.
 - ethereum_types: Add support for `ethereum_types` crate.
 - nightly: Experimental const generic support.

### cargo-solid Subcommand

cargo-solid is a cargo subcommand that allows you to generate the encoding functions and decodable struct 
definitions for named return types of associated Solidity methods. [example](examples/cargo-solid-example/src/stateful.rs)

#### Subcommand Install

``` bash
cargo install cargo-solid
```

The following command generates the solidity abi for a contract.
``` bash
solc --combined-json abi solidity_contract.sol > solidity_contract.json
```

Then run the following command to generate the rust definition.
``` bash
cargo solid solidity_contract.json
```

The name of the output file will the be same name as the input file with the extension set to `.rs`, and will be
located in the local `src` directory. You can freely move the file if you would like, but either way you will
still need to add the file as a module: 
```rust
mod solidity_contract;
```
As of version cargo-solidv0.1.4 you can also generate the files into a directory such as `src/generated`, and
there is also support for generating code using nightly definitions of `BytesFix` and `Int`.
```bash
cargo solid --nightly -o generated stateful.json
```

[example](examples/cargo-solid-example/src/main.rs)
