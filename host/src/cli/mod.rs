use clap::Parser;

/// The arguments for configuring the chain data provider.
#[derive(Debug, Clone, Parser)]
pub struct ProviderArgs {
    /// The rpc url used to fetch data about the block
    #[clap(long, default_value = "https://beacon.mekong.ethpandaops.io/")]
    pub rpc_url: Option<String>,
}
