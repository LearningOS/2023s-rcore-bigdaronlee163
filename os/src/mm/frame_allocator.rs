//! Implementation of [`FrameAllocator`] which
//! controls all the frames in the operating system.

use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;

/// tracker for physical page frame allocation and deallocation
pub struct FrameTracker {
    /// physical page number
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    /// Create a new FrameTracker
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        // 返回该物理帧对应的一个页大小的实际地址。
        // 然后将内容全部清空。
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}
/// an implementation for frame allocator
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    // 在使用之前需要。
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        /*
        .0 是 Rust 中的一个语法糖，用于获取一个元组中第一个元素的值。
        在这里，l 和 r 是 PhysPageNum 类型的变量，
        它们实际上是元组结构体，由一个无符号整数字段组成。
        因此，使用 .0 可以获取这个无符号整数值，
        然后赋值给 self.current 和 self.end 两个字段。
         */
        self.current = l.0;
        self.end = r.0;
        // trace!("last {} Physical Frames.", self.end - self.current);
    }
}
// 实现 FrameAllocator 这个trait
impl FrameAllocator for StackFrameAllocator {
    // 初始化该 StackFrameAllocator
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        // 模式匹配 使用Some 来匹配 vec弹出的元素。弹出最后vec中的最后一个元素。
        if let Some(ppn) = self.recycled.pop() {
            // 存在回收的物理帧
            Some(ppn.into())
        } else if self.current == self.end {
            // 物理帧已经分配完
            None
        } else {
            // cur 往后移动 表示之前的cur已经被占用。
            self.current += 1;
            // 返回被cur 使用 into 方法将 usize 转换成了物理页号 PhysPageNum
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        // 如果在 cur后面或者在recycled中，表示没有占用，还没有被分配出去，无需进行回收
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // do recycle
        self.recycled.push(ppn);
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    /// frame allocator instance through lazy_static!
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}
/// initiate the frame allocator using `ekernel` and `MEMORY_END`
pub fn init_frame_allocator() {
    // 从汇编中获取符号。
    extern "C" {
        fn ekernel();
    }
    // exclusive_access 独占所有权
    FRAME_ALLOCATOR.exclusive_access().init(
        // 往上取值
        PhysAddr::from(ekernel as usize).ceil(),
        // 往下取值
        PhysAddr::from(MEMORY_END).floor(),
    );
}

/// Allocate a physical page frame in FrameTracker style
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}

/// Deallocate a physical page frame with a given ppn
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
