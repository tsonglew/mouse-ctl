use windows::{
    core::GUID,
    Win32::Devices::DeviceAndDriverInstallation::{
        SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
        SetupDiGetDeviceRegistryPropertyW, DIGCF_PRESENT, HDEVINFO, SPDRP_DEVICEDESC,
        SPDRP_HARDWAREID, SP_DEVINFO_DATA,
    },
    Win32::Foundation::{GetLastError, ERROR_NO_MORE_ITEMS, WIN32_ERROR},
};

// Windows device property constants
const SPDRP_FRIENDLYNAME: u32 = 0x0000000C;
const SPDRP_MFG: u32 = 0x0000000B;

#[derive(Debug)]
struct MouseDevice {
    vendor_id: String,
    product_id: String,
    device_description: String,
    friendly_name: String,
    manufacturer: String,
    hardware_ids: Vec<String>,
    device_instance_id: u32,
}

impl MouseDevice {
    fn new(
        vendor_id: String,
        product_id: String,
        device_description: String,
        friendly_name: String,
        manufacturer: String,
        hardware_ids: Vec<String>,
        device_instance_id: u32,
    ) -> Self {
        Self {
            vendor_id,
            product_id,
            device_description,
            friendly_name,
            manufacturer,
            hardware_ids,
            device_instance_id,
        }
    }

    fn display(&self) {
        println!("Mouse Device Information:");
        println!("-------------------------");
        println!("Vendor ID: {}", self.vendor_id);
        println!("Product ID: {}", self.product_id);
        println!("Description: {}", self.device_description);
        println!("Friendly Name: {}", self.friendly_name);
        println!("Manufacturer: {}", self.manufacturer);
        println!("Hardware IDs:");
        for id in &self.hardware_ids {
            println!("  - {}", id);
        }
        println!("Device Instance ID: {}", self.device_instance_id);
        println!("-------------------------\n");
    }
}

fn get_mouse_guid() -> GUID {
    GUID::from_values(
        0x4d36e96f,
        0xe325,
        0x11ce,
        [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
    )
}

fn init_device_info(guid: &GUID) -> windows::core::Result<HDEVINFO> {
    unsafe { SetupDiGetClassDevsW(Some(guid), None, None, DIGCF_PRESENT) }
}

fn create_device_info_data() -> SP_DEVINFO_DATA {
    SP_DEVINFO_DATA {
        cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
        ClassGuid: Default::default(),
        DevInst: 0,
        Reserved: 0,
    }
}

fn get_device_info(device_info: HDEVINFO, index: u32) -> Option<SP_DEVINFO_DATA> {
    let mut dev_info_data = create_device_info_data();
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

fn get_device_property(device_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA, property: u32) -> Option<String> {
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

fn get_hardware_ids(device_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA) -> Option<Vec<String>> {
    get_device_property(device_info, dev_info_data, SPDRP_HARDWAREID)
        .map(|ids| ids.split('\0').filter(|s| !s.is_empty()).map(String::from).collect())
}

fn parse_vid_pid(hardware_id: &str) -> Option<(String, String)> {
    if let (Some(vid_pos), Some(pid_pos)) = (hardware_id.find("VID_"), hardware_id.find("PID_")) {
        let vid = hardware_id[vid_pos + 4..vid_pos + 8].to_string();
        let pid = hardware_id[pid_pos + 4..pid_pos + 8].to_string();
        Some((vid, pid))
    } else {
        None
    }
}

fn create_mouse_device(device_info: HDEVINFO, dev_info_data: &SP_DEVINFO_DATA) -> Option<MouseDevice> {
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

fn main() -> windows::core::Result<()> {
    let mouse_guid = get_mouse_guid();
    let device_info = init_device_info(&mouse_guid)?;

    let mut index = 0;
    while let Some(dev_info_data) = get_device_info(device_info, index) {
        if let Some(mouse_device) = create_mouse_device(device_info, &dev_info_data) {
            mouse_device.display();
        }
        index += 1;
    }

    unsafe {
        SetupDiDestroyDeviceInfoList(device_info);
    }

    Ok(())
}
