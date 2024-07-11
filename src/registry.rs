use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use crevice::glsl::Glsl;
use crevice::std140::{self, AsStd140, Std140, WriteStd140};
use indexmap::IndexMap;
use screen_13::prelude::*;

pub const fn align_up(val: usize, base: usize) -> usize {
    div_round_up(val, base) * base
}
pub const fn div_round_up(val: usize, divisor: usize) -> usize {
    (val + divisor - 1) / divisor
}

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
pub struct BufferOffset(u32);

unsafe impl Std140 for BufferOffset {
    const ALIGNMENT: usize = 4;
}

unsafe impl Glsl for BufferOffset {
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

pub enum BufferField {
    Buffer(Arc<Buffer>),
    Vec(Vec<u8>),
}

pub struct Registry {
    pub device: Arc<Device>,
    pub cache: HashPool,
    pub rgraph: RenderGraph,
    pub buffers: Vec<BufferField>,
    pub images: Vec<Arc<Image>>,
    pub data_buffers: IndexMap<usize, BufferIndex>,
    pub callables: IndexMap<&'static [u32], Shader>,
}

pub fn upload_data(
    data: &[u8],
    cache: &mut HashPool,
    rgraph: &mut RenderGraph,
    device: &Arc<Device>,
) -> Arc<Buffer> {
    let size = data.len() as u64;
    let mut tmp = cache
        .lease(BufferInfo::host_mem(
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
        ))
        .unwrap();

    Buffer::copy_from_slice(&mut tmp, 0, data);

    let tmp = rgraph.bind_node(tmp);

    let buf = Arc::new(
        Buffer::create(
            &device,
            BufferInfo::device_mem(size, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    );

    let buf_node = rgraph.bind_node(buf.clone());

    rgraph.copy_buffer(tmp, buf_node);

    buf
}

impl Registry {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            device: device.clone(),
            cache: HashPool::new(&device),
            rgraph: RenderGraph::new(),
            buffers: Default::default(),
            images: Default::default(),
            data_buffers: Default::default(),
            callables: Default::default(),
        }
    }
    pub fn add_buffer(&mut self, buffer: &Arc<Buffer>) -> BufferIndex {
        let index = BufferIndex(self.buffers.len() as u32);
        self.buffers.push(BufferField::Buffer(buffer.clone()));
        index
    }
    pub fn add_vec(&mut self, vec: Vec<u8>) -> BufferIndex {
        let index = BufferIndex(self.buffers.len() as u32);
        self.buffers.push(BufferField::Vec(vec));
        index
    }
    pub fn add_image(&mut self, image: &Arc<Image>) -> ImageIndex {
        let index = ImageIndex(self.buffers.len() as u32);
        self.images.push(image.clone());
        index
    }
    pub fn upload_buffer(&mut self, data: &[u8]) -> Arc<Buffer> {
        upload_data(data, &mut self.cache, &mut self.rgraph, &mut self.device)
    }
    pub fn upload_add_buffer(&mut self, data: &[u8]) -> BufferIndex {
        let buffer = self.upload_buffer(data);
        self.add_buffer(&buffer)
    }
    pub fn upload_std140<T: AsStd140>(&mut self, data: T) -> (BufferIndex, BufferOffset) {
        let alignment = T::Output::ALIGNMENT;
        let size = data.std140_size();
        let size = align_up(size, alignment);

        let entry = self.data_buffers.entry(size);
        let key = match entry {
            indexmap::map::Entry::Occupied(entry) => {
                let buffer_index = *entry.get();
                let buffer_field = &mut self.buffers[buffer_index.0 as usize];
                let offset = match buffer_field {
                    BufferField::Vec(buffer) => {
                        let offset = buffer.len();
                        let mut writer = std140::Writer::new(buffer);
                        data.write_std140(&mut writer).unwrap();
                        offset
                    }
                    _ => todo!(),
                };
                assert!(offset % size == 0);
                let offset = offset / size;
                (buffer_index, BufferOffset(offset as u32))
            }
            indexmap::map::Entry::Vacant(_) => {
                let buffer_index = self.add_vec(vec![]);
                // Unfortunate second lookup
                self.data_buffers.insert(size, buffer_index);
                (buffer_index, BufferOffset(0))
            }
        };
        key
    }
    pub fn upload_std140_slice<T: AsStd140>(&mut self, data: &[T]) -> BufferIndex {
        let mut buf = vec![];
        let mut writer = crevice::std140::Writer::new(&mut buf);
        for data in data.into_iter() {
            writer.write(data).unwrap();
        }
        self.upload_add_buffer(&buf)
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
