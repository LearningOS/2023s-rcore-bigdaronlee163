//! Uniprocessor interior mutability primitives
use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.
/// 用于
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    /// 在 new 方法中，它使用了一个名为 RefCell 的 Rust 标准库类型作为内部状态的存储器，并通过 unsafe 关键字将其封装为一个 UPSafeCell 对象。由于 RefCell 具有可变内部状态，因此需要使用 unsafe 在 Rust 语言中标记该方法，以确保调用 new 方法的应用程序已经满足了使用可变内部状态的条件。
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    /// Panic if the data has been borrowed.
    /// 在 exclusive_access 方法中，
    /// 它使用 inner 字段中存储的 RefCell 对象调用 borrow_mut() 方法，
    /// 以获取对包含在 UPSafeCell 中的数据的可变引用。
    /// 如果数据已经被借出，则该方法会 panic。
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
