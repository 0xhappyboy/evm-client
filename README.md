# EVM Client

A Lightweight EVM Client Rust library for seamless interaction with multiple EVM-compatible blockchains. Connect to 15+ chains with automatic RPC failover and built-in wallet support.

# Example

## Basic Connection Example

Create a client for Ethereum Mainnet with automatic RPC selection

```rust
use evm_client::{EvmClient, EvmType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EvmClient::from_type(EvmType::ETHEREUM_MAINNET).await?;
    println!("✅ Connected to Ethereum Mainnet");
    client.health().await?;
    println!("✅ Health check passed");
    Ok(())
}
```

## Custom RPC Connection

Connect using a specific RPC endpoint

```rust
use evm_client::EvmClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EvmClient::from_rpc("https://mainnet.infura.io/v3/your-api-key").await?;
    println!("✅ Connected via custom RPC");

    Ok(())
}
```

## Wallet Integration

Create a client with wallet for transaction signing

```rust
use evm_client::{EvmClient, EvmType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key = "0xYourPrivateKeyHere";
    let client = EvmClient::from_wallet(EvmType::POLYGON_MAINNET, private_key).await?;
    println!("✅ Wallet client created");
    let chain_id = client.provider.get_chainid().await?;
    println!("🔗 Chain ID: {}", chain_id);
    Ok(())
}
```

## Multi-Chain Testing

Test connectivity across multiple chains

```rust
use evm_client::{EvmClient, EvmType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chains = [
        EvmType::ETHEREUM_MAINNET,
        EvmType::BSC_MAINNET,
        EvmType::POLYGON_MAINNET,
        EvmType::ARB_MAINNET,
        EvmType::BASE_MAINNET,
    ];
    for chain in chains {
        match EvmClient::from_type(chain).await {
            Ok(client) => {
                let chain_id = client.provider.get_chainid().await?;
                println!("✅ {} - Chain ID: {} - Connected", chain.name(), chain_id);
            }
            Err(e) => {
                println!("❌ {} - Connection failed: {}", chain.name(), e);
            }
        }
    }

    Ok(())
}
```

## Error Handling

Proper error handling for connection failures

```rust
use evm_client::{EvmClient, EvmType, EvmClientError};

#[tokio::main]
async fn main() {
    match EvmClient::from_type(EvmType::ETHEREUM_MAINNET).await {
        Ok(client) => {
            println!("✅ Connection successful");

            match client.health().await {
                Ok(()) => println!("✅ Health check passed"),
                Err(EvmClientError::RpcError(msg)) => {
                    eprintln!("❌ Health check failed: {}", msg);
                }
            }
        }
        Err(EvmClientError::RpcError(msg)) => {
            eprintln!("❌ Connection failed: {}", msg);
        }
    }
}
```

## Custom RPC with Wallet

Combine custom RPC with wallet functionality

```rust
use evm_client::EvmClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://polygon-rpc.com";
    let private_key = "0xYourPrivateKeyHere";

    let client = EvmClient::from_rpc_and_wallet(rpc_url, private_key).await?;
    println!("✅ Custom RPC wallet client created");

    Ok(())
}
```

## Chain Information

Get detailed information about supported chains

```rust
use evm_client::EvmType;

fn main() {
    let chains = [
        EvmType::ETHEREUM_MAINNET,
        EvmType::ARB_MAINNET,
        EvmType::OPTIMISM_MAINNET,
    ];

    for chain in chains {
        println!("📋 Chain: {}", chain.name());
        println!("   ID: {}", chain.chain_id());
        println!("   RPC Endpoints: {}", chain.rpc().len());
        println!("---");
    }
}
```
