#![no_std]

use core::marker::PhantomData;

pub struct PartiallyApplied<This, Ext, Args> {
    this: This,
    extension: Ext,
    args: PhantomData<Args>,
}

pub trait PartiallyApply: Sized {
    fn apply<Ext, Args>(self, extension: Ext) -> PartiallyApplied<Self, Ext, Args> {
        PartiallyApplied {
            this: self,
            extension,
            args: PhantomData,
        }
    }

    fn apply_ref<Ext, Args>(&self, extension: Ext) -> PartiallyApplied<&Self, Ext, Args> {
        self.apply(extension)
    }

    fn apply_mut<Ext, Args>(&mut self, extension: Ext) -> PartiallyApplied<&mut Self, Ext, Args> {
        self.apply(extension)
    }
}

impl<T> PartiallyApply for T {}

macro_rules! impls_for_partially_applied {
    ($arg:ident: $T:ident$(, $targ:ident: $tail:ident)*$(,)?) => {
        impl<This, $T$(, $tail)*, U, F: FnOnce(This, $T$(, $tail)*) -> U> PartiallyApplied<This, F, ($T$(, $tail)*,)> {
            pub fn pass(self, $arg: $T$(, $targ: $tail)*) -> U {
                let Self { this, extension, .. } = self;
                extension(this, $arg$(, $targ)*)
            }
        }
        impls_for_partially_applied!($($targ: $tail),*);
    };
    () => {
        impl<This, U, F: FnOnce(This) -> U> PartiallyApplied<This, F, ()> {
            pub fn pass(self) -> U {
                let Self { this, extension, .. } = self;
                extension(this)
            }
        }
    };
}

impls_for_partially_applied!(
    arg0: T0,
    arg1: T1,
    arg2: T2,
    arg3: T3,
    arg4: T4,
    arg5: T5,
    arg6: T6,
    arg7: T7,
    arg8: T8,
    arg9: T9,
    arg10: T10,
    arg11: T11,
);

#[cfg(test)]
mod tests {
    use crate::*;

    mod option {
        pub fn as_slice<T>(this: &Option<T>) -> &[T] {
            this.as_ref().map(core::slice::from_ref).unwrap_or(&[])
        }
    }

    mod slice {
        pub fn get(this: &[i32], index: usize) -> i32 {
            this[index]
        }
    }

    mod i32 {
        pub fn add_then_multiply(this: i32, add: i32, mul: i32) -> i32 {
            (this + add) * mul
        }
    }

    #[test]
    fn it_works() {
        let x = Some(0);
        let _y = x
            .apply_ref(option::as_slice)
            .pass()
            .apply(slice::get)
            .pass(0)
            .apply(i32::add_then_multiply)
            .pass(2, 2);
    }
}
