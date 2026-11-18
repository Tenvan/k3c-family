//! Allokationen je Thread zaehlen (RU42, B-1075): ein `GlobalAlloc`, der an `System` weiterreicht und je Thread
//! zaehlt. Nur fuer Tests; ein Test-Binary setzt ihn mit `#[global_allocator]` (sicherer Code, kein `unsafe` im Test).
//!
//! Dieses Crate hat keine externen Abhaengigkeiten und erbt die Workspace-Lints nicht: `unsafe` ist nur an der
//! einen `GlobalAlloc`-Implementierung erlaubt.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    // `const`-Init ohne Destruktor: der Zugriff allokiert selbst nie (sonst Rekursion im Allocator).
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
    static BYTES: Cell<u64> = const { Cell::new(0) };
}

fn bump(bytes: usize) {
    // `try_with`: waehrend des Thread-Abbaus ist der Speicher evtl. schon weg, dann zaehlt nichts.
    let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
    let _ = BYTES.try_with(|c| c.set(c.get() + bytes as u64));
}

/// Zaehlender Allocator: jede Anforderung (`alloc`, `alloc_zeroed`, `realloc`) zaehlt als eine Allokation.
#[derive(Debug, Default, Clone, Copy)]
pub struct CountingAlloc;

// SAFETY: Alle Methoden reichen unveraendert an `System` weiter und halten damit dessen Vertraege ein (gleiches
// Layout, gleicher Zeiger, gleiche Reihenfolge). Zusaetzlich wird nur ein thread-lokaler `Cell`-Zaehler ohne
// Allokation erhoeht.
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        bump(layout.size());
        // SAFETY: gleiche Vertragslage wie der Aufruf an `GlobalAlloc::alloc`.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        bump(layout.size());
        // SAFETY: gleiche Vertragslage wie der Aufruf an `GlobalAlloc::alloc_zeroed`.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        bump(new_size);
        // SAFETY: `ptr` und `layout` stammen vom Aufrufer und gehoeren zu diesem Allocator (also zu `System`).
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` und `layout` stammen vom Aufrufer und gehoeren zu diesem Allocator (also zu `System`).
        unsafe { System.dealloc(ptr, layout) }
    }
}

/// Zaehlerstand eines Threads: Allokationen und angeforderte Bytes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub allocs: u64,
    pub bytes: u64,
}

/// `now` liefert den Zaehlerstand des aufrufenden Threads (nur sinnvoll, wenn `CountingAlloc` global gesetzt ist).
pub fn now() -> Counts {
    Counts {
        allocs: ALLOCS.try_with(Cell::get).unwrap_or(0),
        bytes: BYTES.try_with(Cell::get).unwrap_or(0),
    }
}

/// `measure` fuehrt `f` aus und liefert Ergebnis und die Allokationen dieses Threads waehrend `f`.
pub fn measure<R>(f: impl FnOnce() -> R) -> (R, Counts) {
    let before = now();
    let r = f();
    let after = now();
    (
        r,
        Counts {
            allocs: after.allocs - before.allocs,
            bytes: after.bytes - before.bytes,
        },
    )
}
