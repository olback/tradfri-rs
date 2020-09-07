// https://github.com/home-assistant-libs/pytradfri/blob/master/pytradfri/const.py

use {
    serde::Deserialize,
    crate::device_worker::DeviceWorker
};

pub mod light;

#[derive(Debug, Deserialize)]
pub(crate) struct DeviceInfo {
    #[serde(rename = "0")]
    pub mfr: String,
    #[serde(rename = "1")]
    pub device_name: String,
    #[serde(rename = "3")]
    pub version: String
}

#[derive(Debug, Deserialize)]
struct BasicDevice {
    #[serde(rename = "5750")]
    pub device_type: u32
}

#[derive(Debug)]
pub enum Device {
    RemoteControl, // 0
    Light(light::Light), // 2
    // Panel,
    // Door,
    // RecessedSpotlight,
    // Driver,
    // ControlOutlet,
    // SquareCeilingWallLamp,
    // RoundCeilingWallLamp,
    // SignalRepeater,
    // Blind,
    // WirelessDimmer,
    // MotionSensor,
    // OnOffSwitchDimmer,
    // OpenCloseRemote
}

impl Device {

    pub fn new(worker: DeviceWorker, bytes: &[u8]) -> crate::Result<Self> {

        let basic_device: BasicDevice = serde_json::from_slice(bytes)?;

        match basic_device.device_type {
            0 => Ok(Self::RemoteControl),
            2 => Ok(Self::Light(light::Light::new(worker, bytes)?)),
            _ => Err(crate::Error::new("Unsupported device"))
        }

    }

}
