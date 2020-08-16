#![allow(unused_macros)]

pub struct ScopeCall<F: FnOnce()> {
    pub c: Option<F>
}
impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

macro_rules! expr  { ($e: expr) => { $e } } // tt hack
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = crate::macros::ScopeCall {
            c: Some(|| -> () { expr!({ $($data)* }) })
        };
    )
}
