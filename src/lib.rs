use std::{fmt, sync::Arc};

use ethers::{
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvmType {
    ETHEREUM_MAINNET,
    ARB_MAINNET,
    BSC_MAINNET,
    BASE_MAINNET,
    HYPEREVM_MAINNET,
    PLASMA_MAINNET,
    POLYGON_MAINNET,
    OPTIMISM_MAINNET,
    ZKSYNC_MAINNET,
    STARKNET_MAINNET,
    AVALANCHE_MAINNET,
    FANTOM_MAINNET,
    RONIN_MAINNET,
    SKALE_MAINNET,
    IMMUTABLE_MAINNET,
}

impl EvmType {
    pub fn name(&self) -> &'static str {
        match self {
            EvmType::ETHEREUM_MAINNET => "Ethereum Mainnet",
            EvmType::ARB_MAINNET => "Arbitrum One",
            EvmType::BSC_MAINNET => "BNB Smart Chain",
            EvmType::BASE_MAINNET => "Base Mainnet",
            EvmType::HYPEREVM_MAINNET => "HyperEVM Mainnet",
            EvmType::PLASMA_MAINNET => "Plasma Mainnet",
            EvmType::POLYGON_MAINNET => "Polygon Mainnet",
            EvmType::OPTIMISM_MAINNET => "Optimism Mainnet",
            EvmType::ZKSYNC_MAINNET => "zkSync Era Mainnet",
            EvmType::STARKNET_MAINNET => "StarkNet Mainnet",
            EvmType::AVALANCHE_MAINNET => "Avalanche C-Chain",
            EvmType::FANTOM_MAINNET => "Fantom Opera",
            EvmType::RONIN_MAINNET => "Ronin Mainnet",
            EvmType::SKALE_MAINNET => "SKALE Mainnet",
            EvmType::IMMUTABLE_MAINNET => "Immutable zkEVM",
        }
    }

    pub fn chain_id(&self) -> u64 {
        match self {
            EvmType::ETHEREUM_MAINNET => 1,
            EvmType::ARB_MAINNET => 42161,
            EvmType::BSC_MAINNET => 56,
            EvmType::BASE_MAINNET => 8453,
            EvmType::HYPEREVM_MAINNET => 123420111,
            EvmType::PLASMA_MAINNET => 123420555,
            EvmType::POLYGON_MAINNET => 137,
            EvmType::OPTIMISM_MAINNET => 10,
            EvmType::ZKSYNC_MAINNET => 324,
            EvmType::STARKNET_MAINNET => 23448594291968334,
            EvmType::AVALANCHE_MAINNET => 43114,
            EvmType::FANTOM_MAINNET => 250,
            EvmType::RONIN_MAINNET => 2020,
            EvmType::SKALE_MAINNET => 1273227453,
            EvmType::IMMUTABLE_MAINNET => 13371,
        }
    }

    pub fn rpc(&self) -> Vec<&str> {
        match self {
            EvmType::ETHEREUM_MAINNET => vec![
                "https://reth-ethereum.ithaca.xyz/rpc",
                "https://cloudflare-eth.com",
                "https://rpc.ankr.com/eth",
                "https://eth.llamarpc.com",
                "https://1rpc.io/eth",
                "https://rpc.flashbots.net",
                "https://ethereum.publicnode.com",
                "https://eth-rpc.gateway.pokt.network",
                "https://api.mycryptoapi.com/eth",
                "https://nodes.mewapi.io/rpc/eth",
                "https://mainnet-nethermind.blockscout.com",
                "https://eth-pokt.nodies.app",
                "https://eth.rpc.blxrbdn.com",
                "https://virginia.rpc.blxrbdn.com",
                "https://uk.rpc.blxrbdn.com",
                "https://singapore.rpc.blxrbdn.com",
                "https://eth-mainnet.public.blastapi.io",
                "https://api.securerpc.com/v1",
                "https://eth-mainnet.gateway.pokt.network/v1/5f3453978e354ab992c4da79",
                "https://eth-mainnet.rpcfast.com?api_key=xbhWBI1Wkguk8SNMu1bvvLurPGLXmgwYeC4S6g2H7WdwFigZSmPWVZRxrskEQwIf",
            ],
            EvmType::ARB_MAINNET => vec![
                "https://arb1.arbitrum.io/rpc",
                "https://arbitrum-one.publicnode.com",
                "https://1rpc.io/arb",
                "https://arbitrum.rpc.subquery.network/public",
                "https://arbitrum.blockpi.network/v1/rpc/public",
                "https://arbitrum.meowrpc.com",
                "https://arbitrum.api.onfinality.io/public",
                "https://endpoints.omniatech.io/v1/arbitrum/one/public",
                "https://arbitrum.drpc.org",
                "https://arb-pokt.nodies.app",
                "https://arbitrum-rpc.publicnode.com",
                "https://rpc.arb1.arbitrum.gateway.fm",
                "https://arbitrum-one-rpc.publicnode.com",
                "https://arb-mainnet.g.alchemy.com/v2/demo",
                "https://arbitrum-mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
                "https://arbitrum.getblock.io/api/mainnet",
            ],
            EvmType::BSC_MAINNET => vec![
                "https://bsc-dataseed.binance.org",
                "https://bsc-dataseed1.binance.org",
                "https://bsc-dataseed2.binance.org",
                "https://bsc-dataseed3.binance.org",
                "https://bsc-dataseed4.binance.org",
                "https://bsc-dataseed1.defibit.io",
                "https://bsc-dataseed2.defibit.io",
                "https://bsc-dataseed3.defibit.io",
                "https://bsc-dataseed1.ninicoin.io",
                "https://bsc-dataseed2.ninicoin.io",
                "https://bsc-dataseed3.ninicoin.io",
                "https://bsc-rpc.publicnode.com",
                "https://bsc.publicnode.com",
                "https://1rpc.io/bnb",
                "https://bsc.meowrpc.com",
                "https://bsc.blockpi.network/v1/rpc/public",
                "https://bsc-mainnet.nodereal.io/v1/64a9df0874fb4a93b9d0a3849de012d3",
                "https://bsc-mainnet.rpcfast.com?api_key=xbhWBI1Wkguk8SNMu1bvvLurPGLXmgwYeC4S6g2H7WdwFigZSmPWVZRxrskEQwIf",
                "https://binance.llamarpc.com",
                "https://bsc-dataseed.binance.gg",
            ],
            EvmType::BASE_MAINNET => vec![
                "https://mainnet.base.org",
                "https://base-rpc.publicnode.com",
                "https://base.gateway.tenderly.co",
                "https://1rpc.io/base",
                "https://base-pokt.nodies.app",
                "https://base.meowrpc.com",
                "https://base.api.onfinality.io/public",
                "https://base.blockpi.network/v1/rpc/public",
                "https://developer-access-mainnet.base.org",
                "https://base.llamarpc.com",
                "https://base-mainnet.public.blastapi.io",
                "https://base.drpc.org",
                "https://base-mainnet.core.chainstack.com",
                "https://base-mainnet.gateway.pokt.network/v1/5f3453978e354ab992c4da79",
                "https://base.getblock.io/api/mainnet",
            ],
            EvmType::HYPEREVM_MAINNET => vec![
                "https://hybrid-mainnet.evm.nodes.onflow.org",
                "https://mainnet.evm.nodes.onflow.org",
                "https://evm-rpc.mainnet.hyperchain.xyz",
                "https://hyperchain.alt.technology",
                "https://rpc.hybrid.mainnet.hyperchain.xyz",
                "https://hyper-evm.rpc.subquery.network/public",
                "https://hyper-evm.publicnode.com",
                "https://hyper-evm.blockpi.network/v1/rpc/public",
                "https://hyper-evm.drpc.org",
                "https://flow-hybrid-mainnet.g.alchemy.com/v2/demo",
            ],
            EvmType::PLASMA_MAINNET => vec![
                "https://rpc.plasma.evm.onyx.org",
                "https://plasma-rpc.daoswap.cc",
                "https://plasma.daoswap.cc",
                "https://plasma-evm.rpc.subquery.network/public",
                "https://plasma-evm.blockpi.network/v1/rpc/public",
                "https://plasma-evm.publicnode.com",
                "https://rpc.plasma.quarkchain.io",
                "https://plasma-mainnet.rpc.thirdweb.com",
                "https://plasma.drpc.org",
                "https://plasma-rpc.nodeinfra.com",
                "https://plasma-mainnet.core.chainstack.com",
            ],
            EvmType::POLYGON_MAINNET => vec![
                "https://polygon-rpc.com",
                "https://polygon-bor.publicnode.com",
                "https://rpc.ankr.com/polygon",
                "https://1rpc.io/matic",
                "https://polygon.blockpi.network/v1/rpc/public",
                "https://polygon.meowrpc.com",
                "https://rpc-mainnet.matic.network",
                "https://rpc-mainnet.maticvigil.com",
                "https://rpc-mainnet.matic.quiknode.pro",
                "https://matic-mainnet-full-rpc.bwarelabs.com",
                "https://matic-mainnet-archive-rpc.bwarelabs.com",
                "https://polygon-mainnet.public.blastapi.io",
                "https://polygon.drpc.org",
                "https://polygon-pokt.nodies.app",
                "https://polygon-mainnet.g.alchemy.com/v2/demo",
                "https://polygon-mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
                "https://polygon.api.onfinality.io/public",
            ],
            EvmType::OPTIMISM_MAINNET => vec![
                "https://optimism-rpc.publicnode.com",
                "https://rpc.ankr.com/optimism",
                "https://1rpc.io/op",
                "https://optimism.blockpi.network/v1/rpc/public",
                "https://mainnet.optimism.io",
                "https://optimism.meowrpc.com",
                "https://opt-mainnet.g.alchemy.com/v2/demo",
                "https://optimism-mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
                "https://optimism.api.onfinality.io/public",
                "https://optimism.drpc.org",
                "https://optimism-pokt.nodies.app",
                "https://optimism-mainnet.public.blastapi.io",
                "https://endpoints.omniatech.io/v1/op/mainnet/public",
                "https://optimism.getblock.io/api/mainnet",
            ],
            EvmType::ZKSYNC_MAINNET => vec![
                "https://mainnet.era.zksync.io",
                "https://zksync-rpc.com",
                "https://1rpc.io/zksync",
                "https://zksync.drpc.org",
                "https://zksync.meowrpc.com",
                "https://zksync-era.blockpi.network/v1/rpc/public",
                "https://zksync-mainnet.public.blastapi.io",
                "https://zksync.rpc.subquery.network/public",
                "https://zksync-pokt.nodies.app",
                "https://zksync.getblock.io/api/mainnet",
                "https://zksync-era.core.chainstack.com",
                "https://zksync-era.rpc.thirdweb.com",
                "https://zksync-era-mainnet.public.immudb.io",
            ],
            EvmType::STARKNET_MAINNET => vec![
                "https://starknet-mainnet.public.blastapi.io",
                "https://free-rpc.nethermind.io/mainnet",
                "https://rpc.starknet.lava.build",
                "https://starknet-mainnet.public.chainstack.com",
                "https://starknet-mainnet.g.alchemy.com/v2/demo",
                "https://starknet-mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
                "https://starknet.api.onfinality.io/public",
                "https://starknet.drpc.org",
                "https://starknet-mainnet.core.chainstack.com",
                "https://starknet.rpc.subquery.network/public",
                "https://starknet-mainnet.public.immudb.io",
            ],
            EvmType::AVALANCHE_MAINNET => vec![
                "https://avalanche-c-chain-rpc.publicnode.com",
                "https://rpc.ankr.com/avalanche",
                "https://1rpc.io/avax",
                "https://avax.meowrpc.com",
                "https://api.avax.network/ext/bc/C/rpc",
                "https://avalanche-mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161",
                "https://avax-pokt.nodies.app",
                "https://avalanche.drpc.org",
                "https://avalanche-mainnet.public.blastapi.io",
                "https://avalanche.api.onfinality.io/public",
                "https://avalanche-c-chain-rpc.gateway.pokt.network",
                "https://avax.getblock.io/api/mainnet",
                "https://avalanche-mainnet.core.chainstack.com",
                "https://avalanche.rpc.subquery.network/public",
                "https://avalanche-mainnet.gateway.pokt.network/v1/5f3453978e354ab992c4da79",
            ],
            EvmType::FANTOM_MAINNET => vec![
                "https://fantom-rpc.publicnode.com",
                "https://rpc.ankr.com/fantom",
                "https://1rpc.io/ftm",
                "https://fantom.blockpi.network/v1/rpc/public",
                "https://rpc.fantom.network",
                "https://fantom.publicnode.com",
                "https://fantom-mainnet.gateway.pokt.network/v1/5f3453978e354ab992c4da79",
                "https://fantom.drpc.org",
                "https://fantom-mainnet.public.blastapi.io",
                "https://fantom.api.onfinality.io/public",
                "https://fantom.getblock.io/api/mainnet",
                "https://fantom-mainnet.core.chainstack.com",
                "https://fantom.rpc.subquery.network/public",
                "https://fantom-mainnet.g.alchemy.com/v2/demo",
            ],
            EvmType::RONIN_MAINNET => vec![
                "https://rpc.roninchain.com",
                "https://api.roninchain.com/rpc",
                "https://ronin-rpc.nodeinfra.com",
                "https://ronin.drpc.org",
                "https://ronin-mainnet.public.blastapi.io",
                "https://ronin.api.onfinality.io/public",
                "https://ronin.blockpi.network/v1/rpc/public",
                "https://ronin-pokt.nodies.app",
                "https://ronin.getblock.io/api/mainnet",
                "https://ronin-mainnet.core.chainstack.com",
                "https://ronin.rpc.subquery.network/public",
            ],
            EvmType::SKALE_MAINNET => vec![
                "https://mainnet.skalenodes.com/v1/whispering-turais",
                "https://rpc.skale.space",
                "https://skale-rpc.nodeinfra.com",
                "https://skale.drpc.org",
                "https://skale-mainnet.public.blastapi.io",
                "https://skale.api.onfinality.io/public",
                "https://skale.blockpi.network/v1/rpc/public",
                "https://skale-pokt.nodies.app",
                "https://skale.getblock.io/api/mainnet",
                "https://skale-mainnet.core.chainstack.com",
                "https://skale.rpc.subquery.network/public",
            ],
            EvmType::IMMUTABLE_MAINNET => vec![
                "https://rpc.immutable.com",
                "https://immutable-zkevm.drpc.org",
                "https://rpc.immutable.com/node-api",
                "https://immutable-mainnet.core.chainstack.com",
                "https://immutable-mainnet.public.blastapi.io",
                "https://immutable.api.onfinality.io/public",
                "https://immutable.blockpi.network/v1/rpc/public",
                "https://immutable-pokt.nodies.app",
                "https://immutable.getblock.io/api/mainnet",
                "https://immutable.rpc.subquery.network/public",
                "https://immutable-mainnet.g.alchemy.com/v2/demo",
            ],
        }
    }
}

