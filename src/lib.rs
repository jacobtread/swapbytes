pub use swapbytes_derive::*;

/// Trait implement by fields that can swap their byte ordering
pub trait SwapBytes {
    fn swap_bytes_mut(&mut self);
}

/// Macro for implementing [`SwapBytes`] on integer types that
/// all have the `swap_bytes` function
macro_rules! impl_endian_swap_int {
    ($($ty:ty),*) => {
        $(
            impl SwapBytes for $ty {
                #[inline]
                fn swap_bytes_mut(&mut self) {
                    *self = (*self).swap_bytes();
                }
            }
        )*
    };
}

impl_endian_swap_int![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];

impl SwapBytes for () {
    fn swap_bytes_mut(&mut self) {}
}

impl SwapBytes for bool {
    fn swap_bytes_mut(&mut self) {}
}

impl SwapBytes for f32 {
    fn swap_bytes_mut(&mut self) {
        let mut bytes: [u8; 4] = self.to_ne_bytes();
        bytes.reverse();
        *self = f32::from_le_bytes(bytes)
    }
}

impl SwapBytes for f64 {
    fn swap_bytes_mut(&mut self) {
        let mut bytes: [u8; 8] = self.to_ne_bytes();
        bytes.reverse();
        *self = f64::from_le_bytes(bytes)
    }
}

impl<T> SwapBytes for [T]
where
    T: SwapBytes,
{
    fn swap_bytes_mut(&mut self) {
        self.iter_mut().for_each(|value| {
            value.swap_bytes_mut();
        })
    }
}

impl<const LENGTH: usize, T> SwapBytes for [T; LENGTH]
where
    T: SwapBytes,
{
    fn swap_bytes_mut(&mut self) {
        self.iter_mut().for_each(|value| {
            value.swap_bytes_mut();
        })
    }
}
