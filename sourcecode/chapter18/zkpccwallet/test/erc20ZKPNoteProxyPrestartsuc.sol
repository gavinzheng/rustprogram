// File: contracts\libs\GSN\Context.sol

pragma solidity ^0.5.0;

/*
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with GSN meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 * Refer from https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/GSN/Context.sol
 */
contract Context {
    // Empty internal constructor, to prevent people from mistakenly deploying
    // an instance of this contract, which should be used via inheritance.
    constructor () internal { }
    // solhint-disable-previous-line no-empty-blocks

    function _msgSender() internal view returns (address payable) {
        return msg.sender;
    }

    function _msgData() internal view returns (bytes memory) {
        this; // silence state mutability warning without generating bytecode - see https://github.com/ethereum/solidity/issues/2691
        return msg.data;
    }
}

// File: contracts\libs\ownership\Ownable.sol

pragma solidity ^0.5.0;

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
contract Ownable is Context {
    address private _owner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    constructor () internal {
        address msgSender = _msgSender();
        _owner = msgSender;
        emit OwnershipTransferred(address(0), msgSender);
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        require(isOwner(), "Ownable: caller is not the owner");
        _;
    }

    /**
     * @dev Returns true if the caller is the current owner.
     */
    function isOwner() public view returns (bool) {
        return _msgSender() == _owner;
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() public onlyOwner {
        emit OwnershipTransferred(_owner, address(0));
        _owner = address(0);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public  onlyOwner {
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     */
    function _transferOwnership(address newOwner) internal {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        emit OwnershipTransferred(_owner, newOwner);
        _owner = newOwner;
    }
}

// File: contracts\libs\token\ERC20\IERC20.sol

pragma solidity ^0.5.0;

/**
 * @dev Interface of the ERC20 standard as defined in the EIP. Does not include
 * the optional functions; to access them see {ERC20Detailed}.
 */
interface IERC20 {
    /**
     * @dev Returns the amount of tokens in existence.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Returns the amount of tokens owned by `account`.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @dev Moves `amount` tokens from the caller's account to `recipient`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transfer(address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Returns the remaining number of tokens that `spender` will be
     * allowed to spend on behalf of `owner` through {transferFrom}. This is
     * zero by default.
     *
     * This value changes when {approve} or {transferFrom} are called.
     */
    function allowance(address owner, address spender) external view returns (uint256);

    /**
     * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * IMPORTANT: Beware that changing an allowance with this method brings the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     *
     * Emits an {Approval} event.
     */
    function approve(address spender, uint256 amount) external returns (bool);

    /**
     * @dev Moves `amount` tokens from `sender` to `recipient` using the
     * allowance mechanism. `amount` is then deducted from the caller's
     * allowance.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// File: contracts\libs\math\SafeMath.sol

pragma solidity ^0.5.0;

/**
 * @dev Wrappers over Solidity's arithmetic operations with added overflow
 * checks.
 *
 * Arithmetic operations in Solidity wrap on overflow. This can easily result
 * in bugs, because programmers usually assume that an overflow raises an
 * error, which is the standard behavior in high level programming languages.
 * `SafeMath` restores this intuition by reverting the transaction when an
 * operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafeMath {
    /**
     * @dev Returns the addition of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `+` operator.
     *
     * Requirements:
     * - Addition cannot overflow.
     */
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");

        return c;
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting on
     * overflow (when the result is negative).
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     * - Subtraction cannot overflow.
     */
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return sub(a, b, "SafeMath: subtraction overflow");
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting with custom message on
     * overflow (when the result is negative).
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     * - Subtraction cannot overflow.
     *
     * _Available since v2.4.0._
     */
    function sub(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b <= a, errorMessage);
        uint256 c = a - b;

        return c;
    }

    /**
     * @dev Returns the multiplication of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `*` operator.
     *
     * Requirements:
     * - Multiplication cannot overflow.
     */
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        // Gas optimization: this is cheaper than requiring 'a' not being zero, but the
        // benefit is lost if 'b' is also tested.
        // See: https://github.com/OpenZeppelin/openzeppelin-contracts/pull/522
        if (a == 0) {
            return 0;
        }

        uint256 c = a * b;
        require(c / a == b, "SafeMath: multiplication overflow");

        return c;
    }

    /**
     * @dev Returns the integer division of two unsigned integers. Reverts on
     * division by zero. The result is rounded towards zero.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     * - The divisor cannot be zero.
     */
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        return div(a, b, "SafeMath: division by zero");
    }

    /**
     * @dev Returns the integer division of two unsigned integers. Reverts with custom message on
     * division by zero. The result is rounded towards zero.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     * - The divisor cannot be zero.
     *
     * _Available since v2.4.0._
     */
    function div(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        // Solidity only automatically asserts when dividing by 0
        require(b != 0, errorMessage);
        uint256 c = a / b;
        // assert(a == b * c + a % b); // There is no case in which this doesn't hold

        return c;
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * Reverts when dividing by zero.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     * - The divisor cannot be zero.
     */
    function mod(uint256 a, uint256 b) internal pure returns (uint256) {
        return mod(a, b, "SafeMath: modulo by zero");
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * Reverts with custom message when dividing by zero.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     * - The divisor cannot be zero.
     *
     * _Available since v2.4.0._
     */
    function mod(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b != 0, errorMessage);
        return a % b;
    }
}

// File: contracts\libs\utils\Utils.sol

pragma solidity ^0.5.0;


library Utils {

    /* @notice      Convert the bytes array to bytes32 type, the bytes array length must be 32
    *  @param _bs   Source bytes array
    *  @return      bytes32
    */
    function bytesToBytes32(bytes memory _bs) internal pure returns (bytes32 value) {
        require(_bs.length == 32, "bytes length is not 32.");
        assembly {
            // load 32 bytes from memory starting from position _bs + 0x20 since the first 0x20 bytes stores _bs length
            value := mload(add(_bs, 0x20))
        }
    }

    /* @notice      Convert bytes to uint256
    *  @param _b    Source bytes should have length of 32
    *  @return      uint256
    */
    function bytesToUint256(bytes memory _bs) internal pure returns (uint256 value) {
        require(_bs.length == 32, "bytes length is not 32.");
        assembly {
            // load 32 bytes from memory starting from position _bs + 32
            value := mload(add(_bs, 0x20))
        }
        require(value <= 0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, "Value exceeds the range");
    }

    /* @notice      Convert uint256 to bytes
    *  @param _b    uint256 that needs to be converted
    *  @return      bytes
    */
    function uint256ToBytes(uint256 _value) internal pure returns (bytes memory bs) {
        require(_value <= 0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, "Value exceeds the range");
        assembly {
            // Get a location of some free memory and store it in result as
            // Solidity does for memory variables.
            bs := mload(0x40)
            // Put 0x20 at the first word, the length of bytes for uint256 value
            mstore(bs, 0x20)
            //In the next word, put value in bytes format to the next 32 bytes
            mstore(add(bs, 0x20), _value)
            // Update the free-memory pointer by padding our last write location to 32 bytes
            mstore(0x40, add(bs, 0x40))
        }
    }

    /* @notice      Convert bytes to address
    *  @param _bs   Source bytes: bytes length must be 20
    *  @return      Converted address from source bytes
    */
    function bytesToAddress(bytes memory _bs) internal pure returns (address addr)
    {
        require(_bs.length == 20, "bytes length does not match address");
        assembly {
            // for _bs, first word store _bs.length, second word store _bs.value
            // load 32 bytes from mem[_bs+20], convert it into Uint160, meaning we take last 20 bytes as addr (address).
            addr := mload(add(_bs, 0x14))
        }

    }
    
    /* @notice      Convert address to bytes
    *  @param _addr Address need to be converted
    *  @return      Converted bytes from address
    */
    function addressToBytes(address _addr) internal pure returns (bytes memory bs){
        assembly {
            // Get a location of some free memory and store it in result as
            // Solidity does for memory variables.
            bs := mload(0x40)
            // Put 20 (address byte length) at the first word, the length of bytes for uint256 value
            mstore(bs, 0x14)
            // logical shift left _a by 12 bytes, change _a from right-aligned to left-aligned
            mstore(add(bs, 0x20), shl(96, _addr))
            // Update the free-memory pointer by padding our last write location to 32 bytes
            mstore(0x40, add(bs, 0x40))
       }
    }

    /* @notice          Do hash leaf as the multi-chain does
    *  @param _data     Data in bytes format
    *  @return          Hashed value in bytes32 format
    */
    function hashLeaf(bytes memory _data) internal pure returns (bytes32 result)  {
        result = sha256(abi.encodePacked(byte(0x0), _data));
    }

    /* @notice          Do hash children as the multi-chain does
    *  @param _l        Left node
    *  @param _r        Right node
    *  @return          Hashed value in bytes32 format
    */
    function hashChildren(bytes32 _l, bytes32  _r) internal pure returns (bytes32 result)  {
        result = sha256(abi.encodePacked(bytes1(0x01), _l, _r));
    }

    /* @notice              Compare if two bytes are equal, which are in storage and memory, seperately
                            Refer from https://github.com/summa-tx/bitcoin-spv/blob/master/solidity/contracts/BytesLib.sol#L368
    *  @param _preBytes     The bytes stored in storage
    *  @param _postBytes    The bytes stored in memory
    *  @return              Bool type indicating if they are equal
    */
    function equalStorage(bytes storage _preBytes, bytes memory _postBytes) internal view returns (bool) {
        bool success = true;

        assembly {
            // we know _preBytes_offset is 0
            let fslot := sload(_preBytes_slot)
            // Arrays of 31 bytes or less have an even value in their slot,
            // while longer arrays have an odd value. The actual length is
            // the slot divided by two for odd values, and the lowest order
            // byte divided by two for even values.
            // If the slot is even, bitwise and the slot with 255 and divide by
            // two to get the length. If the slot is odd, bitwise and the slot
            // with -1 and divide by two.
            let slength := div(and(fslot, sub(mul(0x100, iszero(and(fslot, 1))), 1)), 2)
            let mlength := mload(_postBytes)

            // if lengths don't match the arrays are not equal
            switch eq(slength, mlength)
            case 1 {
                // fslot can contain both the length and contents of the array
                // if slength < 32 bytes so let's prepare for that
                // v. http://solidity.readthedocs.io/en/latest/miscellaneous.html#layout-of-state-variables-in-storage
                // slength != 0
                if iszero(iszero(slength)) {
                    switch lt(slength, 32)
                    case 1 {
                        // blank the last byte which is the length
                        fslot := mul(div(fslot, 0x100), 0x100)

                        if iszero(eq(fslot, mload(add(_postBytes, 0x20)))) {
                            // unsuccess:
                            success := 0
                        }
                    }
                    default {
                        // cb is a circuit breaker in the for loop since there's
                        //  no said feature for inline assembly loops
                        // cb = 1 - don't breaker
                        // cb = 0 - break
                        let cb := 1

                        // get the keccak hash to get the contents of the array
                        mstore(0x0, _preBytes_slot)
                        let sc := keccak256(0x0, 0x20)

                        let mc := add(_postBytes, 0x20)
                        let end := add(mc, mlength)

                        // the next line is the loop condition:
                        // while(uint(mc < end) + cb == 2)
                        for {} eq(add(lt(mc, end), cb), 2) {
                            sc := add(sc, 1)
                            mc := add(mc, 0x20)
                        } {
                            if iszero(eq(sload(sc), mload(mc))) {
                                // unsuccess:
                                success := 0
                                cb := 0
                            }
                        }
                    }
                }
            }
            default {
                // unsuccess:
                success := 0
            }
        }

        return success;
    }

    /* @notice              Slice the _bytes from _start index till the result has length of _length
                            Refer from https://github.com/summa-tx/bitcoin-spv/blob/master/solidity/contracts/BytesLib.sol#L246
    *  @param _bytes        The original bytes needs to be sliced
    *  @param _start        The index of _bytes for the start of sliced bytes
    *  @param _length       The index of _bytes for the end of sliced bytes
    *  @return              The sliced bytes
    */
    function slice(
        bytes memory _bytes,
        uint _start,
        uint _length
    )
        internal
        pure
        returns (bytes memory)
    {
        require(_bytes.length >= (_start + _length));

        bytes memory tempBytes;

        assembly {
            switch iszero(_length)
            case 0 {
                // Get a location of some free memory and store it in tempBytes as
                // Solidity does for memory variables.
                tempBytes := mload(0x40)

                // The first word of the slice result is potentially a partial
                // word read from the original array. To read it, we calculate
                // the length of that partial word and start copying that many
                // bytes into the array. The first word we copy will start with
                // data we don't care about, but the last `lengthmod` bytes will
                // land at the beginning of the contents of the new array. When
                // we're done copying, we overwrite the full first word with
                // the actual length of the slice.
                // lengthmod <= _length % 32
                let lengthmod := and(_length, 31)

                // The multiplication in the next line is necessary
                // because when slicing multiples of 32 bytes (lengthmod == 0)
                // the following copy loop was copying the origin's length
                // and then ending prematurely not copying everything it should.
                let mc := add(add(tempBytes, lengthmod), mul(0x20, iszero(lengthmod)))
                let end := add(mc, _length)

                for {
                    // The multiplication in the next line has the same exact purpose
                    // as the one above.
                    let cc := add(add(add(_bytes, lengthmod), mul(0x20, iszero(lengthmod))), _start)
                } lt(mc, end) {
                    mc := add(mc, 0x20)
                    cc := add(cc, 0x20)
                } {
                    mstore(mc, mload(cc))
                }

                mstore(tempBytes, _length)

                //update free-memory pointer
                //allocating the array padded to 32 bytes like the compiler does now
                mstore(0x40, and(add(mc, 31), not(31)))
            }
            //if we want a zero-length slice let's just return a zero-length array
            default {
                tempBytes := mload(0x40)

                mstore(0x40, add(tempBytes, 0x20))
            }
        }

        return tempBytes;
    }
    /* @notice              Check if the elements number of _signers within _keepers array is no less than _m
    *  @param _keepers      The array consists of serveral address
    *  @param _signers      Some specific addresses to be looked into
    *  @param _m            The number requirement paramter
    *  @return              True means containment, false meansdo do not contain.
    */
    function containMAddresses(address[] memory _keepers, address[] memory _signers, uint _m) internal pure returns (bool){
        uint m = 0;
        for(uint i = 0; i < _signers.length; i++){
            for (uint j = 0; j < _keepers.length; j++) {
                if (_signers[i] == _keepers[j]) {
                    m++;
                    delete _keepers[j];
                }
            }
        }
        return m >= _m;
    }

    /* @notice              TODO
    *  @param key
    *  @return
    */
    function compressMCPubKey(bytes memory key) internal pure returns (bytes memory newkey) {
         require(key.length >= 67, "key lenggh is too short");
         newkey = slice(key, 0, 35);
         if (uint8(key[66]) % 2 == 0){
             newkey[2] = byte(0x02);
         } else {
             newkey[2] = byte(0x03);
         }
         return newkey;
    }
    
    /**
     * @dev Returns true if `account` is a contract.
     *      Refer from https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/Address.sol#L18
     *
     * This test is non-exhaustive, and there may be false-negatives: during the
     * execution of a contract's constructor, its address will be reported as
     * not containing a contract.
     *
     * IMPORTANT: It is unsafe to assume that an address for which this
     * function returns false is an externally-owned account (EOA) and not a
     * contract.
     */
    function isContract(address account) internal view returns (bool) {
        // This method relies in extcodesize, which returns 0 for contracts in
        // construction, since the code is only stored at the end of the
        // constructor execution.

        // According to EIP-1052, 0x0 is the value returned for not-yet created accounts
        // and 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470 is returned
        // for accounts without code, i.e. `keccak256('')`
        bytes32 codehash;
        bytes32 accountHash = 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470;
        // solhint-disable-next-line no-inline-assembly
        assembly { codehash := extcodehash(account) }
        return (codehash != 0x0 && codehash != accountHash);
    }
}

// File: contracts\libs\token\ERC20\SafeERC20.sol

pragma solidity ^0.5.0;




/**
 * @title SafeERC20
 * @dev Wrappers around ERC20 operations that throw on failure (when the token
 * contract returns false). Tokens that return no value (and instead revert or
 * throw on failure) are also supported, non-reverting calls are assumed to be
 * successful.
 * To use this library you can add a `using SafeERC20 for ERC20;` statement to your contract,
 * which allows you to call the safe operations as `token.safeTransfer(...)`, etc.
 */
library SafeERC20 {
    using SafeMath for uint256;

    function safeTransfer(IERC20 token, address to, uint256 value) internal {
        callOptionalReturn(token, abi.encodeWithSelector(token.transfer.selector, to, value));
    }

    function safeTransferFrom(IERC20 token, address from, address to, uint256 value) internal {
        callOptionalReturn(token, abi.encodeWithSelector(token.transferFrom.selector, from, to, value));
    }

    function safeApprove(IERC20 token, address spender, uint256 value) internal {
        // safeApprove should only be called when setting an initial allowance,
        // or when resetting it to zero. To increase and decrease it, use
        // 'safeIncreaseAllowance' and 'safeDecreaseAllowance'
        // solhint-disable-next-line max-line-length
        require((value == 0) || (token.allowance(address(this), spender) == 0),
            "SafeERC20: approve from non-zero to non-zero allowance"
        );
        callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, value));
    }

    function safeIncreaseAllowance(IERC20 token, address spender, uint256 value) internal {
        uint256 newAllowance = token.allowance(address(this), spender).add(value);
        callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
    }

    function safeDecreaseAllowance(IERC20 token, address spender, uint256 value) internal {
        uint256 newAllowance = token.allowance(address(this), spender).sub(value);
        callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
    }

    /**
     * @dev Imitates a Solidity high-level call (i.e. a regular function call to a contract), relaxing the requirement
     * on the return value: the return value is optional (but if data is returned, it must not be false).
     * @param token The token targeted by the call.
     * @param data The call data (encoded using abi.encode or one of its variants).
     */
    function callOptionalReturn(IERC20 token, bytes memory data) private {
        // We need to perform a low level call here, to bypass Solidity's return data size checking mechanism, since
        // we're implementing it ourselves.

        // A Solidity high level call has three parts:
        //  1. The target address is checked to verify it contains contract code
        //  2. The call itself is made, and success asserted
        //  3. The return value is decoded, which in turn checks the size of the returned data.
        // solhint-disable-next-line max-line-length
        require(Utils.isContract(address(token)), "SafeERC20: call to non-contract");

        // solhint-disable-next-line avoid-low-level-calls
        (bool success, bytes memory returndata) = address(token).call(data);
        require(success, "SafeERC20: low-level call failed");

        if (returndata.length > 0) { // Return data is optional
            // solhint-disable-next-line max-line-length
            require(abi.decode(returndata, (bool)), "SafeERC20: ERC20 operation did not succeed");
        }
    }
}

