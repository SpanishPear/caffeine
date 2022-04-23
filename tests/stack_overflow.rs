// in tests/stack_overflow.rs
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use caffeine::drivers::qemu::{exit_qemu, QemuExitCode};
use caffeine::{kprint, kprintln};
use bootloader::{entry_point, BootInfo};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(caffeine::drivers::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

entry_point!(kernel_main);

pub fn kernel_main(_info: &'static mut BootInfo) -> ! {
    kprint!("stack_overflow::stack_overflow...\t");

    caffeine::drivers::interrupts::gdt::init_gdt();
    // use our custom IDT for tests
    init_test_idt();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    kprintln!("[ok]");
    exit_qemu(QemuExitCode::Success);
}

pub fn init_test_idt() {
    TEST_IDT.load();
}


#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    caffeine::test_panic_handler(info)

}
