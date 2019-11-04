use network::Network;
#[derive(Debug, Clone)]
pub struct ConsensusParams {
    pub network: Network,
    pub bip16_time: u32,
    pub bip34_height: u32,
    pub bip65_height: u32,
    pub bip66_height: u32,
    pub rule_change_activation_threshold: u32,
    pub miner_confirmation_window: u32,
    pub csv_deployment: Option<Deployment>,
}

impl ConsensusParams {
    pub fn return_params(network: Network) -> Self {
        match network {
            Network::Mainnet => ConsensusParams {
                network: network,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 227931, // 000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8
                bip65_height: 388381, // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
                bip66_height: 363725, // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
                rule_change_activation_threshold: 1916, // 95%
                miner_confirmation_window: 2016,
                csv_deployment: Some(Deployment {
                    name: "csv",
                    bit: 0,
                    start_time: 1462060800,
                    timeout: 1493596800,
                    activation: Some(419328),
                }),
            },
            Network::Testnet => ConsensusParams {
                network: network,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 21111, // 0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8
                bip65_height: 581885, // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
                bip66_height: 330776, // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
                rule_change_activation_threshold: 1512, // 75%
                miner_confirmation_window: 2016,
                csv_deployment: Some(Deployment {
                    name: "csv",
                    bit: 0,
                    start_time: 1456790400,
                    timeout: 1493596800,
                    activation: Some(770112),
                }),
            },
            Network::Regtest => ConsensusParams {
                network: network,
                bip16_time: 1333238400,  // Apr 1 2012
                bip34_height: 100000000, // not activated on regtest
                bip65_height: 1351,
                bip66_height: 1251,                    // used only in rpc tests
                rule_change_activation_threshold: 108, // 75%
                miner_confirmation_window: 144,
                csv_deployment: Some(Deployment {
                    name: "csv",
                    bit: 0,
                    start_time: 0,
                    timeout: 0,
                    activation: Some(0),
                }),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Deployment {
    pub name: &'static str,
    pub bit: u8,
    pub start_time: u32,
    pub timeout: u32,
    pub activation: Option<u32>,
}
