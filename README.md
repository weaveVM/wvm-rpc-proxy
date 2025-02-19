<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
Proxy JSON-RPC server for [wvm.dev](https://wvm.dev)

## Build locally

```bash
git clone https://github.com/weavevm/wvm-proxy-rpc.git

cd wvm-proxy-rpc

shuttle run
```

## Networks

| Network  | Supported |
| ------------- |:-------------:|
| testnet      | ✅      |
| mainnet      |  ❌    |

## Usage example

```bash
curl -X POST https://testnet.wvm.dev -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
```
#### Local build

```bash
curl -X POST https://testnet.wvm.dev -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
```

## License
This project is licensed under the [MIT License](./LICENSE)