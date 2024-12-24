use core::{alloc::*, ptr::*, sync::atomic::*};
use uefi::boot::*;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new(PageAllocator::uefi());

// Non cleaning and non thread safe allocator
struct Allocator {
    pager: PageAllocator,

    start: AtomicPtr<u8>,
    end: AtomicPtr<u8>,
}

impl Allocator {
    pub const fn new(pager: PageAllocator) -> Self {
        Self {
            pager: pager,
            start: AtomicPtr::new(null_mut()),
            end: AtomicPtr::new(null_mut()),
        }
    }

    unsafe fn new_page(&self) {
        self.start.store((self.pager.alloc)(1), Ordering::Relaxed);
        self.end.store(
            self.start
                .load(Ordering::Relaxed)
                .byte_add(self.pager.page_size()),
            Ordering::Relaxed,
        )
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Larger sizes to their own page
        if layout.size() >= self.pager.page_size() / 2 {
            return (self.pager.alloc)(
                (layout.size() + self.pager.page_size() - 1) / self.pager.page_size(),
            );
        }

        let mut v = self.start.load(Ordering::Relaxed);
        if v == null_mut()
            || v.byte_add(v.align_offset(layout.align()) + layout.size())
                > self.end.load(Ordering::Relaxed)
        {
            self.new_page();
            v = self.start.load(Ordering::Relaxed);
        }

        v = v.byte_add(v.align_offset(layout.align()));

        self.start
            .store(v.byte_add(layout.size()), Ordering::Relaxed);

        return v;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // TODO: dealloc implementation
    }
}

struct PageAllocator {
    alloc: fn(n: usize) -> *mut u8,
}

impl PageAllocator {
    const fn uefi() -> Self {
        Self {
            alloc: uefi_page_alloc,
        }
    }
    const fn page_size(&self) -> usize {
        return 4096;
    }
}

fn uefi_page_alloc(n: usize) -> *mut u8 {
    return allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, n)
        .unwrap()
        .as_ptr();
}
