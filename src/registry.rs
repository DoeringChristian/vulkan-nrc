use std::sync::Arc;

use crevice::glsl::Glsl;
use crevice::std140::{AsStd140, Std140, WriteStd140};
use screen_13::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct BufferIndex(u32);

unsafe impl Std140 for BufferIndex {
    const ALIGNMENT: usize = 4;
}

unsafe impl Glsl for BufferIndex {
    const NAME: &'static str = "uint";
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct ImageIndex(u32);

unsafe impl Std140 for ImageIndex {
    const ALIGNMENT: usize = 4;
}

unsafe impl Glsl for ImageIndex {
    const NAME: &'static str = "uint";
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct CallableIndex(u32);

unsafe impl Std140 for CallableIndex {
    const ALIGNMENT: usize = 4;
}

unsafe impl Glsl for CallableIndex {
    const NAME: &'static str = "uint";
}

pub struct Registry {
    pub device: Arc<Device>,
    pub cache: HashPool,
    pub rgraph: RenderGraph,
    pub buffers: Vec<Arc<Buffer>>,
    pub images: Vec<Arc<Image>>,
}

impl Registry {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            device: device.clone(),
            cache: HashPool::new(&device),
            rgraph: RenderGraph::new(),
            buffers: Default::default(),
            images: Default::default(),
        }
    }
    pub fn add_buffer(&mut self, buffer: &Arc<Buffer>) -> BufferIndex {
        let index = BufferIndex(self.buffers.len() as u32);
        self.buffers.push(buffer.clone());
        index
    }
    pub fn add_image(&mut self, image: &Arc<Image>) -> ImageIndex {
        let index = ImageIndex(self.buffers.len() as u32);
        self.images.push(image.clone());
        index
    }
    pub fn upload_as_std140(
        &mut self,
        data: impl AsStd140,
        // rgraph: &mut RenderGraph,
    ) -> BufferIndex {
        let data = data.as_std140();
        let size = data.std140_size() as u64;
        let mut tmp = self
            .cache
            .lease(BufferInfo::host_mem(
                size,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ))
            .unwrap();

        let slice = Buffer::mapped_slice_mut(&mut tmp);
        let mut writer = crevice::std140::Writer::new(slice);
        writer.write(&data.as_std140()).unwrap();

        let tmp = self.rgraph.bind_node(tmp);

        let buf = Arc::new(
            Buffer::create(
                &self.device,
                BufferInfo::device_mem(size, vk::BufferUsageFlags::STORAGE_BUFFER),
            )
            .unwrap(),
        );

        let buf_node = self.rgraph.bind_node(buf.clone());

        self.rgraph.copy_buffer(tmp, buf_node);

        return self.add_buffer(&buf);
    }
    pub fn add_callable(source: &'static str) -> CallableIndex {
        todo!()
    }
}

pub trait Register {
    fn register(&self, registry: &mut Registry) -> BufferIndex;
}
