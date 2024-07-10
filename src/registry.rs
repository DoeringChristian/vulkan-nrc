use std::sync::Arc;

use crevice::glsl::Glsl;
use crevice::std140::{AsStd140, Std140, WriteStd140};
use indexmap::IndexMap;
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
    pub callables: IndexMap<&'static [u32], Shader>,
}

impl Registry {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            device: device.clone(),
            cache: HashPool::new(&device),
            rgraph: RenderGraph::new(),
            buffers: Default::default(),
            images: Default::default(),
            callables: Default::default(),
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
    pub fn upload_buffer(&mut self, data: &[u8]) -> BufferIndex {
        let size = data.len() as u64;
        let mut tmp = self
            .cache
            .lease(BufferInfo::host_mem(
                size,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ))
            .unwrap();

        Buffer::copy_from_slice(&mut tmp, 0, data);

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
    pub fn upload_std140(&mut self, data: impl AsStd140) -> BufferIndex {
        let mut buf = vec![];
        let mut writer = crevice::std140::Writer::new(&mut buf);
        writer.write(&data.as_std140()).unwrap();
        self.upload_buffer(&buf)
    }
    pub fn upload_std140_slice<T: AsStd140>(&mut self, data: &[T]) -> BufferIndex {
        let mut buf = vec![];
        let mut writer = crevice::std140::Writer::new(&mut buf);
        for data in data.into_iter() {
            writer.write(data).unwrap();
        }
        self.upload_buffer(&buf)
    }
    pub fn add_callable(&mut self, source: &'static [u32]) -> CallableIndex {
        let entry = self.callables.entry(source);
        let index = match entry {
            indexmap::map::Entry::Occupied(entry) => entry.index(),
            indexmap::map::Entry::Vacant(entry) => {
                let index = entry.index();
                entry.insert(Shader::new_callable(source).build());
                index
            }
        };
        CallableIndex(index as _)
    }
}

pub trait Register {
    fn register(&self, registry: &mut Registry) -> BufferIndex;
}
