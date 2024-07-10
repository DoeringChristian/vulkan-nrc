use crevice::glsl::Glsl;
use crevice::std140::Std140;
use screen_13::prelude::Buffer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct DeviceAddress(u64);

impl DeviceAddress {
    pub fn of(buffer: &Buffer) -> Self {
        Self(Buffer::device_address(&buffer) as _)
    }
}

impl From<u64> for DeviceAddress {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

unsafe impl Glsl for DeviceAddress {
    const NAME: &'static str = "uvec2";
}

unsafe impl Std140 for DeviceAddress {
    const ALIGNMENT: usize = 8;
}
