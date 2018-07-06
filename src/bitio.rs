use std::io;

/// numeric cast helper (u32 as T)
pub trait FromU32 {
    fn from_u32(v: u32) -> Self;
}

impl FromU32 for bool {
    #[inline]
    fn from_u32(v: u32) -> Self {
        v != 0
    }
}

macro_rules! impl_from_u32 {
    ($($ty:ty)*) => {
        $(
            impl FromU32 for $ty {
            #[inline]
                fn from_u32(v: u32) -> $ty {
                    v as $ty
                }
            }
        )*
    }
}

impl_from_u32!(u8 u16 u32 u64 usize);

///
/// Bitwise reader
///
pub struct BitReader<R> {
    inner: R,
    bbuf: u8,
    bpos: u8,
    limit: u32,
}

impl<R: io::Read> BitReader<R> {
    pub fn new(inner: R, limit: u32) -> BitReader<R> {
        BitReader {
            inner: inner,
            bbuf: 0,
            bpos: 0,
            limit: limit,
        }
    }

    /// read_bit: read 1 bit
    pub fn read_bit(&mut self) -> Option<u8> {
        if self.bpos == 0 {
            if self.limit == 0 {
                return None;
            }
            let mut bbuf = [0; 1];
            match self.inner.read(&mut bbuf) {
                Ok(n) => assert_eq!(n, 1),
                Err(_) => return None,
            }
            self.bbuf = bbuf[0];
            self.bpos = 8;
            self.limit -= 1;
        }
        self.bpos -= 1;
        Some((self.bbuf >> self.bpos) & 1)
    }

    /// f(n): read n-bits
    pub fn f<T: FromU32>(&mut self, nbit: usize) -> Option<T> {
        assert!(nbit <= 32);
        let mut x: u32 = 0;
        for _ in 0..nbit {
            x = (x << 1) | self.read_bit()? as u32;
        }
        Some(FromU32::from_u32(x))
    }

    // ns(n)
    pub fn ns(&mut self, n: u32) -> Option<u32> {
        let w = Self::floor_log2(n) + 1;
        let m = (1 << w) - n;
        let v = self.f::<u32>(w as usize - 1)?; // f(w - 1)
        if v < m {
            return Some(v);
        }
        let extra_bit = self.f::<u32>(1)?; // f(1)
        Some((v << 1) - m + extra_bit)
    }

    // FloorLog2(x)
    fn floor_log2(mut x: u32) -> u32 {
        let mut s = 0;
        while x != 0 {
            x = x >> 1;
            s += 1;
        }
        s - 1
    }
}
