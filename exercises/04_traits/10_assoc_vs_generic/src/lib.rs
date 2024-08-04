// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

trait Power<T: Sized>: Sized + Copy {
    fn power(&self, exp: T) -> Self;
}

macro_rules! impl_power {
    ($trait_name:ident, $([$base_ty:ty, $exp_ty:ty] $(= $pow:expr)?),+) => {
        $(impl_power! {
            @power $trait_name, [$base_ty, $exp_ty] $(= $pow)?
        })+
    };
    (@power $trait_name:ident, [$base_ty:ty, $exp_ty:ty]) => {
        impl_power! {
            @power $trait_name, [$base_ty, $exp_ty] = |b: $base_ty, e: $exp_ty| b.pow(e.into())
        }
    };
    (@power $trait_name:ident, [$base_ty:ty, $exp_ty:ty] = $pow:expr) => {
        impl $trait_name<$exp_ty> for $base_ty {
            fn power(&self, exp: $exp_ty) -> $base_ty {
                $pow(*self, exp)
            }
        }
    };
}

impl_power! {
    Power,
    [u32, u16],
    [u32, u32],
    [u32, &u32] = |b: u32, e: &u32| b.pow((*e).into())
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
