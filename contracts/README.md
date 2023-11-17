# Devnet contracts for development

This folder contains Cairo and Solidity contracts for developement of Devnet.
If you wish you to check specifically one of the two chains contracts, please check:
1. `solidity` folder for Ethereum related contracts.
2. `cairo` folder for Starknet related contracts, and example of how to work with starknet without
   running a L1 node.

## Full example of e2e testing with Anvil and Devnet

### Setup of the nodes
Please ensure that you've [anvil](https://book.getfoundry.sh/getting-started/installation) installed (or you can do the same with HardHat, but the commands here are done with anvil).
```bash
anvil
```

For Starknet, ensure you've Devnet compiled and running with the following params:
```bash
cargo run -- --seed 42
```

Now both nodes are running, Devnet for Starkne and Anvil for Ethereum.

Then, **open two terminals**:
* one for Starknet then `cd contracts/cairo`.
* one for Ethereum then `cd contracts/solidity`.

### Etherum setup
1. Use Devnet postman endpoint to load the `MockStarknetMessaging` contract:
```bash
curl -H 'Content-Type: application/json' \
     -d '{"networkUrl": "http://127.0.0.1:8545"}' \
     http://127.0.0.1:5050/postman/load_l1_messaging_contract

{
    "messageContractAddress":"0x5fbdb2315678afecb367f032d93f642f64180aa3"
}
```

2. Deploy the `L1L2.sol` contract in order to receive/send messages from/to L2.
```bash
# Ethereum terminal (contracts/solidity)
source .env
forge script script/L1L2.s.sol:Deploy --broadcast --rpc-url $ETH_RPC_URL --silent

✅  [Success]Hash: 0x942cfaadc557f360b91e2bfe98e8246d87b8efb4bfe6c1803162cd4aa7a71e1d
Contract Address: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
Block: 2
Paid: 0.0013459867197597 ETH (346581 gas * 3.8836137 gwei)
```

3. Check balance is 0 for user `1`:
```bash
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "get_balance(uint256)(uint256)" 0x1

0
```

### Starknet contracts and send message to L1
1. On the Starknet terminal, set [starkli](https://book.starkli.rs/installation) variables:
```bash
# Starknet terminal (contracts/cairo)
source .env

# Declare
starkli declare target/dev/cairo_l1_l2.contract_class.json

# Deploy (adjust the class hash if needed)
starkli deploy 0x0211fd0483be230ba40d43f51bd18ae239b913f529f95ce10253e514175efb3e --salt 123

# Add some balance and check it.
starkli invoke 0x03c80468c8fe2fd36fadf1b484136b4cd8a372f789e8aebcc6671e00101290a4 increase_balance 0x1 0xff
starkli call 0x03c80468c8fe2fd36fadf1b484136b4cd8a372f789e8aebcc6671e00101290a4 get_balance 0x1

# Issue a withdraw to send message to L1 with amount 1 for user 1.
starkli invoke 0x03c80468c8fe2fd36fadf1b484136b4cd8a372f789e8aebcc6671e00101290a4 withdraw 0x1 1 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512

# Here, you can still check on ethereum, the balance is still 0.
# You can use the `dry run` version if you just want to check the messages before actually sending them.
curl -H 'Content-Type: application/json' -d '{"dryRun": true}' http://127.0.0.1:5050/postman/flush

{
    "messagesToL1": [
        {
            "l2_contract_address":"0x34ba56f92265f0868c57d3fe72ecab144fc96f97954bbbc4252cef8e8a979ba",
            "l1_contract_address":"0xe7f1725e7734ce288f8367e1bb143e90bb3f0512",
            "payload":["0x0","0x1","0x1"]
        }
    ],
    "messagesToL2":[],
    "l1Provider":"dry run"
}

# Flushing the message to actually send them to the L1.
curl -H 'Content-Type: application/json' -d '{}' http://127.0.0.1:5050/postman/flush

{
    "messagesToL1": [
        {
            "l2_contract_address":"0x34ba56f92265f0868c57d3fe72ecab144fc96f97954bbbc4252cef8e8a979ba",
            "l1_contract_address":"0xe7f1725e7734ce288f8367e1bb143e90bb3f0512",
            "payload":["0x0","0x1","0x1"]
        }
    ],
    "messagesToL2":[],
    "l1Provider":"http://127.0.0.1:8545/"
}

```

### Etherum receive message and send message to L2
1. Now the message is received, we can consume it.
```bash
# Ethereum terminal (contracts/solidity)
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "withdraw(uint256, uint256, uint256)" \
     0x34ba56f92265f0868c57d3fe72ecab144fc96f97954bbbc4252cef8e8a979ba 0x1 0x1 \
     --rpc-url $ETH_RPC_URL --private-key $ACCOUNT_PRIVATE_KEY \
     --gas-limit 999999
     
# We can now check the balance, it's 1.
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "get_balance(uint256)(uint256)" 0x1

1
```

2. Let's now send back the amount 1 we just received. As we will send a message, we need
to provide at least 30k WEI.
```bash
# Ethereum terminal (contracts/solidity)
cast send 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "deposit(uint256, uint256, uint256)" \
     0x03c80468c8fe2fd36fadf1b484136b4cd8a372f789e8aebcc6671e00101290a4 0x1 0x1 \
     --rpc-url $ETH_RPC_URL --private-key $ACCOUNT_PRIVATE_KEY \
     --gas-limit 999999 --value 1gwei
     
# The balance is now 0.
cast call 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 "get_balance(uint256)(uint256)" 0x1

0
```

3. Flush the messages.
```bash
curl -H 'Content-Type: application/json' -d '{}' http://127.0.0.1:5050/postman/flush

{
    "messagesToL1": [],
    "messagesToL2": [
        {
            "l2ContractAddress":"0x3c80468c8fe2fd36fadf1b484136b4cd8a372f789e8aebcc6671e00101290a4",
            "entryPointSelector":"0xc73f681176fc7b3f9693986fd7b14581e8d540519e27400e88b8713932be01",
            "l1ContractAddress":"0xe7f1725e7734ce288f8367e1bb143e90bb3f0512",
            "payload":["0x1","0x1"],
            "paidFeeOnL1":"0x3b9aca00",
            "nonce":"0x1"
        }
    ],
    "l1Provider":"http://127.0.0.1:8545/"
}
```