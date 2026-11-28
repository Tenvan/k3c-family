//! Heap-Zähler (Ersatz für Go `runtime/metrics`): ein dünner Mantel um den Allocator des Systems, der nur zählt.
//! Prozessweit für jede EXE und jeden Test, die diese Bibliothek einbinden.

// `GlobalAlloc` ist nur als `unsafe impl` zu haben.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

/// Jetzt belegte Bytes.
static LIVE: AtomicU64 = AtomicU64::new(0);
/// Seit Prozessstart angeforderte Bytes.
static ALLOCATED: AtomicU64 = AtomicU64::new(0);

fn bytes(size: usize) -> u64 {
    size as u64
}

fn grow(size: usize) {
    // Reine Statistik, keine Reihenfolge nötig.
    LIVE.fetch_add(bytes(size), Ordering::Relaxed);
    ALLOCATED.fetch_add(bytes(size), Ordering::Relaxed);
}

fn shrink(size: usize) {
    LIVE.fetch_sub(bytes(size), Ordering::Relaxed);
}

struct Counting;

// SAFETY: Jede Methode reicht ihre Argumente unverändert an `System` weiter und gibt dessen Zeiger zurück; die Zähler
// fassen den Speicher nicht an, allokieren nicht und können nicht abbrechen.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: Zusage des Aufrufers zu `layout`.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            grow(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: wie `alloc`.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            grow(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: Zusage des Aufrufers: `pointer` stammt mit `layout` von hier, also von `System`.
        unsafe { System.dealloc(pointer, layout) };
        shrink(layout.size());
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: wie `dealloc`; `new_size` prüft der Aufrufer.
        let moved = unsafe { System.realloc(pointer, layout, new_size) };
        // Bei null bleibt der alte Block bestehen.
        if !moved.is_null() {
            shrink(layout.size());
            grow(new_size);
        }
        moved
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Jetzt belegte Heap-Bytes.
pub(super) fn live_bytes() -> u64 {
    LIVE.load(Ordering::Relaxed)
}

/// Seit Prozessstart angeforderte Heap-Bytes (kumulativ).
pub(super) fn allocated_bytes() -> u64 {
    ALLOCATED.load(Ordering::Relaxed)
}
