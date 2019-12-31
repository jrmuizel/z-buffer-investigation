#![cfg_attr(
    not(any(
        feature = "vulkan",
        feature = "dx11",
        feature = "dx12",
        feature = "metal"
    )),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as back;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

use std::{fs, ptr, slice, str::FromStr};

use gfx_hal::{adapter::MemoryType, buffer, command, memory, pool, prelude::*, pso, image::*, format::*};


#[cfg(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal"
))]
fn main() {

    let instance = back::Instance::create("gfx-rs compute", 1)
        .expect("Failed to create an instance!");

    let adapter = instance
        .enumerate_adapters()
        .into_iter()
        .find(|a| {
            a.queue_families
                .iter()
                .any(|family| family.queue_type().supports_compute())
        })
        .expect("Failed to find a GPU with compute support!");

    let memory_properties = adapter.physical_device.memory_properties();
    let family = adapter
        .queue_families
        .iter()
        .find(|family| family.queue_type().supports_compute())
        .unwrap();
    let mut gpu = unsafe {
        adapter
            .physical_device
            .open(&[(family, &[1.0])], gfx_hal::Features::empty())
            .unwrap()
    };
    let device = &gpu.device;
    println!("{:?}", adapter.info);
    println!("{:#?}", adapter.physical_device.memory_properties());

    for d in &[512, 1024, 2048] {
        println!("dim {}", d);
        for (f, pixel_size) in &[(Format::D16Unorm, 2), (Format::D16UnormS8Uint, 3), (Format::D32Sfloat, 4), (Format::D24UnormS8Uint, 4), (Format::D32SfloatS8Uint, 5)] {
            unsafe {
                let prop = adapter.physical_device.image_format_properties(*f, 2, Tiling::Optimal, Usage::DEPTH_STENCIL_ATTACHMENT,
                                              ViewCapabilities::empty());
                if prop.is_none() {
                    continue;
                }
                let img = device.create_image(Kind::D2(*d, *d, 1, 1), 1, *f, Tiling::Optimal, Usage::DEPTH_STENCIL_ATTACHMENT,
                                              ViewCapabilities::empty());
                if let Ok(img) = img {
                    let r = device.get_image_requirements(&img);
                    let extra = r.size - (*d**d**pixel_size) as u64;
                    println!("{:?} req: {:?} extra {} {}", f, r, extra, extra as f64 / (*d**d) as f64);
                }
            }
        }
    }

}

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal"
)))]
fn main() {
    println!("You need to enable one of the next-gen API feature (vulkan, dx12, metal) to run this example.");
}

