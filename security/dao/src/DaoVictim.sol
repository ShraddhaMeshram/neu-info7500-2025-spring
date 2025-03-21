// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract DaoVictim {

    mapping (address => uint256) public balances;

    function put() public payable {
        balances[msg.sender] += msg.value;
    }

    function get() public {
        uint256 b = balances[msg.sender];

        (bool success, ) = msg.sender.call{value: b}("");
        if (!success) {
            revert();
        }

        balances[msg.sender] = 0;
    }

}
