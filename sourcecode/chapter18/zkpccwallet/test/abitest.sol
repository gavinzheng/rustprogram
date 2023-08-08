pragma solidity ^0.5.0;

contract ERC20ZKPNoteProxy  {

    constructor() public {
 
    }
    
    event ZKPNoteSpentnote(uint[2] a, uint[2][2] b, uint[2]  c, uint[19]  input, bytes32 notereq, uint256 amount, address recipient); 
    event ZKPNoteSpentnote001(uint[2]  a, uint[2][2]  b, uint[2]  c) ;
    event ZKPNoteSpentnote002(uint[2]  a, uint[2][2]  b, uint[2]  c,uint256 amount, address recipient);
    event ZKPNoteSpentnote003(uint[2]  a, uint[2][2]  b, uint[2]  c, bytes32 notereq, uint256 amount, address recipient) ;
    event ZKPNoteSpentnote004(uint[2]  a, uint[2][2]  b, uint[2]  c, uint[19] input);
    event ZKPNoteSpentnote005(uint[2]  a); 
    event ZKPNoteSpentnote006(uint[2]  a, uint[2][2]  b, uint[2]  c, uint[16]   input001, uint[3]  input002,bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote007(uint[2]  a, uint[2][2]  b, uint[2]  c, uint[16]   input001, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote008(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[8]   input, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote009(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[12]   input, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote010(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[6]   input, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote011(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[4]   input, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote012(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[2]   input, bytes32 notereq, uint256 amount, address recipient);// public payable returns (bool){
    event ZKPNoteSpentnote013(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[4]   input);// public payable returns (bool){
    event ZKPNoteSpentnote014(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[8]   input);// public payable returns (bool){
    event ZKPNoteSpentnote015(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[12]   input);// public payable returns (bool){
    event ZKPNoteSpentnote016(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[16]   input);// public payable returns (bool){
    event ZKPNoteSpentnote017(uint[2]  a, uint[2][2]   b, uint[2]   c, uint[5]   input, uint[8] array, bytes32 notereq, uint256 amount, address recipient);//public payable returns (bool){

    // pass    
    function spendNote001(uint[2] memory a, uint[2][2] memory b, uint[2] memory c) public payable returns (bool){
        
        emit ZKPNoteSpentnote001(a, b, c); 
        return true;      
    }
    // pass
    function spendNote002(uint[2] memory a, uint[2][2] memory b, uint[2] memory c,uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote002(a, b, c, amount, recipient); 
        return true;      
    }
    // pass
    function spendNote003(uint[2] memory a, uint[2][2] memory b, uint[2] memory c, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote003(a, b, c, notereq, amount, recipient); 
        return true;      
    }
    // Fail
    function spendNote004(uint[2] memory a, uint[2][2] memory b, uint[2]  memory c, uint[19]  memory input) public payable returns (bool){
        
        emit ZKPNoteSpentnote004(a, b, c,input); 
        return true;      
    }
    function spendNote005(uint[2] memory a) public payable returns (bool){
        
        emit ZKPNoteSpentnote005(a); 
        return true;      
    }
    function spendNote006(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[16]  memory input001, uint[3]  memory input002,bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote006(a, b, c,input001, input002, notereq, amount, recipient); 
        return true;      
    }
    //Fail
    function spendNote007(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[16]  memory input001, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote007(a, b, c,input001, notereq, amount, recipient); 
        return true;      
    }

    // Fail
    function spendNote008(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[8]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote008(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }
    function spendNote009(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[12]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote009(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }

    function spendNote010(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[6]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote010(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }
    // Fail
    function spendNote011(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[4]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote011(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }

    // Fail
    function spendNote012(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[2]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote012(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }
    // Pass
    function spendNote013(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[4]  memory input) public payable returns (bool){
        
        emit ZKPNoteSpentnote013(a, b, c,input); 
        return true;      
    }
    // pass
    function spendNote014(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[8]  memory input) public payable returns (bool){
        
        emit ZKPNoteSpentnote014(a, b, c,input); 
        return true;      
    }
    // Fail
    function spendNote015(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[12]  memory input) public payable returns (bool){
        
        emit ZKPNoteSpentnote015(a, b, c,input); 
        return true;      
    }
    function spendNote016(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[16]  memory input) public payable returns (bool){
        
        emit ZKPNoteSpentnote016(a, b, c,input); 
        return true;      
    }
    // Fail/pass
    function spendNote017(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[5]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        uint256[8] memory array;
        uint256 remainder = input[2] ;
       for (uint256 i = 0 ;i<8 ; i++ ){
           uint256 aa = 2**(32*(7-i));
           array[i] = remainder / aa;
           remainder = remainder % aa;
        }
        emit ZKPNoteSpentnote017(a, b, c,input, array, notereq, amount, recipient); 
        return true;      
    }
    function spendNote(uint[2] memory a, uint[2][2]  memory b, uint[2]  memory c, uint[19]  memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
        
        emit ZKPNoteSpentnote(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }
    function spendNoteOrig(uint[2] memory a, uint[2][2] memory b, uint[2] memory c, uint[19] memory input, bytes32 notereq, uint256 amount, address recipient) public payable returns (bool){
    
        emit ZKPNoteSpentnote(a, b, c,input, notereq, amount, recipient); 
        return true;      
    }
}