// Adapted from https://github.com/sotrh/learn-wgpu
// by Benjamin Hansen, licensed under the MIT license
use crate::rect::Rect;
use crate::util::size_of_slice;
use crate::vertex::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    current_quad: u32,
}

impl QuadBufferBuilder {
    pub fn new() -> Self {
        Self {
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            current_quad: 0,
        }
    }

    pub fn push_rect(self, rect: &Rect) -> Self {
        let coords = rect.top_left_coords;
        self.push_quad(
            coords.x,
            coords.y - rect.height,
            coords.x + rect.width,
            coords.y,
            rect.color,
        )
    }

    pub fn push_quad(
        mut self,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        color: [f32; 3],
    ) -> Self {
        self.vertex_data.extend(&[
            Vertex {
                position: (min_x, min_y).into(),
                color,
            },
            Vertex {
                position: (max_x, min_y).into(),
                color,
            },
            Vertex {
                position: (max_x, max_y).into(),
                color,
            },
            Vertex {
                position: (min_x, max_y).into(),
                color,
            },
        ]);
        self.index_data.extend(&[
            self.current_quad * 4,
            self.current_quad * 4 + 1,
            self.current_quad * 4 + 2,
            self.current_quad * 4,
            self.current_quad * 4 + 2,
            self.current_quad * 4 + 3,
        ]);
        self.current_quad += 1;
        self
    }

    pub fn build(self, device: &wgpu::Device) -> (StagingBuffer, StagingBuffer, u32) {
        (
            StagingBuffer::new(device, &self.vertex_data),
            StagingBuffer::new(device, &self.index_data),
            self.index_data.len() as u32,
        )
    }
}

pub struct RectBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_rects: u32,
}

pub fn create_rect_buffers(
    gpu_device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
) -> RectBuffers {
    // Test Rectangles
    let test_rect_1 = Rect {
        top_left_coords: (-0.2, 0.6).into(),
        width: 0.1,
        height: 0.5,
        color: [0.0, 0.0, 1.0],
    };
    let test_rect_2 = Rect {
        top_left_coords: (-0.5, 0.0).into(),
        width: 0.5,
        height: 0.5,
        color: [0.0, 1.0, 0.0],
    };
    let test_rect_3 = Rect {
        top_left_coords: (0.3, 0.3).into(),
        width: 0.6,
        height: 0.1,
        color: [1.0, 0.0, 0.0],
    };

    let vertex_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: Vertex::SIZE * 4 * 3,
        usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    let u32_size = std::mem::size_of::<u32>() as wgpu::BufferAddress;

    let index_buffer = gpu_device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u32_size * 6 * 3,
        usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    let num_rects = {
        let (stg_vertex, stg_index, num_indices) = QuadBufferBuilder::new()
            .push_rect(&test_rect_1)
            .push_rect(&test_rect_2)
            .push_rect(&test_rect_3)
            .build(&gpu_device);

        stg_vertex.copy_to_buffer(encoder, &vertex_buffer);
        stg_index.copy_to_buffer(encoder, &index_buffer);
        num_indices
    };

    RectBuffers {
        vertex_buffer,
        index_buffer,
        num_rects,
    }
}

pub struct StagingBuffer {
    buffer: wgpu::Buffer,
    size: wgpu::BufferAddress,
}

impl StagingBuffer {
    pub fn new<T: bytemuck::Pod + Sized>(device: &wgpu::Device, data: &[T]) -> StagingBuffer {
        StagingBuffer {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsage::COPY_SRC,
                label: Some("Staging Buffer"),
            }),
            size: size_of_slice(data) as wgpu::BufferAddress,
        }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, other: &wgpu::Buffer) {
        encoder.copy_buffer_to_buffer(&self.buffer, 0, other, 0, self.size)
    }
}