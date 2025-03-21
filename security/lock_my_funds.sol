pragma solidity ^0.8.0;

contract LockMyFunds {
    // time lock, to let ppl with their funds after a certain point
    uint256 public  time;

    function deposit() public payable {

    }

    function withdraw()  public  {
        uint256 currentBalance = address(this).balance;
        msg.sender.call{value: currentBalance}();
    }
}