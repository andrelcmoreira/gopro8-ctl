use std::io::Error;

use crate::api::bluetooth::*;

use crate::data::factory_info::FactoryInfo;
use crate::data::status_info::StatusInfo;
use crate::data::wifi_info::WifiInfo;

// TODO: handle errors
pub async fn get_wifi_info() -> Result<WifiInfo, Error> {
    let adapter = get_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = WifiInfo {
        wifi_ssid: get_wifi_ssid(&cam).await,
        wifi_password: get_wifi_password(&cam).await
    };

    Ok(info)
}

// TODO: handle errors
pub async fn get_factory_info() -> Result<FactoryInfo, Error> {
    let adapter = get_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = FactoryInfo {
        hw_revision: get_hw_revision(&cam).await,
        fw_revision: get_fw_revision(&cam).await,
        sw_revision: get_sw_revision(&cam).await,
        serial_number: get_serial_number(&cam).await,
        model_number: get_model_number(&cam).await,
        manufacturer_name: get_manufacturer_name(&cam).await
    };

    Ok(info)
}

// TODO: handle errors
pub async fn get_status_info() -> Result<StatusInfo, Error> {
    let adapter = get_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = StatusInfo {
        battery_level: get_battery_level(&cam).await,
        tx_power_level: get_tx_power_level(&cam).await
    };

    Ok(info)
}