// File: contracts\libs\common\ZeroCopySource.sol

pragma solidity ^0.5.0;

/**
 * @dev Wrappers over decoding and deserialization operation from bytes into bassic types in Solidity for PolyNetwork cross chain utility.
 *
 * Decode into basic types in Solidity from bytes easily. It's designed to be used 
 * for PolyNetwork cross chain application, and the decoding rules on Ethereum chain 
 * and the encoding rule on other chains should be consistent, and . Here we
 * follow the underlying deserialization rule with implementation found here: 
 * https://github.com/polynetwork/poly/blob/master/common/zero_copy_source.go
 *
 * Using this library instead of the unchecked serialization method can help reduce
 * the risk of serious bugs and handfule, so it's recommended to use it.
 *
 * Please note that risk can be minimized, yet not eliminated.
 */
library ZeroCopySource {
    /* @notice              Read next byte as boolean type starting at offset from buff
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the boolean value
    *  @return              The the read boolean value and new offset
    */
    function NextBool(bytes memory buff, uint256 offset) internal pure returns(bool, uint256) {
        require(offset + 1 <= buff.length && offset < offset + 1, "Offset exceeds limit");
        // byte === bytes1
        byte v;
        assembly{
            v := mload(add(add(buff, 0x20), offset))
        }
        bool value;
        if (v == 0x01) {
		    value = true;
    	} else if (v == 0x00) {
            value = false;
        } else {
            revert("NextBool value error");
        }
        return (value, offset + 1);
    }

    /* @notice              Read next byte starting at offset from buff
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the byte value
    *  @return              The read byte value and new offset
    */
    function NextByte(bytes memory buff, uint256 offset) internal pure returns (byte, uint256) {
        require(offset + 1 <= buff.length && offset < offset + 1, "NextByte, Offset exceeds maximum");
        byte v;
        assembly{
            v := mload(add(add(buff, 0x20), offset))
        }
        return (v, offset + 1);
    }

    /* @notice              Read next byte as uint8 starting at offset from buff
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the byte value
    *  @return              The read uint8 value and new offset
    */
    function NextUint8(bytes memory buff, uint256 offset) internal pure returns (uint8, uint256) {
        require(offset + 1 <= buff.length && offset < offset + 1, "NextUint8, Offset exceeds maximum");
        uint8 v;
        assembly{
            let tmpbytes := mload(0x40)
            let bvalue := mload(add(add(buff, 0x20), offset))
            mstore8(tmpbytes, byte(0, bvalue))
            mstore(0x40, add(tmpbytes, 0x01))
            v := mload(sub(tmpbytes, 0x1f))
        }
        return (v, offset + 1);
    }

    /* @notice              Read next two bytes as uint16 type starting from offset
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the uint16 value
    *  @return              The read uint16 value and updated offset
    */
    function NextUint16(bytes memory buff, uint256 offset) internal pure returns (uint16, uint256) {
        require(offset + 2 <= buff.length && offset < offset + 2, "NextUint16, offset exceeds maximum");
        
        uint16 v;
        assembly {
            let tmpbytes := mload(0x40)
            let bvalue := mload(add(add(buff, 0x20), offset))
            mstore8(tmpbytes, byte(0x01, bvalue))
            mstore8(add(tmpbytes, 0x01), byte(0, bvalue))
            mstore(0x40, add(tmpbytes, 0x02))
            v := mload(sub(tmpbytes, 0x1e))
        }
        return (v, offset + 2);
    }


    /* @notice              Read next four bytes as uint32 type starting from offset
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the uint32 value
    *  @return              The read uint32 value and updated offset
    */
    function NextUint32(bytes memory buff, uint256 offset) internal pure returns (uint32, uint256) {
        require(offset + 4 <= buff.length && offset < offset + 4, "NextUint32, offset exceeds maximum");
        uint32 v;
        assembly {
            let tmpbytes := mload(0x40)
            let byteLen := 0x04
            for {
                let tindex := 0x00
                let bindex := sub(byteLen, 0x01)
                let bvalue := mload(add(add(buff, 0x20), offset))
            } lt(tindex, byteLen) {
                tindex := add(tindex, 0x01)
                bindex := sub(bindex, 0x01)
            }{
                mstore8(add(tmpbytes, tindex), byte(bindex, bvalue))
            }
            mstore(0x40, add(tmpbytes, byteLen))
            v := mload(sub(tmpbytes, sub(0x20, byteLen)))
        }
        return (v, offset + 4);
    }

    /* @notice              Read next eight bytes as uint64 type starting from offset
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the uint64 value
    *  @return              The read uint64 value and updated offset
    */
    function NextUint64(bytes memory buff, uint256 offset) internal pure returns (uint64, uint256) {
        require(offset + 8 <= buff.length && offset < offset + 8, "NextUint64, offset exceeds maximum");
        uint64 v;
        assembly {
            let tmpbytes := mload(0x40)
            let byteLen := 0x08
            for {
                let tindex := 0x00
                let bindex := sub(byteLen, 0x01)
                let bvalue := mload(add(add(buff, 0x20), offset))
            } lt(tindex, byteLen) {
                tindex := add(tindex, 0x01)
                bindex := sub(bindex, 0x01)
            }{
                mstore8(add(tmpbytes, tindex), byte(bindex, bvalue))
            }
            mstore(0x40, add(tmpbytes, byteLen))
            v := mload(sub(tmpbytes, sub(0x20, byteLen)))
        }
        return (v, offset + 8);
    }

    /* @notice              Read next 32 bytes as uint256 type starting from offset,
                            there are limits considering the numerical limits in multi-chain
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the uint256 value
    *  @return              The read uint256 value and updated offset
    */
    function NextUint255(bytes memory buff, uint256 offset) internal pure returns (uint256, uint256) {
        require(offset + 32 <= buff.length && offset < offset + 32, "NextUint255, offset exceeds maximum");
        uint256 v;
        assembly {
            let tmpbytes := mload(0x40)
            let byteLen := 0x20
            for {
                let tindex := 0x00
                let bindex := sub(byteLen, 0x01)
                let bvalue := mload(add(add(buff, 0x20), offset))
            } lt(tindex, byteLen) {
                tindex := add(tindex, 0x01)
                bindex := sub(bindex, 0x01)
            }{
                mstore8(add(tmpbytes, tindex), byte(bindex, bvalue))
            }
            mstore(0x40, add(tmpbytes, byteLen))
            v := mload(tmpbytes)
        }
        require(v <= 0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, "Value exceeds the range");
        return (v, offset + 32);
    }
    /* @notice              Read next variable bytes starting from offset,
                            the decoding rule coming from multi-chain
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the bytes value
    *  @return              The read variable bytes array value and updated offset
    */
    function NextVarBytes(bytes memory buff, uint256 offset) internal pure returns(bytes memory, uint256) {
        uint len;
        (len, offset) = NextVarUint(buff, offset);
        require(offset + len <= buff.length && offset < offset + len, "NextVarBytes, offset exceeds maximum");
        bytes memory tempBytes;
        assembly{
            switch iszero(len)
            case 0 {
                // Get a location of some free memory and store it in tempBytes as
                // Solidity does for memory variables.
                tempBytes := mload(0x40)

                // The first word of the slice result is potentially a partial
                // word read from the original array. To read it, we calculate
                // the length of that partial word and start copying that many
                // bytes into the array. The first word we copy will start with
                // data we don't care about, but the last `lengthmod` bytes will
                // land at the beginning of the contents of the new array. When
                // we're done copying, we overwrite the full first word with
                // the actual length of the slice.
                let lengthmod := and(len, 31)

                // The multiplication in the next line is necessary
                // because when slicing multiples of 32 bytes (lengthmod == 0)
                // the following copy loop was copying the origin's length
                // and then ending prematurely not copying everything it should.
                let mc := add(add(tempBytes, lengthmod), mul(0x20, iszero(lengthmod)))
                let end := add(mc, len)

                for {
                    // The multiplication in the next line has the same exact purpose
                    // as the one above.
                    let cc := add(add(add(buff, lengthmod), mul(0x20, iszero(lengthmod))), offset)
                } lt(mc, end) {
                    mc := add(mc, 0x20)
                    cc := add(cc, 0x20)
                } {
                    mstore(mc, mload(cc))
                }

                mstore(tempBytes, len)

                //update free-memory pointer
                //allocating the array padded to 32 bytes like the compiler does now
                mstore(0x40, and(add(mc, 31), not(31)))
            }
            //if we want a zero-length slice let's just return a zero-length array
            default {
                tempBytes := mload(0x40)

                mstore(0x40, add(tempBytes, 0x20))
            }
        }

        return (tempBytes, offset + len);
    }
    /* @notice              Read next 32 bytes starting from offset,
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the bytes value
    *  @return              The read bytes32 value and updated offset
    */
    function NextHash(bytes memory buff, uint256 offset) internal pure returns (bytes32 , uint256) {
        require(offset + 32 <= buff.length && offset < offset + 32, "NextHash, offset exceeds maximum");
        bytes32 v;
        assembly {
            v := mload(add(buff, add(offset, 0x20)))
        }
        return (v, offset + 32);
    }

    /* @notice              Read next 20 bytes starting from offset,
    *  @param buff          Source bytes array
    *  @param offset        The position from where we read the bytes value
    *  @return              The read bytes20 value and updated offset
    */
    function NextBytes20(bytes memory buff, uint256 offset) internal pure returns (bytes20 , uint256) {
        require(offset + 20 <= buff.length && offset < offset + 20, "NextBytes20, offset exceeds maximum");
        bytes20 v;
        assembly {
            v := mload(add(buff, add(offset, 0x20)))
        }
        return (v, offset + 20);
    }
    
    function NextVarUint(bytes memory buff, uint256 offset) internal pure returns(uint, uint256) {
        byte v;
        (v, offset) = NextByte(buff, offset);

        uint value;
        if (v == 0xFD) {
            // return NextUint16(buff, offset);
            (value, offset) = NextUint16(buff, offset);
            require(value >= 0xFD && value <= 0xFFFF, "NextUint16, value outside range");
            return (value, offset);
        } else if (v == 0xFE) {
            // return NextUint32(buff, offset);
            (value, offset) = NextUint32(buff, offset);
            require(value > 0xFFFF && value <= 0xFFFFFFFF, "NextVarUint, value outside range");
            return (value, offset);
        } else if (v == 0xFF) {
            // return NextUint64(buff, offset);
            (value, offset) = NextUint64(buff, offset);
            require(value > 0xFFFFFFFF, "NextVarUint, value outside range");
            return (value, offset);
        } else{
            // return (uint8(v), offset);
            value = uint8(v);
            require(value < 0xFD, "NextVarUint, value outside range");
            return (value, offset);
        }
    }
}

