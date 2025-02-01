use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiGetDeviceRegistryPropertyW, HDEVINFO, SP_DEVINFO_DATA,
};

pub fn get_device_property(device_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA, property: u32) -> Option<String> {
    let mut buffer = [0u8; 512];
    let mut required_size: u32 = 0;
    
    let success = unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            device_info,
            dev_info_data,
            property,
            Some(&mut required_size as *mut u32),
            Some(&mut buffer),
            Some(&mut required_size as *mut u32),
        )
    };

    if success.as_bool() {
        let slice = unsafe {
            std::slice::from_raw_parts(
                buffer.as_ptr() as *const u16,
                required_size as usize / 2,
            )
        };
        Some(String::from_utf16_lossy(slice))
    } else {
        None
    }
}

pub fn parse_vid_pid(hardware_id: &str) -> Option<(String, String)> {
    if let (Some(vid_pos), Some(pid_pos)) = (hardware_id.find("VID_"), hardware_id.find("PID_")) {
        let vid = hardware_id[vid_pos + 4..vid_pos + 8].to_string();
        let pid = hardware_id[pid_pos + 4..pid_pos + 8].to_string();
        Some((vid, pid))
    } else {
        None
    }
}
