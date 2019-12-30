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

    for f in &[Format::D16Unorm, Format::D16UnormS8Uint, Format::D32Sfloat, Format::D24UnormS8Uint, Format::D32SfloatS8Uint] {
        unsafe {
            let img = device.create_image(Kind::D2(1024, 1024, 1, 1), 1, *f, Tiling::Optimal, Usage::DEPTH_STENCIL_ATTACHMENT,
                                          ViewCapabilities::empty());
            if let Ok(img) = img {
                let r = device.get_image_requirements(&img);
                println!("{:?} req: {:?}", f, r);
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