#[derive(Debug)]
pub enum EvmClientError {
    RpcError(String),
}

impl fmt::Display for EvmClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvmClientError::RpcError(msg) => write!(f, "RPC error: {}", msg),
        }
    }
}

impl std::error::Error for EvmClientError {}

/// EVM Client for interacting with various EVM chains
#[derive(Clone)]
pub struct EvmClient {
    /// The ethers-rs provider for blockchain interactions
    pub provider: Arc<Provider<Http>>,
    /// The type of EVM chain this client is connected to
    pub evm_type: Option<EvmType>,
    /// Optional wallet for transaction signing
    pub wallet: Option<LocalWallet>,
}

impl EvmClient {
    /// Create a new EVM client from chain type
    ///
    /// # Params
    ///
    /// evm_type - The type of EVM chain to connect to
    ///
    pub async fn from_type(evm_type: EvmType) -> Result<Self, EvmClientError> {
        for (_index, url) in evm_type.rpc().iter().enumerate() {
            match Provider::<Http>::try_from(url.clone()) {
                Ok(p) => {
                    return Ok(Self {
                        provider: Arc::new(p),
                        evm_type: Some(evm_type),
                        wallet: None,
                    });
                }
                Err(_) => continue,
            }
        }
        return Err(EvmClientError::RpcError(format!(
            "All RPC links are invalid"
        )));
    }