// File: contracts\libs\common\ZeroCopySink.sol

pragma solidity ^0.5.0;

/**
 * @dev Wrappers over encoding and serialization operation into bytes from bassic types in Solidity for PolyNetwork cross chain utility.
 *
 * Encode basic types in Solidity into bytes easily. It's designed to be used 
 * for PolyNetwork cross chain application, and the encoding rules on Ethereum chain 
 * and the decoding rules on other chains should be consistent. Here we  
 * follow the underlying serialization rule with implementation found here: 
 * https://github.com/polynetwork/poly/blob/master/common/zero_copy_sink.go
 *
 * Using this library instead of the unchecked serialization method can help reduce
 * the risk of serious bugs and handfule, so it's recommended to use it.
 *
 * Please note that risk can be minimized, yet not eliminated.
 */
library ZeroCopySink {
    /* @notice          Convert boolean value into bytes
    *  @param b         The boolean value
    *  @return          Converted bytes array
    */
    function WriteBool(bool b) internal pure returns (bytes memory) {
        bytes memory buff;
        assembly{
            buff := mload(0x40)
            mstore(buff, 1)
            switch iszero(b)
            case 1 {
                mstore(add(buff, 0x20), shl(248, 0x00))
                // mstore8(add(buff, 0x20), 0x00)
            }
            default {
                mstore(add(buff, 0x20), shl(248, 0x01))
                // mstore8(add(buff, 0x20), 0x01)
            }
            mstore(0x40, add(buff, 0x21))
        }
        return buff;
    }

    /* @notice          Convert byte value into bytes
    *  @param b         The byte value
    *  @return          Converted bytes array
    */
    function WriteByte(byte b) internal pure returns (bytes memory) {
        return WriteUint8(uint8(b));
    }

    /* @notice          Convert uint8 value into bytes
    *  @param v         The uint8 value
    *  @return          Converted bytes array
    */
    function WriteUint8(uint8 v) internal pure returns (bytes memory) {
        bytes memory buff;
        assembly{
            buff := mload(0x40)
            mstore(buff, 1)
            mstore(add(buff, 0x20), shl(248, v))
            // mstore(add(buff, 0x20), byte(0x1f, v))
            mstore(0x40, add(buff, 0x21))
        }
        return buff;
    }

    /* @notice          Convert uint16 value into bytes
    *  @param v         The uint16 value
    *  @return          Converted bytes array
    */
    function WriteUint16(uint16 v) internal pure returns (bytes memory) {
        bytes memory buff;

        assembly{
            buff := mload(0x40)
            let byteLen := 0x02
            mstore(buff, byteLen)
            for {
                let mindex := 0x00
                let vindex := 0x1f
            } lt(mindex, byteLen) {
                mindex := add(mindex, 0x01)
                vindex := sub(vindex, 0x01)
            }{
                mstore8(add(add(buff, 0x20), mindex), byte(vindex, v))
            }
            mstore(0x40, add(buff, 0x22))
        }
        return buff;
    }
    
    /* @notice          Convert uint32 value into bytes
    *  @param v         The uint32 value
    *  @return          Converted bytes array
    */
    function WriteUint32(uint32 v) internal pure returns(bytes memory) {
        bytes memory buff;
        assembly{
            buff := mload(0x40)
            let byteLen := 0x04
            mstore(buff, byteLen)
            for {
                let mindex := 0x00
                let vindex := 0x1f
            } lt(mindex, byteLen) {
                mindex := add(mindex, 0x01)
                vindex := sub(vindex, 0x01)
            }{
                mstore8(add(add(buff, 0x20), mindex), byte(vindex, v))
            }
            mstore(0x40, add(buff, 0x24))
        }
        return buff;
    }

    /* @notice          Convert uint64 value into bytes
    *  @param v         The uint64 value
    *  @return          Converted bytes array
    */
    function WriteUint64(uint64 v) internal pure returns(bytes memory) {
        bytes memory buff;

        assembly{
            buff := mload(0x40)
            let byteLen := 0x08
            mstore(buff, byteLen)
            for {
                let mindex := 0x00
                let vindex := 0x1f
            } lt(mindex, byteLen) {
                mindex := add(mindex, 0x01)
                vindex := sub(vindex, 0x01)
            }{
                mstore8(add(add(buff, 0x20), mindex), byte(vindex, v))
            }
            mstore(0x40, add(buff, 0x28))
        }
        return buff;
    }

    /* @notice          Convert limited uint256 value into bytes
    *  @param v         The uint256 value
    *  @return          Converted bytes array
    */
    function WriteUint255(uint256 v) internal pure returns (bytes memory) {
        require(v <= 0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, "Value exceeds uint255 range");
        bytes memory buff;

        assembly{
            buff := mload(0x40)
            let byteLen := 0x20
            mstore(buff, byteLen)
            for {
                let mindex := 0x00
                let vindex := 0x1f
            } lt(mindex, byteLen) {
                mindex := add(mindex, 0x01)
                vindex := sub(vindex, 0x01)
            }{
                mstore8(add(add(buff, 0x20), mindex), byte(vindex, v))
            }
            mstore(0x40, add(buff, 0x40))
        }
        return buff;
    }

    /* @notice          Encode bytes format data into bytes
    *  @param data      The bytes array data
    *  @return          Encoded bytes array
    */
    function WriteVarBytes(bytes memory data) internal pure returns (bytes memory) {
        uint64 l = uint64(data.length);
        return abi.encodePacked(WriteVarUint(l), data);
    }

    function WriteVarUint(uint64 v) internal pure returns (bytes memory) {
        if (v < 0xFD){
    		return WriteUint8(uint8(v));
    	} else if (v <= 0xFFFF) {
    		return abi.encodePacked(WriteByte(0xFD), WriteUint16(uint16(v)));
    	} else if (v <= 0xFFFFFFFF) {
            return abi.encodePacked(WriteByte(0xFE), WriteUint32(uint32(v)));
    	} else {
    		return abi.encodePacked(WriteByte(0xFF), WriteUint64(uint64(v)));
    	}
    }
}

