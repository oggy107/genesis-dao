# Genesis DAO

A general purpose dao designed to empower community-driven decision-making. Whether you’re managing a community fund, organizing events, or making collective choices, Genesis DAO provides a straightforward framework with core features such as base proposals for adding and removing members to dao, granting and revoking voting accesses and a general purpose proposal for any community action.

The versatile nature of general purpose proposal makes it suitable for any task that needs voting in a community. Weather a socitey is using it to elect their building manager or deciding their monthly maintainance fee, general purpose proposal can cover all.

## Testnet dao contract id

CC3QBUVTXRRIW3GOQ7JG2URJU2CCK4VDYAW7QGDGUJU6L5MSQMINZXIK

*Note: This contract id can become invalid anytime due to data cleanup on stellar testnet*

## TestNet config

- rpc-url: `https://soroban-testnet.stellar.org:443`
- passphrase: `Test SDF Network ; September 2015`

## Settingup Backend and initializing the dao contract

### Configure soroban CLI

https://developers.stellar.org/docs/smart-contracts/getting-started/setup

#### configure testnet

```ps
soroban network add --global testnet --rpc-url "https://soroban-testnet.stellar.org:443" --network-passphrase "Test SDF Network ; September 2015"
```

### configure identities

Please create atleast 3 identities as we are pass these as initial dao members or you can pass your own public addresses as well

```ps
soroban config identity generate --rpc-url "https://soroban-testnet.stellar.org:443" --network-passphrase "Test SDF Network ; September 2015" --network testnet <IDENTITY NAME>
```

## Deploy dao and dao-token contract

Compile the contracts in `./contracts` directory by running `soroban contract compile` command. This will generate `wasm` files in target directory.

Upload the wasm file of **dao-token** contract by running following command. This returns the hash of the Wasm bytes, like `6ddb28e0980f643bb97350f7e3bacb0ff1fe74d846c6d4f2c625e766210fbb5b`. store this for further process

```ps
soroban contract install --rpc-url "https://soroban-testnet.stellar.org:443" --network-passphrase "Test SDF Network ; September 2015" --network testnet --source <IDENTITY> --wasm <DAO TOKEN WASM FILE PATH>
```

Next deploy the dao contract by providing path to generated dao wasm file. This will return contract id of dao contract. Note this for further process

```ps
soroban contract deploy --source oggy --wasm "<DAO WASM FILE PATH>" --rpc-url "https://soroban-testnet.stellar.org:443" --network-passphrase "Test SDF Network ; September 2015" --network testnet
```

## Initialize dao contract

**Note: Work still in progress on initialization script**

Take note of public addresses of the 3 identites you have just created by using `soroban config identity address <IDENTITY NAME>` command or you can pass your own addresses as well

Edit `./initialize.sh` and add dao contract id, initial members and dao token arguments such as wasm hash generated when we uploaded dao token wasm to network, salt, name and symbol.

Run `./initialize.sh` after setting up everything