use std::process::Command;

use async_trait::async_trait;

use crate::operations::calculate::Calculate;

pub struct GetWifiName;

impl GetWifiName {
    pub fn new() -> Self {
        GetWifiName {}
    }

    pub async fn get_wifi_name(&self) -> String {
        if cfg!(target_os = "linux") {
            LinuxWifiInfoProvider.get_wifi_name().await
        } else if cfg!(target_os = "windows") {
            WindowsWifiInfoProvider.get_wifi_name().await
        } else if cfg!(target_os = "macos") {
            MacOSWifiInfoProvider.get_wifi_name().await
        } else {
            panic!("Unsupported platform");
        }
    }
}

#[async_trait]
impl Calculate for GetWifiName {
    async fn calculate(&self) -> String {
        self.get_wifi_name().await
    }
}

// TODO What if the user is connected to multiple networks? What if the user is connected to a wired network?
#[async_trait]
trait WifiInfoProvider {
    async fn get_wifi_name(&self) -> String;
}

struct LinuxWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for LinuxWifiInfoProvider {
    async fn get_wifi_name(&self) -> String {
        let output = std::process::Command::new("iwconfig")
            .output()
            .expect("Failed to execute iwconfig");

        let output_str = match std::str::from_utf8(&output.stdout) {
            Ok(s) => s,
            Err(_) => return "__".into(),
        };

        for line in output_str.lines() {
            if line.contains("ESSID") {
                if let Some(name) = line.split_whitespace().nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }

        "__".into() // Return a default value
    }
}

struct WindowsWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for WindowsWifiInfoProvider {
    async fn get_wifi_name(&self) -> String {
        // Implement Windows-specific code here
        unimplemented!("Windows implementation not provided");
    }
}

struct MacOSWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for MacOSWifiInfoProvider {
    async fn get_wifi_name(&self) -> String {
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .arg("-I")
            .output()
            .expect("Failed to execute airport");

        let output_str = match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(_) => return "__".into(),
        };

        for line in output_str.lines() {
            if line.contains(" SSID: ") {
                let ssid = line.split(':').nth(1).unwrap();
                return ssid.trim().to_string();
            }
        }

        "__".into()
    }
}