// File: contracts\core\assets\erc20_template\IMerkleTreeHistory.sol

interface IMerkleTreeHistory {
  /**
    @dev Hash 2 tree leaves, returns MiMC(_left, _right)
  */
  // function hashLeftRight(bytes32 _left, bytes32 _right) internal pure returns (bytes32);

  function _insertLeaf(bytes32 _leaf) external returns(bytes32[4] memory arr, bool[2] memory directionSelector);
  // function _insertLeaf(bytes32 _leaf) external returns(uint32 index, bool[2] memory directionSelector, bytes32 PathDigest0, bytes32 PathDigest1, bytes32 rootDigest);
  // function _insertLeaf(bytes32 _leaf) external returns(/*uint32 index,*/ bool[2] memory directionSelector, bytes32[4] memory digest);

  /**
    @dev Whether the root is present in the root history
  */
  function isKnownRoot(bytes32 _root) external view returns(bool);
  /**
    @dev Returns the last root
  */
  function getLastRoot() external view returns(bytes32);
}

// File: contracts\core\cross_chain_manager\interface\IEthCrossChainManagerProxy.sol

pragma solidity ^0.5.0;

/**
 * @dev Interface of the EthCrossChainManagerProxy for business contract like LockProxy to obtain the reliable EthCrossChainManager contract hash.
 */
interface IEthCrossChainManagerProxy {
    function getEthCrossChainManager() external view returns (address);
}

// File: contracts\core\cross_chain_manager\interface\IEthCrossChainManager.sol

pragma solidity ^0.5.0;

/**
 * @dev Interface of the EthCrossChainManager contract for business contract like LockProxy to request cross chain transaction
 */
interface IEthCrossChainManager {
    function crossChain(uint64 _toChainId, bytes calldata _toContract, bytes calldata _method, bytes calldata _txData) external returns (bool);
}

// File: contracts\verifier\verifier.sol

// SPDX-License-Identifier: LGPL-3.0-only
// This file is LGPL3 Licensed
pragma solidity ^0.5.0;

/**
 * @title Elliptic curve operations on twist points for alt_bn128
 * @author Mustafa Al-Bassam (mus@musalbas.com)
 * @dev Homepage: https://github.com/musalbas/solidity-BN256G2
 */

