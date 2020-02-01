#![no_std]
#![no_main]
#![feature(panic_info_message)]

use gaea::{serial_print, serial_println, QemuExitCode, exit_qemu};
use core::{
    fmt::{self, Write},
    panic::PanicInfo,
};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 18;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("panic_handler... ");
    panic!(MESSAGE);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_message(info);
    check_location(info);

    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn fail(error: &str) -> ! {
    serial_println!("[failed]");
    serial_println!("{}",error);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn check_location(info: &PanicInfo) {
    let location = info.location().unwrap_or_else(|| fail("no location"));
    if location.file() != file!() {
        fail("file name wrong");
    }
    if location.line() != PANIC_LINE {
        fail("file line wrong")
    }
}

struct CompareMessage {
    equals: bool,
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s == MESSAGE {
            self.equals = true;
        }
        Ok(())
    }
}

fn check_message(info: &PanicInfo) {
    let message = info.message().unwrap_or_else(|| fail("no message"));
    let mut compare_message = CompareMessage { equals: false };
    write!(&mut compare_message, "{}", message)
        .unwrap_or_else(|_| fail("write failed"));
    if !compare_message.equals {
        fail("message not equal to expected message");
    }
}