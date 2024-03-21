# 7 Days Walking Challenge

Challenge inspired and accompanied by [**SEVEN DAYS WALKING**](https://ludovicoeinaudi.com/seven-days-walking/) album by [Ludovico Einaudi](https://ludovicoeinaudi.com/).

---

## `Day 1.` Scaffolding a chain from scratch

2024-03-15

The following steps are based on this documentation: [Ignite CLI](https://docs.ignite.com/)

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
  
  👤 alice's account address: cosmos135hm0swcpvc0ja0dt47zafwdzk24kdlptj7wer
  👤 bob's account address: cosmos1e6kfufjr7ndr9hc4l6mahzvjtq9envrushfsse
  
  🌍 Tendermint node: http://0.0.0.0:26657
  🌍 Blockchain API: http://0.0.0.0:1317
  🌍 Token faucet: http://0.0.0.0:4500
  
  ⋆ Data directory: ~/.sevdays
  ⋆ App binary: ~/go/bin/sevdaysd
  
  Press the 'q' key to stop serve
```

### Test the chain

Open a new terminal and run:

```shell
$  sevdaysd query bank balances cosmos135hm0swcpvc0ja0dt47zafwdzk24kdlptj7wer
balances:
- amount: "100000000"
  denom: stake
- amount: "20000"
  denom: token
pagination:
  total: "2"
```

🎉🎉🎉🎉🎉🎉🎉<br/>

### Remarks

During scaffolding a new chain, a custom module was created automatically.
This module is named `sevdays` (like the chain name) and is placed in the directory named `x`.

---

## `Day 2.` Adding a `wasm` module to the chain 

2024-03-18

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

## `Day 3.` Adding a `wasm` module to the chain

2024-03-21

This time we base on this: [Ignite CLI Wasm App](https://github.com/ignite/apps/tree/main/wasm)

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

Step 6. Check `wasm`

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

Step 7. 🌹 `Every rose has its thorn.` þ

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