library BN256G2 {
    uint256 internal constant FIELD_MODULUS = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47;
    uint256 internal constant TWISTBX = 0x2b149d40ceb8aaae81be18991be06ac3b5b4c5e559dbefa33267e6dc24a138e5;
    uint256 internal constant TWISTBY = 0x9713b03af0fed4cd2cafadeed8fdf4a74fa084e52d1852e4a2bd0685c315d2;
    uint internal constant PTXX = 0;
    uint internal constant PTXY = 1;
    uint internal constant PTYX = 2;
    uint internal constant PTYY = 3;
    uint internal constant PTZX = 4;
    uint internal constant PTZY = 5;

    /**
     * @notice Add two twist points
     * @param pt1xx Coefficient 1 of x on point 1
     * @param pt1xy Coefficient 2 of x on point 1
     * @param pt1yx Coefficient 1 of y on point 1
     * @param pt1yy Coefficient 2 of y on point 1
     * @param pt2xx Coefficient 1 of x on point 2
     * @param pt2xy Coefficient 2 of x on point 2
     * @param pt2yx Coefficient 1 of y on point 2
     * @param pt2yy Coefficient 2 of y on point 2
     * @return (pt3xx, pt3xy, pt3yx, pt3yy)
     */
    function ECTwistAdd(
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy,
        uint256 pt2xx, uint256 pt2xy,
        uint256 pt2yx, uint256 pt2yy
    ) public view returns (
        uint256, uint256,
        uint256, uint256
    ) {
        if (
            pt1xx == 0 && pt1xy == 0 &&
            pt1yx == 0 && pt1yy == 0
        ) {
            if (!(
                pt2xx == 0 && pt2xy == 0 &&
                pt2yx == 0 && pt2yy == 0
            )) {
                assert(_isOnCurve(
                    pt2xx, pt2xy,
                    pt2yx, pt2yy
                ));
            }
            return (
                pt2xx, pt2xy,
                pt2yx, pt2yy
            );
        } else if (
            pt2xx == 0 && pt2xy == 0 &&
            pt2yx == 0 && pt2yy == 0
        ) {
            assert(_isOnCurve(
                pt1xx, pt1xy,
                pt1yx, pt1yy
            ));
            return (
                pt1xx, pt1xy,
                pt1yx, pt1yy
            );
        }

        assert(_isOnCurve(
            pt1xx, pt1xy,
            pt1yx, pt1yy
        ));
        assert(_isOnCurve(
            pt2xx, pt2xy,
            pt2yx, pt2yy
        ));

        uint256[6] memory pt3 = _ECTwistAddJacobian(
            pt1xx, pt1xy,
            pt1yx, pt1yy,
            1,     0,
            pt2xx, pt2xy,
            pt2yx, pt2yy,
            1,     0
        );

        return _fromJacobian(
            pt3[PTXX], pt3[PTXY],
            pt3[PTYX], pt3[PTYY],
            pt3[PTZX], pt3[PTZY]
        );
    }

    /**
     * @notice Multiply a twist point by a scalar
     * @param s     Scalar to multiply by
     * @param pt1xx Coefficient 1 of x
     * @param pt1xy Coefficient 2 of x
     * @param pt1yx Coefficient 1 of y
     * @param pt1yy Coefficient 2 of y
     * @return (pt2xx, pt2xy, pt2yx, pt2yy)
     */
    function ECTwistMul(
        uint256 s,
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy
    ) public view returns (
        uint256, uint256,
        uint256, uint256
    ) {
        uint256 pt1zx = 1;
        if (
            pt1xx == 0 && pt1xy == 0 &&
            pt1yx == 0 && pt1yy == 0
        ) {
            pt1xx = 1;
            pt1yx = 1;
            pt1zx = 0;
        } else {
            assert(_isOnCurve(
                pt1xx, pt1xy,
                pt1yx, pt1yy
            ));
        }

        uint256[6] memory pt2 = _ECTwistMulJacobian(
            s,
            pt1xx, pt1xy,
            pt1yx, pt1yy,
            pt1zx, 0
        );

        return _fromJacobian(
            pt2[PTXX], pt2[PTXY],
            pt2[PTYX], pt2[PTYY],
            pt2[PTZX], pt2[PTZY]
        );
    }

    /**
     * @notice Get the field modulus
     * @return The field modulus
     */
    function GetFieldModulus() public pure returns (uint256) {
        return FIELD_MODULUS;
    }

    function submod(uint256 a, uint256 b, uint256 n) internal pure returns (uint256) {
        return addmod(a, n - b, n);
    }

    function _FQ2Mul(
        uint256 xx, uint256 xy,
        uint256 yx, uint256 yy
    ) internal pure returns (uint256, uint256) {
        return (
            submod(mulmod(xx, yx, FIELD_MODULUS), mulmod(xy, yy, FIELD_MODULUS), FIELD_MODULUS),
            addmod(mulmod(xx, yy, FIELD_MODULUS), mulmod(xy, yx, FIELD_MODULUS), FIELD_MODULUS)
        );
    }

    function _FQ2Muc(
        uint256 xx, uint256 xy,
        uint256 c
    ) internal pure returns (uint256, uint256) {
        return (
            mulmod(xx, c, FIELD_MODULUS),
            mulmod(xy, c, FIELD_MODULUS)
        );
    }

    function _FQ2Add(
        uint256 xx, uint256 xy,
        uint256 yx, uint256 yy
    ) internal pure returns (uint256, uint256) {
        return (
            addmod(xx, yx, FIELD_MODULUS),
            addmod(xy, yy, FIELD_MODULUS)
        );
    }

    function _FQ2Sub(
        uint256 xx, uint256 xy,
        uint256 yx, uint256 yy
    ) internal pure returns (uint256 rx, uint256 ry) {
        return (
            submod(xx, yx, FIELD_MODULUS),
            submod(xy, yy, FIELD_MODULUS)
        );
    }

    function _FQ2Div(
        uint256 xx, uint256 xy,
        uint256 yx, uint256 yy
    ) internal view returns (uint256, uint256) {
        (yx, yy) = _FQ2Inv(yx, yy);
        return _FQ2Mul(xx, xy, yx, yy);
    }

    function _FQ2Inv(uint256 x, uint256 y) internal view returns (uint256, uint256) {
        uint256 inv = _modInv(addmod(mulmod(y, y, FIELD_MODULUS), mulmod(x, x, FIELD_MODULUS), FIELD_MODULUS), FIELD_MODULUS);
        return (
            mulmod(x, inv, FIELD_MODULUS),
            FIELD_MODULUS - mulmod(y, inv, FIELD_MODULUS)
        );
    }

    function _isOnCurve(
        uint256 xx, uint256 xy,
        uint256 yx, uint256 yy
    ) internal pure returns (bool) {
        uint256 yyx;
        uint256 yyy;
        uint256 xxxx;
        uint256 xxxy;
        (yyx, yyy) = _FQ2Mul(yx, yy, yx, yy);
        (xxxx, xxxy) = _FQ2Mul(xx, xy, xx, xy);
        (xxxx, xxxy) = _FQ2Mul(xxxx, xxxy, xx, xy);
        (yyx, yyy) = _FQ2Sub(yyx, yyy, xxxx, xxxy);
        (yyx, yyy) = _FQ2Sub(yyx, yyy, TWISTBX, TWISTBY);
        return yyx == 0 && yyy == 0;
    }

    function _modInv(uint256 a, uint256 n) internal view returns (uint256 result) {
        bool success;
        assembly {
            let freemem := mload(0x40)
            mstore(freemem, 0x20)
            mstore(add(freemem,0x20), 0x20)
            mstore(add(freemem,0x40), 0x20)
            mstore(add(freemem,0x60), a)
            mstore(add(freemem,0x80), sub(n, 2))
            mstore(add(freemem,0xA0), n)
            success := staticcall(sub(gas(), 2000), 5, freemem, 0xC0, freemem, 0x20)
            result := mload(freemem)
        }
        require(success);
    }

    function _fromJacobian(
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy,
        uint256 pt1zx, uint256 pt1zy
    ) internal view returns (
        uint256 pt2xx, uint256 pt2xy,
        uint256 pt2yx, uint256 pt2yy
    ) {
        uint256 invzx;
        uint256 invzy;
        (invzx, invzy) = _FQ2Inv(pt1zx, pt1zy);
        (pt2xx, pt2xy) = _FQ2Mul(pt1xx, pt1xy, invzx, invzy);
        (pt2yx, pt2yy) = _FQ2Mul(pt1yx, pt1yy, invzx, invzy);
    }

    function _ECTwistAddJacobian(
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy,
        uint256 pt1zx, uint256 pt1zy,
        uint256 pt2xx, uint256 pt2xy,
        uint256 pt2yx, uint256 pt2yy,
        uint256 pt2zx, uint256 pt2zy) internal pure returns (uint256[6] memory pt3) {
            if (pt1zx == 0 && pt1zy == 0) {
                (
                    pt3[PTXX], pt3[PTXY],
                    pt3[PTYX], pt3[PTYY],
                    pt3[PTZX], pt3[PTZY]
                ) = (
                    pt2xx, pt2xy,
                    pt2yx, pt2yy,
                    pt2zx, pt2zy
                );
                return pt3;
            } else if (pt2zx == 0 && pt2zy == 0) {
                (
                    pt3[PTXX], pt3[PTXY],
                    pt3[PTYX], pt3[PTYY],
                    pt3[PTZX], pt3[PTZY]
                ) = (
                    pt1xx, pt1xy,
                    pt1yx, pt1yy,
                    pt1zx, pt1zy
                );
                return pt3;
            }

            (pt2yx,     pt2yy)     = _FQ2Mul(pt2yx, pt2yy, pt1zx, pt1zy); // U1 = y2 * z1
            (pt3[PTYX], pt3[PTYY]) = _FQ2Mul(pt1yx, pt1yy, pt2zx, pt2zy); // U2 = y1 * z2
            (pt2xx,     pt2xy)     = _FQ2Mul(pt2xx, pt2xy, pt1zx, pt1zy); // V1 = x2 * z1
            (pt3[PTZX], pt3[PTZY]) = _FQ2Mul(pt1xx, pt1xy, pt2zx, pt2zy); // V2 = x1 * z2

            if (pt2xx == pt3[PTZX] && pt2xy == pt3[PTZY]) {
                if (pt2yx == pt3[PTYX] && pt2yy == pt3[PTYY]) {
                    (
                        pt3[PTXX], pt3[PTXY],
                        pt3[PTYX], pt3[PTYY],
                        pt3[PTZX], pt3[PTZY]
                    ) = _ECTwistDoubleJacobian(pt1xx, pt1xy, pt1yx, pt1yy, pt1zx, pt1zy);
                    return pt3;
                }
                (
                    pt3[PTXX], pt3[PTXY],
                    pt3[PTYX], pt3[PTYY],
                    pt3[PTZX], pt3[PTZY]
                ) = (
                    1, 0,
                    1, 0,
                    0, 0
                );
                return pt3;
            }

            (pt2zx,     pt2zy)     = _FQ2Mul(pt1zx, pt1zy, pt2zx,     pt2zy);     // W = z1 * z2
            (pt1xx,     pt1xy)     = _FQ2Sub(pt2yx, pt2yy, pt3[PTYX], pt3[PTYY]); // U = U1 - U2
            (pt1yx,     pt1yy)     = _FQ2Sub(pt2xx, pt2xy, pt3[PTZX], pt3[PTZY]); // V = V1 - V2
            (pt1zx,     pt1zy)     = _FQ2Mul(pt1yx, pt1yy, pt1yx,     pt1yy);     // V_squared = V * V
            (pt2yx,     pt2yy)     = _FQ2Mul(pt1zx, pt1zy, pt3[PTZX], pt3[PTZY]); // V_squared_times_V2 = V_squared * V2
            (pt1zx,     pt1zy)     = _FQ2Mul(pt1zx, pt1zy, pt1yx,     pt1yy);     // V_cubed = V * V_squared
            (pt3[PTZX], pt3[PTZY]) = _FQ2Mul(pt1zx, pt1zy, pt2zx,     pt2zy);     // newz = V_cubed * W
            (pt2xx,     pt2xy)     = _FQ2Mul(pt1xx, pt1xy, pt1xx,     pt1xy);     // U * U
            (pt2xx,     pt2xy)     = _FQ2Mul(pt2xx, pt2xy, pt2zx,     pt2zy);     // U * U * W
            (pt2xx,     pt2xy)     = _FQ2Sub(pt2xx, pt2xy, pt1zx,     pt1zy);     // U * U * W - V_cubed
            (pt2zx,     pt2zy)     = _FQ2Muc(pt2yx, pt2yy, 2);                    // 2 * V_squared_times_V2
            (pt2xx,     pt2xy)     = _FQ2Sub(pt2xx, pt2xy, pt2zx,     pt2zy);     // A = U * U * W - V_cubed - 2 * V_squared_times_V2
            (pt3[PTXX], pt3[PTXY]) = _FQ2Mul(pt1yx, pt1yy, pt2xx,     pt2xy);     // newx = V * A
            (pt1yx,     pt1yy)     = _FQ2Sub(pt2yx, pt2yy, pt2xx,     pt2xy);     // V_squared_times_V2 - A
            (pt1yx,     pt1yy)     = _FQ2Mul(pt1xx, pt1xy, pt1yx,     pt1yy);     // U * (V_squared_times_V2 - A)
            (pt1xx,     pt1xy)     = _FQ2Mul(pt1zx, pt1zy, pt3[PTYX], pt3[PTYY]); // V_cubed * U2
            (pt3[PTYX], pt3[PTYY]) = _FQ2Sub(pt1yx, pt1yy, pt1xx,     pt1xy);     // newy = U * (V_squared_times_V2 - A) - V_cubed * U2
    }

    function _ECTwistDoubleJacobian(
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy,
        uint256 pt1zx, uint256 pt1zy
    ) internal pure returns (
        uint256 pt2xx, uint256 pt2xy,
        uint256 pt2yx, uint256 pt2yy,
        uint256 pt2zx, uint256 pt2zy
    ) {
        (pt2xx, pt2xy) = _FQ2Muc(pt1xx, pt1xy, 3);            // 3 * x
        (pt2xx, pt2xy) = _FQ2Mul(pt2xx, pt2xy, pt1xx, pt1xy); // W = 3 * x * x
        (pt1zx, pt1zy) = _FQ2Mul(pt1yx, pt1yy, pt1zx, pt1zy); // S = y * z
        (pt2yx, pt2yy) = _FQ2Mul(pt1xx, pt1xy, pt1yx, pt1yy); // x * y
        (pt2yx, pt2yy) = _FQ2Mul(pt2yx, pt2yy, pt1zx, pt1zy); // B = x * y * S
        (pt1xx, pt1xy) = _FQ2Mul(pt2xx, pt2xy, pt2xx, pt2xy); // W * W
        (pt2zx, pt2zy) = _FQ2Muc(pt2yx, pt2yy, 8);            // 8 * B
        (pt1xx, pt1xy) = _FQ2Sub(pt1xx, pt1xy, pt2zx, pt2zy); // H = W * W - 8 * B
        (pt2zx, pt2zy) = _FQ2Mul(pt1zx, pt1zy, pt1zx, pt1zy); // S_squared = S * S
        (pt2yx, pt2yy) = _FQ2Muc(pt2yx, pt2yy, 4);            // 4 * B
        (pt2yx, pt2yy) = _FQ2Sub(pt2yx, pt2yy, pt1xx, pt1xy); // 4 * B - H
        (pt2yx, pt2yy) = _FQ2Mul(pt2yx, pt2yy, pt2xx, pt2xy); // W * (4 * B - H)
        (pt2xx, pt2xy) = _FQ2Muc(pt1yx, pt1yy, 8);            // 8 * y
        (pt2xx, pt2xy) = _FQ2Mul(pt2xx, pt2xy, pt1yx, pt1yy); // 8 * y * y
        (pt2xx, pt2xy) = _FQ2Mul(pt2xx, pt2xy, pt2zx, pt2zy); // 8 * y * y * S_squared
        (pt2yx, pt2yy) = _FQ2Sub(pt2yx, pt2yy, pt2xx, pt2xy); // newy = W * (4 * B - H) - 8 * y * y * S_squared
        (pt2xx, pt2xy) = _FQ2Muc(pt1xx, pt1xy, 2);            // 2 * H
        (pt2xx, pt2xy) = _FQ2Mul(pt2xx, pt2xy, pt1zx, pt1zy); // newx = 2 * H * S
        (pt2zx, pt2zy) = _FQ2Mul(pt1zx, pt1zy, pt2zx, pt2zy); // S * S_squared
        (pt2zx, pt2zy) = _FQ2Muc(pt2zx, pt2zy, 8);            // newz = 8 * S * S_squared
    }

    function _ECTwistMulJacobian(
        uint256 d,
        uint256 pt1xx, uint256 pt1xy,
        uint256 pt1yx, uint256 pt1yy,
        uint256 pt1zx, uint256 pt1zy
    ) internal pure returns (uint256[6] memory pt2) {
        while (d != 0) {
            if ((d & 1) != 0) {
                pt2 = _ECTwistAddJacobian(
                    pt2[PTXX], pt2[PTXY],
                    pt2[PTYX], pt2[PTYY],
                    pt2[PTZX], pt2[PTZY],
                    pt1xx, pt1xy,
                    pt1yx, pt1yy,
                    pt1zx, pt1zy);
            }
            (
                pt1xx, pt1xy,
                pt1yx, pt1yy,
                pt1zx, pt1zy
            ) = _ECTwistDoubleJacobian(
                pt1xx, pt1xy,
                pt1yx, pt1yy,
                pt1zx, pt1zy
            );

            d = d / 2;
        }
    }
}
// This file is MIT Licensed.
//
// Copyright 2017 Christian Reitwiessner
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
pragma solidity ^0.5.0;
library Pairing {
    struct G1Point {
        uint X;
        uint Y;
    }
    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint[2] X;
        uint[2] Y;
    }
    /// @return the generator of G1
    function P1() pure internal returns (G1Point memory) {
        return G1Point(1, 2);
    }
    /// @return the generator of G2
    function P2() pure internal returns (G2Point memory) {
        return G2Point(
            [10857046999023057135944570762232829481370756359578518086990519993285655852781,
             11559732032986387107991004021392285783925812861821192530917403151452391805634],
            [8495653923123431417604973247489272438418190587263600148770280649306958101930,
             4082367875863433681332203403145435568316851327593401208105741076214120093531]
        );
    }
    /// @return the negation of p, i.e. p.addition(p.negate()) should be zero.
    function negate(G1Point memory p) pure internal returns (G1Point memory) {
        // The prime q in the base field F_q for G1
        uint q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;
        if (p.X == 0 && p.Y == 0)
            return G1Point(0, 0);
        return G1Point(p.X, q - (p.Y % q));
    }
    /// @return r the sum of two points of G1
    function addition(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
        uint[4] memory input;
        input[0] = p1.X;
        input[1] = p1.Y;
        input[2] = p2.X;
        input[3] = p2.Y;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 6, input, 0xc0, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }
        require(success);
    }
    /// @return r the sum of two points of G2
    function addition(G2Point memory p1, G2Point memory p2) internal view returns (G2Point memory r) {
        (r.X[0], r.X[1], r.Y[0], r.Y[1]) = BN256G2.ECTwistAdd(p1.X[0],p1.X[1],p1.Y[0],p1.Y[1],p2.X[0],p2.X[1],p2.Y[0],p2.Y[1]);
    }
    /// @return r the product of a point on G1 and a scalar, i.e.
    /// p == p.scalar_mul(1) and p.addition(p) == p.scalar_mul(2) for all points p.
    function scalar_mul(G1Point memory p, uint s) internal view returns (G1Point memory r) {
        uint[3] memory input;
        input[0] = p.X;
        input[1] = p.Y;
        input[2] = s;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }
        require (success);
    }
    /// @return the result of computing the pairing check
    /// e(p1[0], p2[0]) *  .... * e(p1[n], p2[n]) == 1
    /// For example pairing([P1(), P1().negate()], [P2(), P2()]) should
    /// return true.
    function pairing(G1Point[] memory p1, G2Point[] memory p2) internal view returns (bool) {
        require(p1.length == p2.length);
        uint elements = p1.length;
        uint inputSize = elements * 6;
        uint[] memory input = new uint[](inputSize);
        for (uint i = 0; i < elements; i++)
        {
            input[i * 6 + 0] = p1[i].X;
            input[i * 6 + 1] = p1[i].Y;
            input[i * 6 + 2] = p2[i].X[1];
            input[i * 6 + 3] = p2[i].X[0];
            input[i * 6 + 4] = p2[i].Y[1];
            input[i * 6 + 5] = p2[i].Y[0];
        }
        uint[1] memory out;
        bool success;
        assembly {
            success := staticcall(sub(gas(), 2000), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
            // Use "invalid" to make gas estimation work
            switch success case 0 { invalid() }
        }
        require(success);
        return out[0] != 0;
    }
    /// Convenience method for a pairing check for two pairs.
    function pairingProd2(G1Point memory a1, G2Point memory a2, G1Point memory b1, G2Point memory b2) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](2);
        G2Point[] memory p2 = new G2Point[](2);
        p1[0] = a1;
        p1[1] = b1;
        p2[0] = a2;
        p2[1] = b2;
        return pairing(p1, p2);
    }
    /// Convenience method for a pairing check for three pairs.
    function pairingProd3(
            G1Point memory a1, G2Point memory a2,
            G1Point memory b1, G2Point memory b2,
            G1Point memory c1, G2Point memory c2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](3);
        G2Point[] memory p2 = new G2Point[](3);
        p1[0] = a1;
        p1[1] = b1;
        p1[2] = c1;
        p2[0] = a2;
        p2[1] = b2;
        p2[2] = c2;
        return pairing(p1, p2);
    }
    /// Convenience method for a pairing check for four pairs.
    function pairingProd4(
            G1Point memory a1, G2Point memory a2,
            G1Point memory b1, G2Point memory b2,
            G1Point memory c1, G2Point memory c2,
            G1Point memory d1, G2Point memory d2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](4);
        G2Point[] memory p2 = new G2Point[](4);
        p1[0] = a1;
        p1[1] = b1;
        p1[2] = c1;
        p1[3] = d1;
        p2[0] = a2;
        p2[1] = b2;
        p2[2] = c2;
        p2[3] = d2;
        return pairing(p1, p2);
    }
}

