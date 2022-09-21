# Ink! XVM SDK 

The Ink! smart contract SDK for XVM interface.

This SDK contains contract wrappers and all middleware code to make XVM development easy.

## Usage

The XVM ERC20 adds EVM based ERC20 contract directly into your Ink! code. 

```rust
    #[ink(message)]
    pub fn claim(&mut self) -> bool {
        let to = [0xffu8; 20];
        let value = 424242u128;
        self.erc20.transfer(to, value)
    }
```

Transaction pass multiple layers of XVM abstractions in one line. All cross-VM communication
looks like it all going inside the smart contract.
