pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TokenTKN is ERC20 {
    constructor() ERC20("TestToken", "TKN") {
        _mint(msg.sender, 1000 * 10 ** 18);
    }
}