contract Verifier {
    using Pairing for *;
    struct VerifyingKey {
        Pairing.G1Point alpha;
        Pairing.G2Point beta;
        Pairing.G2Point gamma;
        Pairing.G2Point delta;
        Pairing.G1Point[] gamma_abc;
    }
    struct Proof {
        Pairing.G1Point a;
        Pairing.G2Point b;
        Pairing.G1Point c;
    }
    function verifyingKey() pure internal returns (VerifyingKey memory vk) {
        vk.alpha = Pairing.G1Point(uint256(0x0d13a1752de7732724792af3955df588a7d98c2b3f17914131d63b69f1ddf4de), uint256(0x29baacf764188c30e4ca1264127876247cd84452ec9fcad7aeb7cd229160101a));
        vk.beta = Pairing.G2Point([uint256(0x10dc0c2638a36856c52f8c7d025852c9edcc34e69aa0d105ff9ddc954f627541), uint256(0x159cd8e7f6eb2079ba6ebfd5bcaceb619070ede5a9fbb4c58ab89d566f4c3e42)], [uint256(0x2aa48c6e340dab3d76d2c2b08865262f5884d3acabdee276de16e88a08988a82), uint256(0x22ec5f4b22b2de22424dfe3ea15cc543398df58f50b77ee7fa69b383212ac000)]);
        vk.gamma = Pairing.G2Point([uint256(0x14c69c3193ae3e5d02494505675aa8560a0130b65086270ae55394f457fa15a0), uint256(0x062b0d4100f275acefe897f1e83f8744efe09f3e31cc4899c0e359d9f6f43c86)], [uint256(0x001f24e8e1e9f374fb291f037ed0f4093b4f378e3aaa1cfb896e224c35d92cd4), uint256(0x13217b42b7c7fdca0279d8e54dfda650c71e04b696d9bea047f0f8eb53bc1b39)]);
        vk.delta = Pairing.G2Point([uint256(0x035f59f2ffd7aa1df24d5bfd6d37f53bf9fc49ba56a2ae0dc1f7707bfb3fece4), uint256(0x1229558475deb82a4866671aab3a7011728d56a860316b4e924ece7a927e1fff)], [uint256(0x032aa42739d90180e0c902d2882272f66e6bdfc346de60a86d755655e8f48f0c), uint256(0x225231e237332cc2cdb8f029c952f2f760a6abec6ab5a5fd8bb11985add77b09)]);
        vk.gamma_abc = new Pairing.G1Point[](20);
        vk.gamma_abc[0] = Pairing.G1Point(uint256(0x210e0e92f0e4f0a5ba71402542137fb57a7f26905e5311ac2905152f4098080b), uint256(0x1a7ceb21c456519ccfa6aa67a2b2127685be5c88adbcbb4940d2914fd6250bb0));
        vk.gamma_abc[1] = Pairing.G1Point(uint256(0x11a8a5e63c1a16b79827330f530388b175a00119b3cd1cc6785572ab58a023ff), uint256(0x2405a2def213780e1f545ad4c9c2070cba4c4f8ea1155cde934e98fb22c15110));
        vk.gamma_abc[2] = Pairing.G1Point(uint256(0x2848a78483f29d9019e1035f998794db22c2d4e834c3a6afdb46ae434b95de8b), uint256(0x00282ace3656f42a19da42d96f8a3f878e4555c770dd38282f97a8fabde0010b));
        vk.gamma_abc[3] = Pairing.G1Point(uint256(0x04c344d7ddd090aea1845ff88074c062e1420106bb9f42f60706b28fa92eab28), uint256(0x08163f60f8ad4a686d56a6a98c7efe31da87713418f5ca99030a2a34996554d5));
        vk.gamma_abc[4] = Pairing.G1Point(uint256(0x0fb488fe860e0693fc941f04f3354cf3bd56c02de43f669ccd359c3c0ace2c27), uint256(0x2a37568bc83274219c521b65b9f373139446b37ce008b6cff822cdf0f2aae852));
        vk.gamma_abc[5] = Pairing.G1Point(uint256(0x058f8cbe6fec05346fe731bdef56f94e9915082ad2a2e860e671ca61c97c90c2), uint256(0x0434b714304ef7e9af10c1718dea691504751933aa8b0849595442234499ccd1));
        vk.gamma_abc[6] = Pairing.G1Point(uint256(0x011d718705ac97965dc5b7a2bdf94504500bf67fbe5599cfcc168000477c2f40), uint256(0x2a7e8e5465ea6b99542ce47242983b14e2a829e7f2a0c00844af7009719e8ff5));
        vk.gamma_abc[7] = Pairing.G1Point(uint256(0x282a419b13d806ce38595caff2e04e59529879157d4480faed6c4856a7f02dc6), uint256(0x261b0f0e8354512328eef1486400251829eb6600dee3995328009b8be7408344));
        vk.gamma_abc[8] = Pairing.G1Point(uint256(0x0c0408d9d885050032d902fcd312404f6b4837183079f4b73d09f929483e1c97), uint256(0x1b14c95f4c3dcd4a96bf4703efb78fd57d63ea7038f5f75342574a6a1e00d81d));
        vk.gamma_abc[9] = Pairing.G1Point(uint256(0x2c5cccb9589d9f3504cb4b2fe346d7d90d819d803e996a993193d3dda3c32f59), uint256(0x0e24e745a75b834820261e5cce0978853c35d47b8710e59b0cec25954de3a200));
        vk.gamma_abc[10] = Pairing.G1Point(uint256(0x2813d3ef10887f6f049fb799ba8e3780a1f198e00ea39d8d2b7d1d3822da0cae), uint256(0x26986a6a0a06baf01b97e1f3b95533babca2ae278be6b8566333d7e503cbc22e));
        vk.gamma_abc[11] = Pairing.G1Point(uint256(0x222c857c9d52cef62663c120876274b2f5af015068a1df090699f171f6e525dc), uint256(0x01239d7e0ba7a3de543e1678c9d07b39bc8ccbd56b3e0edc997dba7e3d221791));
        vk.gamma_abc[12] = Pairing.G1Point(uint256(0x1ac85407ddc6b93cb9d5de3a0b6bd7b7bdf46adca690ca95dfd288c049ae8609), uint256(0x01b62017c5636d8b6c999b7a945478d9668e7458d976a4e6d2350f44633b0bd3));
        vk.gamma_abc[13] = Pairing.G1Point(uint256(0x04e151aec7c97f35655f77dbd52aea7f2658e014ebe949d61d1158c2e82be59d), uint256(0x1671319b686056d95424726743458543037d6959fda179dae07418c9c344ce51));
        vk.gamma_abc[14] = Pairing.G1Point(uint256(0x11ae54effce744b31d56590a0eebf3135c4fb9ba0e85eef37274acf8b92187e0), uint256(0x0e92f38b0a9af756b21d4aeec43358c54bdd31651448e16345c4120ccf0fb3e2));
        vk.gamma_abc[15] = Pairing.G1Point(uint256(0x1b2857484b3b99e9102dcc6fe1a07227b055586515bff18f2b37083bec1551a5), uint256(0x2e2a6312fd06f2c30740d9d2947c001074706e83f904e01a90975a4b55beef6e));
        vk.gamma_abc[16] = Pairing.G1Point(uint256(0x2987d5e3cbdc50215b45718a0176f5c8831010fd3f1a8e8ecf00a5b1aaa2d751), uint256(0x1e4e78f904635750ab72939a669fa5f0978bf4cac6157e0b0f5f18201970ecf5));
        vk.gamma_abc[17] = Pairing.G1Point(uint256(0x301802347beb6861f1f166d39733bbf351b51b38fb40f8297097794c5818cd00), uint256(0x3002e0e1ce21825b41750579c12f3429d78a253785014e5064ae02547acd7c95));
        vk.gamma_abc[18] = Pairing.G1Point(uint256(0x25622f0dd0b6070fb82f3a71cc111740ffe8406f278a32c25997da968070e20a), uint256(0x1a7402071bdffe0c848b79425d2ade64d78cd6461e1eb35097023767f682c078));
        vk.gamma_abc[19] = Pairing.G1Point(uint256(0x0011b488bb0018f0b404064e644d081b7b8862bf126933ad741f2c32b142daca), uint256(0x2a1970c5a038a4e5469b499e5369bf577a3af10fc0b8db012a9c9086dbda54c1));
    }
    function verify(uint[] memory input, Proof memory proof) internal view returns (uint) {
        uint256 snark_scalar_field = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        VerifyingKey memory vk = verifyingKey();
        require(input.length + 1 == vk.gamma_abc.length);
        // Compute the linear combination vk_x
        Pairing.G1Point memory vk_x = Pairing.G1Point(0, 0);
        for (uint i = 0; i < input.length; i++) {
            require(input[i] < snark_scalar_field);
            vk_x = Pairing.addition(vk_x, Pairing.scalar_mul(vk.gamma_abc[i + 1], input[i]));
        }
        vk_x = Pairing.addition(vk_x, vk.gamma_abc[0]);
        if(!Pairing.pairingProd4(
             proof.a, proof.b,
             Pairing.negate(vk_x), vk.gamma,
             Pairing.negate(proof.c), vk.delta,
             Pairing.negate(vk.alpha), vk.beta)) return 1;
        return 0;
    }
    function verifyTx(
            uint[2] memory a, uint[2][2] memory b, uint[2] memory c, uint[19] memory input
        ) public view returns (bool r) {
        uint[] memory inputValues = new uint[](19);
        Proof memory proof = Proof({a:Pairing.G1Point(a[0],a[1]),b:Pairing.G2Point(b[0],b[1]),c:Pairing.G1Point(c[0],c[1])});
          
        for(uint i = 0; i < input.length; i++){
            inputValues[i] = input[i];
        }
        if (verify(inputValues, proof) == 0) {
            return true;
        } else {
            return false;
        }
    }
}

