use clap;
use network::Network;
pub struct RPC {}

impl RPC {
    pub fn parse_rpc_config(
        network: Network,
        matches: &clap::ArgMatches,
    ) -> Result<HttpConfig, String> {
        let mut config = HttpConfig::with_port(network.rpc_port());
        config.enabled = !matches.is_present("no-jsonrpc");
        if !config.enabled {
            return Ok(config);
        }

        // if let Some(apis) = matches.value_of("jsonrpc-apis") {
        //     config.apis = ApiSet::List(
        //         vec![apis.parse().map_err(|_| "Invalid APIs".to_owned())?]
        //             .into_iter()
        //             .collect(),
        //     );
        // }
        // if let Some(port) = matches.value_of("jsonrpc-port") {
        //     config.port = port
        //         .parse()
        //         .map_err(|_| "Invalid JSON RPC port".to_owned())?;
        // }
        // if let Some(interface) = matches.value_of("jsonrpc-interface") {
        //     config.interface = interface.to_owned();
        // }
        // if let Some(cors) = matches.value_of("jsonrpc-cors") {
        //     config.cors = Some(vec![cors
        //         .parse()
        //         .map_err(|_| "Invalid JSON RPC CORS".to_owned())?]);
        // }
        // if let Some(hosts) = matches.value_of("jsonrpc-hosts") {
        //     config.hosts = Some(vec![hosts
        //         .parse()
        //         .map_err(|_| "Invalid JSON RPC hosts".to_owned())?]);
        // }

        Ok(config)
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpConfig {
    pub enabled: bool,
    pub interface: String,
    pub port: u16,
    // pub apis: ApiSet,
    pub cors: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
}

impl HttpConfig {
    pub fn with_port(port: u16) -> Self {
        HttpConfig {
            enabled: true,
            interface: "127.0.0.1".into(),
            port: port,
            // apis: ApiSet::default(),
            cors: None,
            hosts: Some(Vec::new()),
        }
    }
}
