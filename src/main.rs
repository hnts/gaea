#![no_std] 
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gaea::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use gaea::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use gaea::allocator;
	use gaea::memory::{self, BootInfoFrameAllocator};

	println!("Hello World{}", "!");
	gaea::init();

	let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator)
		.expect("heap initialization failed");

	let heap_value = Box::new(41);
	println!("heap_value as {:p}", heap_value);

	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
	println!("vec at {:p}", vec.as_slice());

	let reference_counted = Rc::new(vec![1,2,3]);
	let cloned_reference = reference_counted.clone();
	println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));


	#[cfg(test)]
	test_main();

    println!("It did not crash!");
	gaea::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
    gaea::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gaea::test_panic_handler(info)
}