    /// Create a new EVM client from a specific RPC URL
    ///
    /// # Params
    ///
    /// rpc_url - The RPC endpoint URL to connect to
    ///
    pub async fn from_rpc(rpc_url: &str) -> Result<Self, EvmClientError> {
        match Provider::<Http>::try_from(rpc_url) {
            Ok(p) => {
                return Ok(Self {
                    provider: Arc::new(p),
                    evm_type: None,
                    wallet: None,
                });
            }
            Err(e) => Err(EvmClientError::RpcError(format!("RPC Error: {:?}", e))),
        }
    }

    /// Create a new EVM client from chain type and private key
    ///
    /// # Params
    ///
    /// evm_type - The type of EVM chain to connect to
    /// private_key - The private key string for the wallet
    ///
    pub async fn from_wallet(evm_type: EvmType, private_key: &str) -> Result<Self, EvmClientError> {
        let mut client = Self::from_type(evm_type).await?;
        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| EvmClientError::RpcError(format!("Failed to parse private key: {}", e)))?;
        client.wallet = Some(wallet);
        Ok(client)
    }

    /// Create a new EVM client from RPC URL and private key
    ///
    /// # Params
    ///
    /// rpc_url - The RPC endpoint URL to connect to
    /// private_key - The private key string for the wallet
    ///
    pub async fn from_rpc_and_wallet(
        rpc_url: &str,
        private_key: &str,
    ) -> Result<Self, EvmClientError> {
        let mut client = Self::from_rpc(rpc_url).await?;
        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| EvmClientError::RpcError(format!("Failed to parse private key: {}", e)))?;
        client.wallet = Some(wallet);
        Ok(client)
    }

    /// heartbeat check
    pub async fn health(&self) -> Result<(), EvmClientError> {
        self.provider
            .get_chainid()
            .await
            .map_err(|e| EvmClientError::RpcError(format!("Health check failed: {}", e)))?;
        Ok(())
    }
}
