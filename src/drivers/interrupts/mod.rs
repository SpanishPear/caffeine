// https://github.com/anellie/yacuri/blob/main/kernel/src/drivers/interrupts/interrupts.rs
// big inspo

use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;



lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
