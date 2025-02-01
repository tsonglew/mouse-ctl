#[derive(Debug)]
pub struct MouseDevice {
    pub vendor_id: String,
    pub product_id: String,
    pub device_description: String,
    pub friendly_name: String,
    pub manufacturer: String,
    pub hardware_ids: Vec<String>,
    pub device_instance_id: u32,
}

impl MouseDevice {
    pub fn new(
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

    pub fn display(&self) {
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
