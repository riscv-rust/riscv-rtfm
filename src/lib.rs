//! Real Time For the Masses (RTFM) framework for RISCV microcontrollers
//!
//! This crate is based on [the RTFM framework] created by the Embedded Systems
//! group at [Luleå University of Technology][ltu], led by Prof. Per Lindgren,
//! and uses a simplified version of the Stack Resource Policy as scheduling
//! policy (check the [references] for details).
//!
//! [the RTFM framework]: http://www.rtfm-lang.org/
//! [ltu]: https://www.ltu.se/?l=en
//! [per]: https://www.ltu.se/staff/p/pln-1.11258?l=en
//! [references]: ./index.html#references
//!
//! # Features
//!
//! - **Event triggered tasks** as the unit of concurrency.
//! - Support for prioritization of tasks and, thus, **preemptive
//!   multitasking**.
//! - **Efficient and data race free memory sharing** through fine grained *non
//!   global* critical sections.
//! - **Deadlock free execution** guaranteed at compile time.
//! - **Minimal scheduling overhead** as the scheduler has no "software
//!   component": the hardware does all the scheduling.
//! - **Highly efficient memory usage**: All the tasks share a single call stack
//!   and there's no hard dependency on a dynamic memory allocator.
//! - **All Cortex M devices are fully supported**.
//! - This task model is amenable to known WCET (Worst Case Execution Time)
//!   analysis and scheduling analysis techniques. (Though we haven't yet
//!   developed Rust friendly tooling for that.)
//!
//! # Constraints
//!
//! - Tasks must run to completion. That's it, tasks can't contain endless
//!   loops. However, you can run an endless event loop in the `idle` *loop*.
//!
//! - Task priorities must remain constant at runtime.
//!
//! # Dependencies
//!
//! The application crate must depend on a device crate generated using
//! [`svd2rust`] v0.11.x and the "rt" feature of that crate must be enabled. The
//! SVD file used to generate the device crate *must* contain [`<cpu>`]
//! information.
//!
//! [`svd2rust`]: https://docs.rs/svd2rust/0..0/svd2rust/
//! [`<cpu>`]: https://www.keil.com/pack/doc/CMSIS/SVD/html/elem_cpu.html
//!
//! # `app!`
//!
//! The `app!` macro is documented [here].
//!
//! [here]: https://docs.rs/cortex-m-rtfm-macros/0.2.0/cortex_m_rtfm_macros/fn.app.html
//!
//! # Examples
//!
//! In increasing grade of complexity. See the [examples](./examples/index.html)
//! module.
//!
//! # References
//!
//! - Baker, T. P. (1991). Stack-based scheduling of realtime processes.
//!   *Real-Time Systems*, 3(1), 67-99.
//!
//! > The original Stack Resource Policy paper. [PDF][srp].
//!
//! [srp]: http://www.cs.fsu.edu/~baker/papers/mstacks3.pdf
//!
//! - Eriksson, J., Häggström, F., Aittamaa, S., Kruglyak, A., & Lindgren, P.
//!   (2013, June). Real-time for the masses, step 1: Programming API and static
//!   priority SRP kernel primitives. In Industrial Embedded Systems (SIES),
//!   2013 8th IEEE International Symposium on (pp. 110-113). IEEE.
//!
//! > A description of the RTFM task and resource model. [PDF][rtfm]
//!
//! [rtfm]: http://www.diva-portal.org/smash/get/diva2:1005680/FULLTEXT01.pdf
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate riscv;
extern crate rtfm_core;
extern crate untagged_option;

pub use riscv::asm::wfi;
pub use rtfm_core::{Resource, Threshold};
pub use untagged_option::UntaggedOption;

use core::u8;
use riscv::interrupt;

/// Executes the closure `f` in a preemption free context
///
/// During the execution of the closure no task can preempt the current task.
pub fn atomic<R, F>(t: &mut Threshold, f: F) -> R
where
    F: FnOnce(&mut Threshold) -> R,
{
    if t.value() == u8::MAX {
        f(t)
    } else {
        unsafe { interrupt::disable() };
        let r = f(&mut unsafe { Threshold::max() });
        unsafe { interrupt::enable() };
        r
    }
}
