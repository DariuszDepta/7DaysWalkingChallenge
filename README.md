# 7 Days Walking Challenge

Challenge inspired and accompanied by [**SEVEN DAYS WALKING**](https://ludovicoeinaudi.com/seven-days-walking/)
album composed by [Ludovico Einaudi](https://ludovicoeinaudi.com/).

---

- [Day 1.](#day-1-scaffolding-a-chain-from-scratch) Scaffolding a chain from scratch
- [Day 2.](#day-2-adding-a-wasm-module-to-the-chain) Adding a `wasm` module to the chain
- [Day 3.](#day-3-adding-a-wasm-module-to-the-chain-continued) Adding a `wasm` module to the chain (continued)
- [Day 4.](#day-4-deploying-a-smart-contract-written-using-sylvia-framework) Deploying a smart contract written using [Sylvia Framework](https://github.com/CosmWasm/sylvia)
- [Day 5.](#day-5-deploying-a-smart-contract-continued) Deploying a smart contract (continued)

---

## `Day 1.` Scaffolding a chain from scratch

> 2024-03-15

The following steps are based on this documentation: [Ignite CLI](https://docs.ignite.com/)

### Prerequisities

- Install the newest version of [Go](https://go.dev/doc/install).
    ```shell
    $ go version
    go version go1.22.2 linux/amd64
    ```
- Install the newest version of [IgniteCLI](https://docs.ignite.com/welcome/install).
    ```shell
    $ ignite version
    Ignite CLI version:             v28.3.0
    Ignite CLI build date:          2024-03-20T15:31:07Z
    Ignite CLI source hash:         159abdca88605ed82cb4aabd52618db91069b7af
    Ignite CLI config version:      v1
    Cosmos SDK version:             v0.50.5
    Your OS:                        linux
    Your arch:                      amd64
    Your Node.js version:           v21.7.3
    Your go version:                go version go1.22.2 linux/amd64
    Your uname -a:                  [...]
    Your cwd:                       [...]
    Is on Gitpod:                   false
    ```

### Scaffold a chain

Being in the directory where this empty project was cloned for the first time:

```shell
$ ignite scaffold chain sevdays

⭐️ Successfully created a new blockchain 'sevdays'.
👉 Get started with the following commands:

 % cd sevdays
 % ignite chain serve

Documentation: https://docs.ignite.com
```

### Run a chain

```shell
$ cd sevdays
$ ignite chain serve

  Blockchain is running
  
  ✔ Added account alice with address cosmos1pzgy9e0cfzsp8uyeg8ux67su2w57erdl8se9dy and mnemonic:
  mass coconut churn train control switch reward glory jar limb daughter rent knock one machine furnace pioneer clip shed unusual biology sausage wood hotel
  
  ✔ Added account bob with address cosmos1jmfnuxsyyg92treap2055qnkeyq6ksck9ye0cc and mnemonic:
  come wedding destroy hen replace object cry obscure jealous entire decide scare family random diamond call cactus eternal era year brand stomach merge bench
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: ~/.sevdays
  ⋆ App binary: ~/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

Press `q` to stop the chain.

```shell
  💿 Genesis state saved in ~/.ignite/local-chains/sevdays/exported_genesis.json
  
  𝓲 Stopped
```

### Test the chain

```shell
$ ignite chain serve

  Blockchain is running
  
  👤 alice's account address: cosmos1pzgy9e0cfzsp8uyeg8ux67su2w57erdl8se9dy
  👤 bob's account address: cosmos1jmfnuxsyyg92treap2055qnkeyq6ksck9ye0cc
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: ~/.sevdays
  ⋆ App binary: ~/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

Open another terminal and run:

```shell
$ sevdaysd query bank balances cosmos1pzgy9e0cfzsp8uyeg8ux67su2w57erdl8se9dy
balances:
- amount: "100000000"
  denom: stake
- amount: "20000"
  denom: token
pagination:
  total: "2"
```

Press `q` to stop the chain.

```shell
  💿 Genesis state saved in ~/.ignite/local-chains/sevdays/exported_genesis.json
  
  𝓲 Stopped
```

🎉🎉🎉🎉🎉🎉🎉<br/>

### Remarks

During scaffolding a new chain, a custom module is created automatically.
This module is named `sevdays` (like the chain name) and is placed in the directory named `x`.

---

## `Day 2.` Adding a `wasm` module to the chain

> 2024-03-18

Tried following these instructions: [INTEGRATION](https://github.com/CosmWasm/wasmd/blob/main/INTEGRATION.md).

Installed dependencies:

```shell
$ go get github.com/CosmWasm/wasmd/x/wasm/keeper@v0.50.0
go: downloading github.com/CosmWasm/wasmd v0.50.0
go: module github.com/golang/protobuf is deprecated: Use the "google.golang.org/protobuf" module instead.
go: added github.com/CosmWasm/wasmd v0.50.0
go: added github.com/CosmWasm/wasmvm v1.5.0
```

Added dependencies to `app.go` file and tried to compile:

```shell
$ ignite chain serve

  cannot build app:                                                                          
                                                                                             
  error while running command go install github.com/bufbuild/buf/cmd/buf                     
  github.com/cosmos/gogoproto/protoc-gen-gocosmos                                            
  google.golang.org/grpc/cmd/protoc-gen-go-grpc                                              
  google.golang.org/protobuf/cmd/protoc-gen-go                                               
  github.com/cosmos/cosmos-proto/cmd/protoc-gen-go-pulsar                                    
  github.com/grpc-ecosystem/grpc-gateway/protoc-gen-grpc-gateway                             
  github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2: #                          
  github.com/bufbuild/buf/private/pkg/protosource                                            
  ../../../go/pkg/mod/github.com/bufbuild/buf@v1.28.1/private/pkg/protosource/file.go:146:39:
  f.fileDescriptor.GetOptions().GetPhpGenericServices undefined (type                        
  *descriptorpb.FileOptions has no field or method GetPhpGenericServices)                    
  : exit status 1                                                                            
  
  Waiting for a fix before retrying...
  
  Press the 'q' key to stop serve
```

💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>

The first try has failed...

---

## `Day 3.` Adding a `wasm` module to the chain (continued)

> 2024-03-21

This time we are basing on this: [Ignite CLI Wasm App](https://github.com/ignite/apps/tree/main/wasm)

### Step 1. Delete the old blockchain

```shell
# make sure we are in 7DaysWalkingChallenge directory 
$ ls
data  README.md  sevdays
# delete the configuration of the old blockchain 
$ rm -rf ~/.sevdays
# delete the old blockchain
$ rm -rf sevdays
```

### Step 2. Install newest Ignite CLI

Installation instructions can be found [here](https://docs.ignite.com/welcome/install).

```shell
$ curl https://get.ignite.com/cli! | bash
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  4073    0  4073    0     0   5129      0 --:--:-- --:--:-- --:--:--  5136
Installing ignite v28.3.0.....
######################################################################## 100.0%
Installed at /usr/local/bin/ignite
```

Check the installed version, should be equal or greater to the one shown below:
```shell
$ ignite version
Ignite CLI version:             v28.3.0
```

### Step 3. Install Ignite CLI Wasm app

```shell
$ ignite app install -g github.com/ignite/apps/wasm
✔ Done loading apps
🎉 Installed github.com/ignite/apps/wasm
```

### Step 4. Scaffold, run and stop a brand-new chain using Ignite CLI v28.3.0

```shell
$ ignite scaffold chain sevdays

⭐️ Successfully created a new blockchain 'sevdays'.
👉 Get started with the following commands:

 % cd sevdays
 % ignite chain serve

Documentation: https://docs.ignite.com

$ cd sevdays
$ ignite chain serve

  Blockchain is running
  
  ✔ Added account alice with address cosmos1rl0dzwylhk377s7tencn383zudz59rumunexyl and mnemonic:
  eyebrow mouse cash danger try reform pledge develop suit play nut room evoke amateur sentence glad zebra daring speed forward noise blame brief manage
  
  ✔ Added account bob with address cosmos1gy4nz9ggajpkccf992usqehgmvdayaz56rhpjc and mnemonic:
  quick bridge tilt tray coral shrimp town someone ethics nerve quarter shuffle wisdom capital endless drastic involve host close tongue knock family ketchup bike
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: ~/.sevdays
  ⋆ App binary: ~/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

Stop the chain by pressing `q`

```shell

  💿 Genesis state saved in ~/.ignite/local-chains/sevdays/exported_genesis.json
  
  𝓲 Stopped

```

### Step 5. Add Wasm support

```shell
$ ignite wasm add

create app/ante.go
modify app/app.go
modify app/app_config.go
modify app/ibc.go
create app/wasm.go
modify cmd/sevdaysd/cmd/commands.go

🎉 CosmWasm added (`~/7DaysWalkingChallenge/sevdays`).
```

Re-run the chain:

```shell
$ ignite chain serve

  cannot build app:                                                                   
                                                                                      
  error while running command go build -o ~/go/bin/sevdaysd -mod           
  readonly -tags  -ldflags -X github.com/cosmos/cosmos-sdk/version.Name=Sevdays -X    
  github.com/cosmos/cosmos-sdk/version.AppName=sevdaysd -X                            
  github.com/cosmos/cosmos-sdk/version.Version=0.0.2-9a05f70d -X                      
  github.com/cosmos/cosmos-sdk/version.Commit=9a05f70d94d773b7db71225eb7fd971f2b55c58d
  -X github.com/cosmos/cosmos-sdk/version.BuildTags= -X                               
  sevdays/cmd/sevdaysd/cmd.ChainID=sevdays -gcflags all=-N -l .: # sevdays/app        
  ../../app/ibc.go:10:2: "github.com/cosmos/cosmos-sdk/types/module" imported and     
  not used                                                                            
  : exit status 1                                                                     
  
  Waiting for a fix before retrying...
  
  Press the 'q' key to stop serve
```

Remove the unused import:

```shell
$ sed -i '10d' app/ibc.go
```

Re-run the chain:

```shell
$ ignite chain serve

  Blockchain is running
  
  👤 alice's account address: cosmos1rl0dzwylhk377s7tencn383zudz59rumunexyl
  👤 bob's account address: cosmos1gy4nz9ggajpkccf992usqehgmvdayaz56rhpjc
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/confio/.sevdays
  ⋆ App binary: /home/confio/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

### Step 6. Check `wasm`

In another terminal, run:

```shell
$ sevdaysd q wasm pinned
code_ids: []
pagination:
  next_key: null
  total: "0"
```

🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>

### Step 7. `Every rose 🌹 has its thorn þ.`

> þ - [thorn](https://en.wikipedia.org/wiki/Thorn_(letter))

Stop the chain by pressing `q`:

```shell
✘ panic: collections: not found: key 'no_key' of type github.com/cosmos/gogoproto/cosmwasm.wasm.v1.Params                                                                                                    
                                                                                                                                                                                                           
goroutine 122 [running]:                                                                                                                                                                                   
github.com/CosmWasm/wasmd/x/wasm/keeper.Keeper.GetParams({{0x58efde0, 0xc00150ed80}, {0x59625a0, 0xc000c7dd00}, {0x5907830, 0xc0013252c0}, {0x58ef8c0, 0xc0007bf680}, {0x58eeea0, 0xc0028120a8}, ...}, ...)
        /home/confio/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/keeper/keeper.go:124 +0x24d                                                                                                             
github.com/CosmWasm/wasmd/x/wasm/keeper.ExportGenesis({{0x5929630, 0x74e1e00}, {0x59419a0, 0xc002e72c00}, {{0x0, 0x0}, {0x0, 0x0}, 0x13b, {0x0, ...}, ...}, ...}, ...)                                     
        /home/confio/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/keeper/genesis.go:89 +0x106                                                                                                             
github.com/CosmWasm/wasmd/x/wasm.AppModule.ExportGenesis({{}, {0x59625a0, 0xc000c7dd00}, 0xc00015fca8, {0x58eef20, 0xc000df9880}, {0x5907830, 0xc001325680}, {0x7f8ade336108, 0xc00166f558}, ...}, ...)    
        /home/confio/go/pkg/mod/github.com/!cosm!wasm/wasmd@v0.50.0/x/wasm/module.go:194 +0x7b                                                                                                                     
github.com/cosmos/cosmos-sdk/types/module.(*Manager).ExportGenesisForModules.func3({0x7f8adce66258, 0xc002abe690}, 0xc00275cba0)                                                                           
        /home/confio/go/pkg/mod/github.com/cosmos/cosmos-sdk@v0.50.5/types/module/module.go:584 +0x13e                                                                                                             
created by github.com/cosmos/cosmos-sdk/types/module.(*Manager).ExportGenesisForModules in goroutine 1                                                                                                     
        /home/confio/go/pkg/mod/github.com/cosmos/cosmos-sdk@v0.50.5/types/module/module.go:582 +0xb69                                                                                                             
: exit status 2                     
```

💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>

Is it something we can handle or fix?

The error shown above disappears after single state reset: 
```shell
$ ignite chain serve --reset-once
```

## `Day 4.` Deploying a smart contract written using [Sylvia Framework](https://github.com/CosmWasm/sylvia)

> 2024-03-27

In the [walking-contract](./walking-contract) directory there is a smart contract prepared.
This simplistic smart contract was designed using [Sylvia Framework](https://github.com/CosmWasm/sylvia). 

### Step 1. Compile smart contract

```shell
$ ls
data  README.md  sevdays  walking-contract
$ cd walking-contract
$ cargo wasm
```

The `Wasm` compiled binary named `walking_contract.wasm` is available in `target/wasm32-unknown-unknown/release/`

### Step 2. Start the chain

```shell
$ cd ../sevdays
$ ignite chain serve

  Blockchain is running
  
  👤 alice's account address: cosmos1mn8un49p58dr6j2f85u5axnd8dyqlgmmfnfc6h
  👤 bob's account address: cosmos1rwng8afqdgam45x5m75e6z9mc6pwrhfyust477
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: /home/confio/.sevdays
  ⋆ App binary: /home/confio/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

### Step 3. Store the smart contract on the chain

Open another terminal. Make sure you are in the right directory, and you can run `sevdaysd`:

```shell
$ ls
data  README.md  sevdays  walking-contract

$ which sevdaysd
~/go/bin/sevdaysd
```

Store contract's binary on a chain:

```shell
$ sevdaysd tx wasm store ./walking-contract/target/wasm32-unknown-unknown/release/walking_contract.wasm --from alice --chain-id sevdays --gas 10000000 -y
```

Query stored wasm binaries:

```shell
$ sevdaysd q wasm list-code
code_infos: []
pagination:
  next_key: null
  total: "0"
```

Smart contract was not deployed! Let's check why.

Let's try to store the contract's binary once again:

```shell
$ sevdaysd tx wasm store ./walking-contract/target/wasm32-unknown-unknown/release/walking_contract.wasm --from alice --chain-id sevdays --gas 10000000 -y
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 6FE7640A637A8D35C56661A6CDBD9E638C20B50244774F63AF871247A1C72E30
```

Now let's query the transaction:

```shell
$ sevdaysd q tx 6FE7640A637A8D35C56661A6CDBD9E638C20B50244774F63AF871247A1C72E30
```

Unfortunately the whole wasm binary is displayed to the console, so the output is hard to read without long scrolling.
But the output is in YAML format, so let's use `yq` to filer out the wasm binary dump. 

To install `yq` type:
```shell
$ pip install yq
```

Now let's query the transaction once again, but with the wasm dump filtered out:

```shell
$ sevdaysd q tx 6FE7640A637A8D35C56661A6CDBD9E638C20B50244774F63AF871247A1C72E30 | yq 'del(.tx.body.messages[0].wasm_byte_code)'
```

The transaction looks like this:

```json
{
  "code": 2,
  "codespace": "wasm",
  "data": "",
  "events": [
    {
      "attributes": [
        {
          "index": true,
          "key": "fee",
          "value": ""
        },
        {
          "index": true,
          "key": "fee_payer",
          "value": "cosmos1j2sy5h7uks4f55wrqsmyfrlxfys20ake9026e3"
        }
      ],
      "type": "tx"
    },
    {
      "attributes": [
        {
          "index": true,
          "key": "acc_seq",
          "value": "cosmos1j2sy5h7uks4f55wrqsmyfrlxfys20ake9026e3/3"
        }
      ],
      "type": "tx"
    },
    {
      "attributes": [
        {
          "index": true,
          "key": "signature",
          "value": "wiCaKZ7vlswjAjLyqCsds5Svq96ZLUo8laFbDwvXSrIAlgjs6imMcDXpKokgYTmjl453EPNWgCbfY1Ry90/NYQ=="
        }
      ],
      "type": "tx"
    }
  ],
  "gas_used": "4418391",
  "gas_wanted": "10000000",
  "height": "2665",
  "info": "",
  "logs": [],
  "raw_log": "failed to execute message; message index: 0: uncompress wasm archive: max 819200 bytes: exceeds limit: create wasm contract failed",
  "timestamp": "2024-03-28T10:26:26Z",
  "tx": {
    "@type": "/cosmos.tx.v1beta1.Tx",
    "auth_info": {
      "fee": {
        "amount": [],
        "gas_limit": "10000000",
        "granter": "",
        "payer": ""
      },
      "signer_infos": [
        {
          "mode_info": {
            "single": {
              "mode": "SIGN_MODE_DIRECT"
            }
          },
          "public_key": {
            "@type": "/cosmos.crypto.secp256k1.PubKey",
            "key": "A9tNsCSNtKk93c1ENAy1v0fXKsertI32vStSX0MaugXo"
          },
          "sequence": "3"
        }
      ],
      "tip": null
    },
    "body": {
      "extension_options": [],
      "memo": "",
      "messages": [
        {
          "@type": "/cosmwasm.wasm.v1.MsgStoreCode",
          "instantiate_permission": null,
          "sender": "cosmos1j2sy5h7uks4f55wrqsmyfrlxfys20ake9026e3"
        }
      ],
      "non_critical_extension_options": [],
      "timeout_height": "0"
    },
    "signatures": [
      "wiCaKZ7vlswjAjLyqCsds5Svq96ZLUo8laFbDwvXSrIAlgjs6imMcDXpKokgYTmjl453EPNWgCbfY1Ry90/NYQ=="
    ]
  },
  "txhash": "6FE7640A637A8D35C56661A6CDBD9E638C20B50244774F63AF871247A1C72E30"
}
```

The error is:

```text
"raw_log": "failed to execute message; message index: 0: uncompress wasm archive: max 819200 bytes: exceeds limit: create wasm contract failed",
```

💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>
💣💣💣💣💣💣💣</br>

So there must be an option to increase this parameter...

## `Day 5.` Deploying a smart contract (continued)

> 2024-03-28

### Step 1. Fix: change the maximum allowed contract size 

...and [Pinò](https://github.com/pinosu) has spotted it. There is a parameter called `MaxWasmSize` in `wasmd` which
must be adjusted to allow bigger contracts to be storable on the chain.

Add the following function to the end of the `app.go` file:

```Go
// overrideWasmVariables overrides the wasm variables.
func overrideWasmVariables() {
	// Override Wasm size limitation from `wasmd`.
	wasmtypes.MaxWasmSize = 2.5 * 1024 * 1024 // ~2.5 mb
	wasmtypes.MaxProposalWasmSize = wasmtypes.MaxWasmSize
}
```

Call this function on initialization in `app.go` file:

```Go
// New returns a reference to an initialized App.
func New(
	logger log.Logger,
	db dbm.DB,
	traceStore io.Writer,
	loadLatest bool,
	appOpts servertypes.AppOptions,
	baseAppOptions ...func(*baseapp.BaseApp),
) (*App, error) {
	overrideWasmVariables() // <--- call it here!
	var (
		app        = &App{}
		appBuilder *runtime.AppBuilder
```

Add all needed imports. The full diff patch is shown below:

```text
diff --git a/sevdays/app/app.go b/sevdays/app/app.go
index 830a26f..4bdac9c 100644
--- a/sevdays/app/app.go
+++ b/sevdays/app/app.go
@@ -19,6 +19,7 @@ import (
 	_ "cosmossdk.io/x/nft/module" // import for side-effects
 	_ "cosmossdk.io/x/upgrade"    // import for side-effects
 	upgradekeeper "cosmossdk.io/x/upgrade/keeper"
+	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
 	dbm "github.com/cosmos/cosmos-db"
 	"github.com/cosmos/cosmos-sdk/baseapp"
 	"github.com/cosmos/cosmos-sdk/client"
@@ -75,9 +76,11 @@ import (
 	ibctransferkeeper "github.com/cosmos/ibc-go/v8/modules/apps/transfer/keeper"
 	ibckeeper "github.com/cosmos/ibc-go/v8/modules/core/keeper"
 
+	sevdaysmodulekeeper "sevdays/x/sevdays/keeper"
+
 	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
 	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
-	sevdaysmodulekeeper "sevdays/x/sevdays/keeper"
+
 	// this line is used by starport scaffolding # stargate/app/moduleImport
 
 	"sevdays/docs"
@@ -202,6 +205,7 @@ func New(
 	appOpts servertypes.AppOptions,
 	baseAppOptions ...func(*baseapp.BaseApp),
 ) (*App, error) {
+	overrideWasmVariables()
 	var (
 		app        = &App{}
 		appBuilder *runtime.AppBuilder
@@ -475,3 +479,11 @@ func BlockedAddresses() map[string]bool {
 	}
 	return result
 }
+
+// overrideWasmVariables overrides the wasm variables to:
+//   - allow for larger wasm files
+func overrideWasmVariables() {
+	// Override Wasm size limitation from WASMD.
+	wasmtypes.MaxWasmSize = 1024 * 1024 * 2.5 // ~2.5 mb
+	wasmtypes.MaxProposalWasmSize = wasmtypes.MaxWasmSize
+}
```

### Step 2. Store the contract on the chain

Start the chain:

```shell
$ ignite chain serve

  Blockchain is running
  
  👤 alice's account address: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
  👤 bob's account address: cosmos1pkt49ry08kmen25xslrs969lkpj7u93trha34r
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: ~/.sevdays
  ⋆ App binary: ~/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

Store the contract code on the chain: 

```shell
$ sevdaysd tx wasm store ./walking-contract/target/wasm32-unknown-unknown/release/walking_contract.wasm --from alice --chain-id sevdays --gas 10000000 -y
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 004F6D1421F5FB7CDEE00260ACF6C62924CAAE138A3B4290C32BCDE89362CF83
```

Let's check if this transaction was successful:

```shell
$ sevdaysd query tx --type=hash 004F6D1421F5FB7CDEE00260ACF6C62924CAAE138A3B4290C32BCDE89362CF83 | yq 'del(.tx.body.messages[0].wasm_byte_code)'
```

Let's list all stored contact codes on the chain (we should see only one):

```shell
$ sevdaysd q wasm list-code
code_infos:
- code_id: "1"
  creator: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
  data_hash: 1478E606B86886D210BF12026B0D3CFECB6D869144E1CF3E3872A37F1D7AD8B8
  instantiate_permission:
    addresses: []
    permission: Everybody
pagination:
  next_key: null
  total: "0"
```

Having `code_id` we can check the code details:

```shell
$ sevdaysd q wasm code-info 1
code_id: "1"
creator: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
data_hash: 1478E606B86886D210BF12026B0D3CFECB6D869144E1CF3E3872A37F1D7AD8B8
instantiate_permission:
  addresses: []
  permission: Everybody
```

### Step 3. Instantiate a contract

Instantiate a new contract based on contract code with `code_id = 1`:

```shell
$ sevdaysd tx wasm instantiate 1 {} --label walking_contract --no-admin --from alice --chain-id sevdays
auth_info:
  fee:
    amount: []
    gas_limit: "200000"
    granter: ""
    payer: ""
  signer_infos: []
  tip: null
body:
  extension_options: []
  memo: ""
  messages:
  - '@type': /cosmwasm.wasm.v1.MsgInstantiateContract
    admin: ""
    code_id: "1"
    funds: []
    label: walking_contract
    msg: {}
    sender: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
  non_critical_extension_options: []
  timeout_height: "0"
signatures: []
confirm transaction before signing and broadcasting [y/N]: y
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: BC5D36820AC173AFC60CE5210490EA654F8222E86BC5BF279CC519F6F5771175
```

Let's list all contract instances created basing on the contract code with `code_id = 1`,
wes should get a single contract's address.

```shell
$ sevdaysd q wasm list-contract-by-code 1
contracts:
- cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr
pagination:
  next_key: null
  total: "0"
```

To query the details of the contract instance, having its address, just run:
```shell
$ sevdaysd q wasm contract cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr
address: cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr
contract_info:
  admin: ""
  code_id: "1"
  created:
    block_height: "1326"
    tx_index: "0"
  creator: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
  extension: null
  ibc_port_id: ""
  label: walking_contract
```

### Step 4. Interact with the contract

The example contract has an internal counter we can query, the initial value of the counter should be zero:

```shell
$ sevdaysd q wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"count":{}}'
data:
  count: 0
```

Now let's increment this counter, executing message on the contract:

```shell
$ sevdaysd tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"increment":{}}' --from alice --chain-id sevdays
auth_info:
  fee:
    amount: []
    gas_limit: "200000"
    granter: ""
    payer: ""
  signer_infos: []
  tip: null
body:
  extension_options: []
  memo: ""
  messages:
  - '@type': /cosmwasm.wasm.v1.MsgExecuteContract
    contract: cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr
    funds: []
    msg:
      increment: {}
    sender: cosmos1rche4ezyvjfd5wc4trc78r7jv9g0en4eln8p6g
  non_critical_extension_options: []
  timeout_height: "0"
signatures: []
confirm transaction before signing and broadcasting [y/N]: y
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: E27286A5CA393D4DBB0039F822607535A6247D09F145E7741CE54825EE349015
```

Let's check the current counter value:

```shell
$ sevdaysd q wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"count":{}}'
data:
  count: 1
```

Let's increment once again (we have added `-y` at the end, so the transaction is confirmed automatically):

```shell
$ sevdaysd tx wasm execute cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"increment":{}}' --from alice --chain-id sevdays -y
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 0D1E5A3C848A641D564D97CAED45B7D6D028595DDD888BE2180ED84DDB850926
```

Let's check the counter now:

```shell
$ sevdaysd q wasm contract-state smart cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr '{"count":{}}'
data:
  count: 2
```

🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>
🎉🎉🎉🎉🎉🎉🎉<br/>

## `Day 6.` Query custom module from smart contract

> (tbd)
