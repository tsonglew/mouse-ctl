use windows::{
    core::GUID,
    Win32::Devices::DeviceAndDriverInstallation::{
        SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, DIGCF_PRESENT,
        HDEVINFO, SPDRP_DEVICEDESC, SPDRP_HARDWAREID, SP_DEVINFO_DATA,
    },
    Win32::Foundation::{GetLastError, ERROR_NO_MORE_ITEMS, WIN32_ERROR},
};

use crate::types::MouseDevice;
use crate::utils::{get_device_property, parse_vid_pid};

const SPDRP_FRIENDLYNAME: u32 = 0x0000000C;
const SPDRP_MFG: u32 = 0x0000000B;

pub fn get_mouse_guid() -> GUID {
    GUID::from_values(
        0x4d36e96f,
        0xe325,
        0x11ce,
        [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
    )
}

pub fn init_device_info(guid: &GUID) -> windows::core::Result<HDEVINFO> {
    unsafe { SetupDiGetClassDevsW(Some(guid), None, None, DIGCF_PRESENT) }
}

pub fn get_device_info(device_info: HDEVINFO, index: u32) -> Option<SP_DEVINFO_DATA> {
    let mut dev_info_data = SP_DEVINFO_DATA {
        cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
        ClassGuid: Default::default(),
        DevInst: 0,
        Reserved: 0,
    };

    let result = unsafe { SetupDiEnumDeviceInfo(device_info, index, &mut dev_info_data) };

    if !result.as_bool() {
        let error = unsafe { WIN32_ERROR(GetLastError().0) };
        if error == ERROR_NO_MORE_ITEMS {
            return None;
        }
        return None;
    }
    Some(dev_info_data)
}

fn get_hardware_ids(device_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA) -> Option<Vec<String>> {
    get_device_property(device_info, dev_info_data, SPDRP_HARDWAREID).map(|ids| {
        ids.split('\0')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    })
}

pub fn create_mouse_device(
    device_info: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Option<MouseDevice> {
    let hardware_ids = get_hardware_ids(device_info, dev_info_data)?;
    let (vid, pid) = hardware_ids.first().and_then(|id| parse_vid_pid(id))?;

    let description = get_device_property(device_info, dev_info_data, SPDRP_DEVICEDESC)
        .unwrap_or_else(|| String::from("Unknown"));
    let friendly_name = get_device_property(device_info, dev_info_data, SPDRP_FRIENDLYNAME)
        .unwrap_or_else(|| String::from("Unknown"));
    let manufacturer = get_device_property(device_info, dev_info_data, SPDRP_MFG)
        .unwrap_or_else(|| String::from("Unknown"));

    Some(MouseDevice::new(
        vid,
        pid,
        description,
        friendly_name,
        manufacturer,
        hardware_ids,
        dev_info_data.DevInst,
    ))
}

pub fn cleanup_device_info(device_info: HDEVINFO) {
    unsafe {
        SetupDiDestroyDeviceInfoList(device_info);
    }
}
