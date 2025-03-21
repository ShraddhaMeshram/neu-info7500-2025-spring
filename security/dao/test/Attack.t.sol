// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {DaoVictim} from "../src/DaoVictim.sol";
import {Attacker} from "../src/Attacker.sol";


contract AttackTest is Test {
    DaoVictim public victim;
    Attacker public attacker;

    function setUp() public {
        victim = new DaoVictim();
        attacker = new Attacker(victim);
    }

    function test0() public {

    }

}
