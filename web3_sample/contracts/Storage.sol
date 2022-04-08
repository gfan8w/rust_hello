// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22 <0.9.0;
contract Storage {
    uint public val;

    event ValueSetted(address indexed from, uint256 value, uint256 num, bytes echo);

    constructor(uint v) {
        val = v;
    }

    function setValue(uint v) public {
        val = v;
        emit ValueSetted(msg.sender, v, 2,"abc");
    }
}


