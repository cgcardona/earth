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

#[derive(Debug, Clone, Copy)]
pub struct Deployment {
    pub name: &'static str,
    pub bit: u8,
    pub start_time: u32,
    pub timeout: u32,
    pub activation: Option<u32>,
}
