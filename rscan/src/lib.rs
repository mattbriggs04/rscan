use objc2_core_wlan::CWWiFiClient;
use std::fmt;

#[derive(Clone, Debug)]
pub struct WifiNetInfo {
    pub ssid: Option<String>,
    pub bssid: Option<String>,
    pub signal: Option<isize>,
}

impl fmt::Display for WifiNetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ssid = self.ssid.as_deref().unwrap_or("<hidden>");
        let bssid = self.bssid.as_deref().unwrap_or("MAC Unknown");
        match self.signal {
            Some(signal) => write!(f, "{} | {} | {} dBm", ssid, bssid, signal),
            None => write!(f, "{} | {} | <unknown> dBm", ssid, bssid),
        }
    }
}
pub struct NetworkList {
    networks: Vec<WifiNetInfo>,
}

impl NetworkList {
    // Create a network list by scanning all networks in the area
    pub fn scan() -> Result<Self, String> {
        unsafe {
            let client = CWWiFiClient::sharedWiFiClient();
            let interface = match client.interface().ok_or("rscan error: could not find default interface") {
                Ok(value) => value,
                Err(e) => return Err(e.to_string()),
            };

            let networks = interface.scanForNetworksWithName_error(None)
                .map_err(|e| format!("rscan error: failed scanning\n({})", e.to_string()))?
                .to_vec();

            let mut wifi_vec = Vec::new();
            for n in networks {
                wifi_vec.push(WifiNetInfo {
                    ssid: n.ssid().map(|s| s.to_string()),
                    bssid: n.bssid().map(|s| s.to_string()),
                    signal: Some(n.rssiValue()),
                });
            }

            Ok(NetworkList { networks: wifi_vec })
        }
    }

    pub fn search_mac(self, pattern: &str) -> Self {
        let filtered = self.networks
            .into_iter()
            .filter(|w| w.bssid.as_ref().is_some_and(|mac| mac.contains(pattern)))
            .collect();

        NetworkList { networks: filtered }
    }

    pub fn sort_closest(mut self) -> Self {
        self.networks.sort_by(|a, b| {
            b.signal.unwrap_or(isize::MIN).cmp(&a.signal.unwrap_or(isize::MIN))
        });
        self
    }

    pub fn into_vec(self) -> Vec<WifiNetInfo> {
        self.networks
    }
}