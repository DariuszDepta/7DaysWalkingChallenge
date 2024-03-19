# 7 Days Walking Challenge

Challenge inspired and accompanied by [**SEVEN DAYS WALKING**](https://ludovicoeinaudi.com/seven-days-walking/) album by [Ludovico Einaudi](https://ludovicoeinaudi.com/).

## Day 1 - Scaffolding a chain from scratch

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
  
  ⋆ Data directory: /home/confio/.sevdays
  ⋆ App binary: /home/confio/go/bin/sevdaysd
  
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

### Remarks

During scaffolding a new chain, a custom module was created automatically.
This module is named `sevdays` (like the chain name) and is placed in the directory named `x`.

## Day 2 - Adding a `wasm` module to the chain 

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

The first try failed...

## Day 3 - Adding a `wasm` module to the chain

(tbd)
