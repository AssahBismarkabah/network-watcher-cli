use std::process::Command;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;

use crate::operations::calculate::Calculate;

pub const MISSING_INFO: &str = "__";

pub struct GetWifiName;

impl GetWifiName {
    pub fn new() -> Self {
        GetWifiName {}
    }

    pub async fn get_wifi_name(&self) -> Result<String> {
        let res = self.helper().await?;
        Ok(format!("\"{}\"", res))
    }

    pub async fn helper(&self) -> Result<String> {
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
        self.get_wifi_name()
            .await.unwrap()
    }
}

// TODO What if the user is connected to multiple networks? What if the user is connected to a wired network?
#[async_trait]
trait WifiInfoProvider {
    async fn get_wifi_name(&self) -> Result<String>;
}

struct LinuxWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for LinuxWifiInfoProvider {
    async fn get_wifi_name(&self) -> Result<String> {
        let output = Command::new("iwconfig")
            .output()
            .with_context(|| "Failed to execute iwconfig")?;

        let output_str = match std::str::from_utf8(&output.stdout) {
            Ok(s) => s,
            Err(_) => return Ok(MISSING_INFO.into()),
        };

        for line in output_str.lines() {
            if line.contains("ESSID") {
                if let Some(name) = line.split_whitespace().nth(1) {
                    return Ok(name.trim_matches('"').to_string());
                }
            }
        }

        Ok(MISSING_INFO.into()) // Return a default value
    }
}

struct WindowsWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for WindowsWifiInfoProvider {
    async fn get_wifi_name(&self) -> Result<String> {
        // Implement Windows-specific code here
        bail!("Windows implementation not provided");
    }
}

struct MacOSWifiInfoProvider;

#[async_trait]
impl WifiInfoProvider for MacOSWifiInfoProvider {
    async fn get_wifi_name(&self) -> Result<String> {
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .arg("-I")
            .output()
            .with_context(|| "Failed to execute airport")?;

        let output_str = match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(_) => return Ok(MISSING_INFO.into()),
        };

        for line in output_str.lines() {
            if line.contains(" SSID: ") {
                let ssid = line.split(':').nth(1).unwrap();
                return Ok(ssid.trim().to_string());
            }
        }

        Ok(MISSING_INFO.into())
    }
}