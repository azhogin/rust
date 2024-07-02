#![unstable(feature = "async_drop", issue = "126482")]

#[allow(unused_imports)]
use crate::pin::Pin;

#[allow(unused_imports)]
use crate::task::{Context, Poll};

#[allow(unused_imports)]
use core::future::Future;

/// Async version of Drop trait.
///
/// When a value is no longer needed, Rust will run a "destructor" on that value.
/// The most common way that a value is no longer needed is when it goes out of
/// scope. Destructors may still run in other circumstances, but we're going to
/// focus on scope for the examples here. To learn about some of those other cases,
/// please see [the reference] section on destructors.
///
/// [the reference]: https://doc.rust-lang.org/reference/destructors.html
///
/// ## `Copy` and ([`Drop`]|`AsyncDrop`) are exclusive
///
/// You cannot implement both [`Copy`] and ([`Drop`]|`AsyncDrop`) on the same type. Types that
/// are `Copy` get implicitly duplicated by the compiler, making it very
/// hard to predict when, and how often destructors will be executed. As such,
/// these types cannot have destructors.
#[cfg(not(bootstrap))]
#[unstable(feature = "async_drop", issue = "126482")]
#[lang = "async_drop"]
pub trait AsyncDrop {
    /// Executes the async destructor for this type.
    ///
    /// This method is called implicitly when the value goes out of scope,
    /// and cannot be called explicitly.
    ///
    /// When this method has been called, `self` has not yet been deallocated.
    /// That only happens after the method is over.
    ///
    /// # Panics
    #[allow(async_fn_in_trait)]
    async fn drop(self: Pin<&mut Self>);
}

/// Async drop of coroutine.
/// Coroutine with internal async drops will have its own drop function in async form.
/// And required to be polled in loop.
#[cfg(not(bootstrap))]
#[unstable(feature = "async_drop", issue = "126482")]
#[lang = "future_drop_poll"]
#[allow(unconditional_recursion)]
pub fn future_drop_poll<F>(fut: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<()>
where
    F: Future,
{
    // Code here does not matter - this is replaced by the
    // real implementation by the compiler.

    future_drop_poll(fut, cx)
}

/// Async drop.
#[cfg(not(bootstrap))]
#[unstable(feature = "async_drop", issue = "126482")]
#[lang = "async_drop_in_place"]
pub async unsafe fn async_drop_in_place<T: ?Sized>(_to_drop: *mut T) {
    // Code here does not matter - this is replaced by the
    // real implementation by the compiler.
}

/// Used for noop async destructors. We don't use [`core::future::Ready`]
/// because it panics after its second poll, which could be potentially
/// bad if that would happen during the cleanup.
#[cfg(not(bootstrap))]
#[unstable(feature = "async_drop", issue = "126482")]
#[lang = "Noop"]
#[derive(Clone, Copy)]
struct Noop;

#[cfg(not(bootstrap))]
#[unstable(feature = "async_drop", issue = "126482")]
impl Future for Noop {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
