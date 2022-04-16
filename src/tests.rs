use crate::{kprint, kprintln};

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        kprint!("{}...\t", core::any::type_name::<T>());
        self();
        kprintln!("[ok]");
    }
}
#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