// File: contracts\core\assets\erc20_template\ERC20ZKPNoteProxy.sol

pragma solidity ^0.5.0;










contract ERC20ZKPNoteProxy is Ownable, Verifier {
    using SafeMath for uint;
    using SafeERC20 for IERC20;

    struct CCNote {
        uint256 amount; 
        uint256 nonce; 
        uint64 ChainId ; 
        address fromAssetHash;
        address fromadderss;
    }

    struct CCNoteReq {
        bytes toAssetHash;
        uint256 amount ; 
        uint256 pk1;  
        uint256 pk2; 
        uint64 toChainId ; 
        bytes32 noteid;
    }

    // should be set to non-public
    uint256 public gnonce;
    uint32 public chainId = 2;  // Default is Ethereum
    address public  merklehistory ; 

    enum State {Invalid, Created, CreatfromTrans, LockforTrans, Transferred, Spent}
    mapping(bytes32 => State) public stikonotereqstate; // mapping of hash of the request to its state
    mapping(bytes32 => CCNoteReq) public stikonotereq; // mapping of hash of the note to request
    mapping(bytes32 => CCNote) public allnotes; // mapping of hash of the note to note details
    bytes32[] public  allnoteids ; 

    //string[] public allNotes;
    //bytes32[] public allHashedNotes;

    // All Notes flowed into chain and their states
    mapping(bytes32 => CCNoteReq) public allInputZKPNotes;
    mapping(bytes32 => State) public allInputZKPNotereqstate; // mapping of hash of the request to its state

    // chainId => Proxy address on target chain 
    mapping(uint64 => bytes) public proxyHashMap;

    // Asset Contract Address => To chainId => Asset Contract Address on target chain
    mapping(address => mapping(uint64 => bytes)) public assetHashMap;

    // event BindZKPProxyEvent(uint64 toChainId, bytes targetProxyHash);
    // event BindZKPAssetEvent(address fromAssetHash, uint64 toChainId, bytes targetProxyHash, uint initialAmount);
    event ZKPInputEvent(bytes32 noteid, bytes32 reqhash, bytes toAssetHash, uint256 amount,bytes32[4] arr, bool[2] directionSelector);
    // event ZKPInputEvent(bytes32 noteid, bytes32 reqhash, bytes toAssetHash, uint256 amount);
    // event ZKPOutputEvent(bytes32 noteid, uint256 length, address fromAssetHash, address fromAddress, uint64 toChainId, bytes toAssetHash);
    event ZKPOutputEvent(bytes32 noteid, bytes toAssetHash, address fromAssetHash, uint64 toChainId, uint256 amount, uint256 pk1, uint256 pk2, string encryptedNote, address fromaddress/*, bytes32[4] arr, bool[2] directionSelector*/);

    //event ZKPOutputEvent(bytes32 noteid, uint256 length, address fromAssetHash, address fromAddress, uint64 toChainId, bytes toAssetHash, 
    //        bool[2] directionSelector, bytes32[4] digest);

    event ZKPNoteSpent(bytes32 reqhash, address recipient, uint256 amount);
    // event ZKPNoteCreated(bytes32 noteId, uint index);

    // event SerializeEvent(bytes args);
    event DeserializeEvent(bytes toAssetHash, uint256 amount, uint256 pk1, uint256 pk2, uint64 toChainId, bytes32 noteid);

    address public managerProxyContract;
    // event SetManagerProxyEvent(address manager);
    function setManagerProxy(address ethCCMProxyAddr) onlyOwner public {
        managerProxyContract = ethCCMProxyAddr;
        // emit SetManagerProxyEvent(managerProxyContract);
    } 

    //event SetChainIdEvent (uint32 chainId);
    function SetChainId(uint32 chainid) onlyOwner public {
        chainId = chainid;
        //emit SetChainIdEvent (chainId);
    }

    // event SetMerkletreeAddressEvent (address merklehistory);
    function SetMerkletreeAddress(address merkle) onlyOwner public {
        merklehistory = merkle; 
        // emit SetMerkletreeAddressEvent (merklehistory);
    }

//    constructor() public MerkleTreeHistory(31) {
    constructor() public {
        chainId = 2; // default to Ethereum
    }

    function bindProxyHash(uint64 toChainId, bytes memory targetProxyHash) onlyOwner public returns (bool) {
        proxyHashMap[toChainId] = targetProxyHash;
        // emit BindZKPProxyEvent(toChainId, targetProxyHash);
        return true;
    }
    
    function bindAssetHash(address fromAssetHash, uint64 toChainId, bytes memory toAssetHash) onlyOwner public returns (bool) {
        assetHashMap[fromAssetHash][toChainId] = toAssetHash;
        // emit BindZKPAssetEvent(fromAssetHash, toChainId, toAssetHash, getBalanceFor(fromAssetHash));
        return true;
    }

    function createNote(address fromAssetHash, uint256 amount) public returns (bytes32 id, uint256 index){
        gnonce ++ ; 
        //bytes32 noteid = sha256(abi.encodePacked(bytes32(amount), gnonce, chainId, fromAssetHash, _msgSender()));
        bytes32 noteid = sha256(abi.encodePacked(bytes32(amount), gnonce, chainId, fromAssetHash));
        CCNote  memory ccnote = CCNote({
            amount: amount, 
            nonce: gnonce, 
            ChainId : chainId ,  
            fromAssetHash : fromAssetHash,
            fromadderss: _msgSender()
        });
        allnotes[noteid] = ccnote ;
        allnoteids.push(noteid);
        // emit ZKPNoteCreated(noteid, allnoteids.length - 1);

        return (noteid, allnoteids.length - 1);
    }
    function depositCCOut(address fromAssetHash, uint64 toChainId, uint256 amount, uint256 pk1, uint256 pk2,string memory encryptedNote) public payable returns (bool) {
        require(amount != 0, "amount cannot be zero!");
        require(IERC20(fromAssetHash).allowance(_msgSender(), address(this)) >= amount,"Insuficient Allowance");
        require(_transferToContract(fromAssetHash, amount), "transfer asset from fromAddress to lock_proxy contract  failed!");
        
        //IERC20 erc20Token = IERC20(fromAssetHash);
        //require(IERC20(fromAssetHash).transferFrom(_msgSender(), address(this), amount), "Lock for Note Failed" );

        createNote(fromAssetHash, amount) ;

        // Get Asset Contract on Target Chain
        bytes memory toAssetHash = assetHashMap[fromAssetHash][toChainId];
        require(toAssetHash.length != 0, "empty illegal toAssetHash");

        /*
        CCNoteReq memory ccnotereq = CCNoteReq({
            toAssetHash: hex"473850cB9b3Ee1225304834DE679521d7f21d52f",  // toAssetHash, // Utils.bytesToAddress(toAssetHash), 
            amount : amount, 
            pk1: pk1,  
            pk2: pk2,    
            toChainId : toChainId,  
            noteid : hex"40" // allnoteids[allnoteids.length - 1]
        });
        */

        CCNoteReq memory ccnotereq = CCNoteReq({
            toAssetHash: toAssetHash, // Utils.bytesToAddress(toAssetHash), 
            amount : amount, 
            pk1: pk1,  
            pk2: pk2,    
            toChainId : toChainId,  
            noteid : allnoteids[allnoteids.length - 1]
        });

        stikonotereq[allnoteids[allnoteids.length - 1]] = ccnotereq ; 
        stikonotereqstate[allnoteids[allnoteids.length - 1]] = State.LockforTrans;

        // bytes32 reqHash = sha256(abi.encodePacked(ccnotereq.pk1,ccnotereq.pk2, bytes32(ccnotereq.amount), ccnotereq.toChainId, ccnotereq.toAssetHash,ccnotereq.noteid));
        // uint32 index = IMerkleTreeHistory(merklehistory)._insertLeaf(reqHash);  // emit MerkleInsertEvent

        bytes memory txData = _serializeTxArgs(ccnotereq);
        // emit SerializeEvent(txData);

        // For test only     
        //ccnotereq = _deserializeTxArgs(txData);
        //emit DeserializeEvent(ccnotereq.toAssetHash,ccnotereq.amount,ccnotereq.pk1,ccnotereq.pk2,ccnotereq.toChainId,ccnotereq.noteid);

        bool bCCret = CCTransfer(txData, toChainId);

/*        IEthCrossChainManagerProxy eccmp = IEthCrossChainManagerProxy(managerProxyContract);
        address eccmAddr = eccmp.getEthCrossChainManager();
        IEthCrossChainManager eccm = IEthCrossChainManager(eccmAddr);
        
        bytes memory toProxyHash = proxyHashMap[toChainId];
        require(toProxyHash.length != 0, "empty illegal toProxyHash");
        require(eccm.crossChain(toChainId, toProxyHash, "depositCCIn", txData), "EthCrossChainManager crossChain executed error!");
*/
        // emit ZKPOutputEvent(allnoteids[allnoteids.length - 1], allnoteids.length - 1, fromAssetHash, _msgSender(), toChainId, toAssetHash);
        emit ZKPOutputEvent(ccnotereq.noteid, ccnotereq.toAssetHash, fromAssetHash, toChainId, amount, pk1, pk2, encryptedNote, _msgSender());

        return bCCret;
    }

    function CCTransfer( bytes memory txData, uint64 toChainId) internal returns (bool){

        IEthCrossChainManagerProxy eccmp = IEthCrossChainManagerProxy(managerProxyContract);
        address eccmAddr = eccmp.getEthCrossChainManager();
        IEthCrossChainManager eccm = IEthCrossChainManager(eccmAddr);
        
        bytes memory toProxyHash = proxyHashMap[toChainId];
        require(toProxyHash.length != 0, "empty illegal toProxyHash");
        require(eccm.crossChain(toChainId, toProxyHash, "depositCCIn", txData), "EthCrossChainManager crossChain executed error!");
        
        return true;
    }

    //function depositCCIn(address toAssetHash, uint256 amount, uint256 pk1, uint256 pk2, uint64 toChainId, bytes32 noteid )  public payable returns (bool) {
    function depositCCIn(bytes memory argsBs, bytes memory fromContractAddr, uint64 fromChainId) public returns (bool) {
 
 /*       CCNoteReq memory ccnotereq = CCNoteReq({
            toAssetHash:Utils.addressToBytes(toAssetHash), 
            amount : amount, 
            pk1: pk1,  
            pk2: pk2,    
            toChainId : toChainId,  
            noteid : noteid
        });
*/
        CCNoteReq memory ccnotereq = _deserializeTxArgs(argsBs);
        emit DeserializeEvent(ccnotereq.toAssetHash,ccnotereq.amount,ccnotereq.pk1,ccnotereq.pk2,ccnotereq.toChainId,ccnotereq.noteid);

        // bytes32 reqHash = sha256(abi.encodePacked(ccnotereq.pk1,ccnotereq.pk2, bytes32(ccnotereq.amount), ccnotereq.toChainId, ccnotereq.toAssetHash));
        bytes32 reqHash = sha256(abi.encodePacked(ccnotereq.pk1,ccnotereq.pk2, bytes32(ccnotereq.amount), ccnotereq.toChainId, ccnotereq.toAssetHash,ccnotereq.noteid));
        (bytes32[4] memory arr, bool[2] memory directionSelector ) = IMerkleTreeHistory(merklehistory)._insertLeaf(reqHash);  // emit MerkleInsertEvent
        
        allInputZKPNotes[reqHash]= ccnotereq ; 
        allInputZKPNotereqstate[reqHash] = State.CreatfromTrans;

        emit ZKPInputEvent(ccnotereq.noteid, reqHash, ccnotereq.toAssetHash, ccnotereq.amount,arr, directionSelector);
        return true;
    }

    event ZKPInput002Event(uint256[19] inputvalues);
    function spendNote(uint[2] calldata a, uint[2][2] calldata b, uint[2] calldata c, uint[5] calldata input, bytes32 notereq, uint256 amount, address recipient) external payable returns (bool){
        require(allInputZKPNotes[notereq].amount != 0,"Note Req Id can not be 0");
        require(allInputZKPNotereqstate[notereq] == State.CreatfromTrans, "Ensure CC Asset is not spent");
        // require(IMerkleTreeHistory(merklehistory).isKnownRoot(bytes32(input[2])),"Unknow Merkle Root");

        uint256[19] memory inputvalues;
        // uint256[8] memory array;

        inputvalues[0]=input[0];
        inputvalues[1]=input[1];
        uint256 remainder = input[2] ;
        for (uint256 i = 0 ;i<8 ; i++ ){
           uint256 divisor = 2**(32*(7-i));
           inputvalues[2+i] = remainder / divisor;
           remainder = remainder % divisor;
           //inputvalues[2+i]= array[i];
        }
        remainder = input[3] ;
        for (uint256 i = 0 ;i<8 ; i++ ){
           uint256 divisor = 2**(32*(7-i));
           inputvalues[10+i] = remainder / divisor;
           remainder = remainder % divisor;
           // inputvalues[10+i]= array[i];
        }
        inputvalues[18] = input[4];
        emit ZKPInput002Event(inputvalues);

        require(IMerkleTreeHistory(merklehistory).isKnownRoot(bytes32(input[2])),"Unknow Merkle Root");
        require(
            verifyTx(a, b, c, inputvalues),
            'Invalid zk proof'
        );
        
        // if (allInputZKPNotereqstate[notereq] == State.Spent) { 
        //     return false;
        // }

        // require( note == sha256(abi.encodePacked(input[0], input[1], bytes32(amount))),"Note Does not exist");
        address toassetaddr = Utils.bytesToAddress(allInputZKPNotes[notereq].toAssetHash);
        IERC20 zkpAsset = IERC20(toassetaddr);

        // todo  require enough quota
        require(zkpAsset.transfer(recipient, amount), "Transfer failed in spendNote() in ERC20ZKPNoteProxy contract!!!" );
        allInputZKPNotereqstate[notereq] = State.Spent;

        emit ZKPNoteSpent(notereq, recipient, amount); 
        return true;      
    }

    function getBalanceFor(address fromAssetHash) public view returns (uint256) {
        if (fromAssetHash == address(0)) {
            // return address(this).balance; // this expression would result in error: Failed to decode output: Error: insufficient data for uint256 type
            address selfAddr = address(this);
            return selfAddr.balance;
        } else {
            IERC20 erc20Token = IERC20(fromAssetHash);
            return erc20Token.balanceOf(address(this));
        }
    }

   function _transferToContract(address fromAssetHash, uint256 amount) internal returns (bool) {
        if (fromAssetHash == address(0)) {
            // fromAssetHash === address(0) denotes user choose to lock ether
            // passively check if the received msg.value equals amount
            require(msg.value != 0, "transferred ether cannot be zero!");
            require(msg.value == amount, "transferred ether is not equal to amount!");
        } else {
            // make sure lockproxy contract will decline any received ether
            require(msg.value == 0, "there should be no ether transfer!");
            // actively transfer amount of asset from msg.sender to lock_proxy contract
            require(_transferERC20ToContract(fromAssetHash, _msgSender(), address(this), amount), "transfer erc20 asset to lock_proxy contract failed!");
        }
        return true;
    }
    function _transferFromContract(address toAssetHash, address toAddress, uint256 amount) internal returns (bool) {
        if (toAssetHash == address(0x0000000000000000000000000000000000000000)) {
            // toAssetHash === address(0) denotes contract needs to unlock ether to toAddress
            // convert toAddress from 'address' type to 'address payable' type, then actively transfer ether
            address(uint160(toAddress)).transfer(amount);
        } else {
            // actively transfer amount of asset from msg.sender to lock_proxy contract 
            require(_transferERC20FromContract(toAssetHash, toAddress, amount), "transfer erc20 asset to lock_proxy contract failed!");
        }
        return true;
    }
    
    
    function _transferERC20ToContract(address fromAssetHash, address fromAddress, address toAddress, uint256 amount) internal returns (bool) {
         IERC20 erc20Token = IERC20(fromAssetHash);
        //  require(erc20Token.transferFrom(fromAddress, toAddress, amount), "trasnfer ERC20 Token failed!");
         erc20Token.safeTransferFrom(fromAddress, toAddress, amount);
         return true;
    }
    function _transferERC20FromContract(address toAssetHash, address toAddress, uint256 amount) internal returns (bool) {
         IERC20 erc20Token = IERC20(toAssetHash);
        //  require(erc20Token.transfer(toAddress, amount), "trasnfer ERC20 Token failed!");
         erc20Token.safeTransfer(toAddress, amount);
         return true;
    }
    
    function _serializeTxArgs(CCNoteReq memory args) internal pure returns (bytes memory) {
        bytes memory buff;
        buff = abi.encodePacked(
            ZeroCopySink.WriteVarBytes(args.toAssetHash),
            ZeroCopySink.WriteUint255(args.amount),
            ZeroCopySink.WriteUint255(args.pk1),
            ZeroCopySink.WriteUint255(args.pk2),
            ZeroCopySink.WriteUint64(args.toChainId),
            ZeroCopySink.WriteVarBytes(abi.encodePacked(args.noteid))
            );

        // emit SerializeEvent(buff);
        return buff;
    }

    function _deserializeTxArgs(bytes memory valueBs) internal pure returns (CCNoteReq memory) {
        CCNoteReq memory args;
        uint256 off = 0;
	    bytes memory tempbytes;

        (args.toAssetHash, off) = ZeroCopySource.NextVarBytes(valueBs, off);
        (args.amount, off) = ZeroCopySource.NextUint255(valueBs, off);
        (args.pk1, off) = ZeroCopySource.NextUint255(valueBs, off);
        (args.pk2, off) = ZeroCopySource.NextUint255(valueBs, off);
        (args.toChainId, off) = ZeroCopySource.NextUint64(valueBs, off);
        (tempbytes, off) = ZeroCopySource.NextVarBytes(valueBs, off);
	    args.noteid = Utils.bytesToBytes32(tempbytes) ;

        //emit DeserializeEvent(args.toAssetHash,args.amount,args.pk1,args.pk2,args.toChainId,args.noteid);
        return args;
    }

    function () external payable {}
}
