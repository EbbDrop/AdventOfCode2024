// Code modified from https://github.com/BurntSushi/memchr

use std::arch::x86_64::__m256i;

#[inline(always)]
unsafe fn fwd_byte_by_byte_inv<F: Fn(u8) -> bool>(
    start: *const u8,
    end: *const u8,
    confirm: F,
) -> Option<*const u8> {
    debug_assert!(start <= end);
    let mut ptr = start;
    while ptr < end {
        if !confirm(*ptr) {
            return Some(ptr);
        }
        ptr = ptr.offset(1);
    }
    None
}

/// A trait for adding some helper routines to pointers.
trait Pointer {
    /// Returns the distance, in units of `T`, between `self` and `origin`.
    ///
    /// # Safety
    ///
    /// Same as `ptr::offset_from` in addition to `self >= origin`.
    unsafe fn distance(self, origin: Self) -> usize;

    /// Casts this pointer to `usize`.
    ///
    /// Callers should not convert the `usize` back to a pointer if at all
    /// possible. (And if you believe it's necessary, open an issue to discuss
    /// why. Otherwise, it has the potential to violate pointer provenance.)
    /// The purpose of this function is just to be able to do arithmetic, i.e.,
    /// computing offsets or alignments.
    fn as_usize(self) -> usize;
}

impl<T> Pointer for *const T {
    unsafe fn distance(self, origin: *const T) -> usize {
        // TODO: Replace with `ptr::sub_ptr` once stabilized.
        usize::try_from(self.offset_from(origin)).unwrap_unchecked()
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl<T> Pointer for *mut T {
    unsafe fn distance(self, origin: *mut T) -> usize {
        (self as *const T).distance(origin as *const T)
    }

    fn as_usize(self) -> usize {
        (self as *const T).as_usize()
    }
}

/// An iterator over all occurrences of a set of bytes in a haystack.
///
/// This iterator implements the routines necessary to provide a
/// `DoubleEndedIterator` impl, which means it can also be used to find
/// occurrences in reverse order.
///
/// The lifetime parameters are as follows:
///
/// * `'h` refers to the lifetime of the haystack being searched.
///
/// This type is intended to be used to implement all iterators for the
/// `memchr` family of functions. It handles a tiny bit of marginally tricky
/// raw pointer math, but otherwise expects the caller to provide `find_raw`
/// and `rfind_raw` routines for each call of `next` and `next_back`,
/// respectively.
#[derive(Clone, Debug)]
struct Iter<'h> {
    /// The original starting point into the haystack. We use this to convert
    /// pointers to offsets.
    original_start: *const u8,
    /// The current starting point into the haystack. That is, where the next
    /// search will begin.
    start: *const u8,
    /// The current ending point into the haystack. That is, where the next
    /// reverse search will begin.
    end: *const u8,
    /// A marker for tracking the lifetime of the start/cur_start/cur_end
    /// pointers above, which all point into the haystack.
    haystack: core::marker::PhantomData<&'h [u8]>,
}

// SAFETY: Iter contains no shared references to anything that performs any
// interior mutations. Also, the lifetime guarantees that Iter will not outlive
// the haystack.
unsafe impl<'h> Send for Iter<'h> {}

// SAFETY: Iter perform no interior mutations, therefore no explicit
// synchronization is necessary. Also, the lifetime guarantees that Iter will
// not outlive the haystack.
unsafe impl<'h> Sync for Iter<'h> {}

impl<'h> Iter<'h> {
    /// Create a new generic memchr iterator.
    #[inline(always)]
    fn new(haystack: &'h [u8]) -> Iter<'h> {
        Iter {
            original_start: haystack.as_ptr(),
            start: haystack.as_ptr(),
            end: haystack.as_ptr().wrapping_add(haystack.len()),
            haystack: core::marker::PhantomData,
        }
    }

    /// Returns the next occurrence in the forward direction.
    ///
    /// # Safety
    ///
    /// Callers must ensure that if a pointer is returned from the closure
    /// provided, then it must be greater than or equal to the start pointer
    /// and less than the end pointer.
    #[inline(always)]
    unsafe fn next(
        &mut self,
        mut find_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>,
    ) -> Option<usize> {
        // SAFETY: Pointers are derived directly from the same &[u8] haystack.
        // We only ever modify start/end corresponding to a matching offset
        // found between start and end. Thus all changes to start/end maintain
        // our safety requirements.
        //
        // The only other assumption we rely on is that the pointer returned
        // by `find_raw` satisfies `self.start <= found < self.end`, and that
        // safety contract is forwarded to the caller.
        let found = find_raw(self.start, self.end)?;
        let result = found.distance(self.original_start);
        self.start = found.add(1);
        Some(result)
    }
}

/// Finds all occurrences of not equal to a single byte in a haystack.
#[derive(Clone, Copy, Debug)]
struct GenOneInv<V> {
    s1: u8,
    v1: V,
}

impl<V: Vector> GenOneInv<V> {
    /// The number of bytes we examine per each iteration of our search loop.
    const LOOP_SIZE: usize = 4 * V::BYTES;

    /// Create a new searcher that finds non occurrences of the byte given.
    #[inline(always)]
    unsafe fn new(needle: u8) -> GenOneInv<V> {
        GenOneInv {
            s1: needle,
            v1: V::splat(needle),
        }
    }

    /// Returns the needle given to `One::new`.
    #[inline(always)]
    fn needle1(&self) -> u8 {
        self.s1
    }

    /// Comment not updated to inv version
    /// Return a pointer to the first occurrence of the needle in the given
    /// haystack. If no such occurrence exists, then `None` is returned.
    ///
    /// When a match is found, the pointer returned is guaranteed to be
    /// `>= start` and `< end`.
    ///
    /// # Safety
    ///
    /// * It must be the case that `start < end` and that the distance between
    /// them is at least equal to `V::BYTES`. That is, it must always be valid
    /// to do at least an unaligned load of `V` at `start`.
    /// * Both `start` and `end` must be valid for reads.
    /// * Both `start` and `end` must point to an initialized value.
    /// * Both `start` and `end` must point to the same allocated object and
    /// must either be in bounds or at most one byte past the end of the
    /// allocated object.
    /// * Both `start` and `end` must be _derived from_ a pointer to the same
    /// object.
    /// * The distance between `start` and `end` must not overflow `isize`.
    /// * The distance being in bounds must not rely on "wrapping around" the
    /// address space.
    #[inline(always)]
    unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8> {
        // If we want to support vectors bigger than 256 bits, we probably
        // need to move up to using a u64 for the masks used below. Currently
        // they are 32 bits, which means we're SOL for vectors that need masks
        // bigger than 32 bits. Overall unclear until there's a use case.
        debug_assert!(V::BYTES <= 32, "vector cannot be bigger than 32 bytes");

        let topos = V::Mask::first_offset;
        let len = end.distance(start);
        debug_assert!(
            len >= V::BYTES,
            "haystack has length {}, but must be at least {}",
            len,
            V::BYTES
        );

        // Search a possibly unaligned chunk at `start`. This covers any part
        // of the haystack prior to where aligned loads can start.
        if let Some(cur) = self.search_chunk(start, topos) {
            return Some(cur);
        }
        // Set `cur` to the first V-aligned pointer greater than `start`.
        let mut cur = start.add(V::BYTES - (start.as_usize() & V::ALIGN));
        debug_assert!(cur > start && end.sub(V::BYTES) >= start);
        if len >= Self::LOOP_SIZE {
            while cur <= end.sub(Self::LOOP_SIZE) {
                debug_assert_eq!(0, cur.as_usize() % V::BYTES);

                let a = V::load_aligned(cur);
                let b = V::load_aligned(cur.add(1 * V::BYTES));
                let c = V::load_aligned(cur.add(2 * V::BYTES));
                let d = V::load_aligned(cur.add(3 * V::BYTES));
                let eqa = self.v1.cmpneq(a);
                let eqb = self.v1.cmpneq(b);
                let eqc = self.v1.cmpneq(c);
                let eqd = self.v1.cmpneq(d);
                let or1 = eqa.or(eqb);
                let or2 = eqc.or(eqd);
                let or3 = or1.or(or2);
                if or3.movemask_will_have_non_zero() {
                    let mask = eqa.movemask();
                    if mask.has_non_zero() {
                        return Some(cur.add(topos(mask)));
                    }

                    let mask = eqb.movemask();
                    if mask.has_non_zero() {
                        return Some(cur.add(1 * V::BYTES).add(topos(mask)));
                    }

                    let mask = eqc.movemask();
                    if mask.has_non_zero() {
                        return Some(cur.add(2 * V::BYTES).add(topos(mask)));
                    }

                    let mask = eqd.movemask();
                    debug_assert!(mask.has_non_zero());
                    return Some(cur.add(3 * V::BYTES).add(topos(mask)));
                }
                cur = cur.add(Self::LOOP_SIZE);
            }
        }
        // Handle any leftovers after the aligned loop above. We use unaligned
        // loads here, but I believe we are guaranteed that they are aligned
        // since `cur` is aligned.
        while cur <= end.sub(V::BYTES) {
            debug_assert!(end.distance(cur) >= V::BYTES);
            if let Some(cur) = self.search_chunk(cur, topos) {
                return Some(cur);
            }
            cur = cur.add(V::BYTES);
        }
        // Finally handle any remaining bytes less than the size of V. In this
        // case, our pointer may indeed be unaligned and the load may overlap
        // with the previous one. But that's okay since we know the previous
        // load didn't lead to a match (otherwise we wouldn't be here).
        if cur < end {
            debug_assert!(end.distance(cur) < V::BYTES);
            cur = cur.sub(V::BYTES - end.distance(cur));
            debug_assert_eq!(end.distance(cur), V::BYTES);
            return self.search_chunk(cur, topos);
        }
        None
    }

    /// Search `V::BYTES` starting at `cur` via an unaligned load.
    ///
    /// `mask_to_offset` should be a function that converts a `movemask` to
    /// an offset such that `cur.add(offset)` corresponds to a pointer to the
    /// match location if one is found. Generally it is expected to use either
    /// `mask_to_first_offset` or `mask_to_last_offset`, depending on whether
    /// one is implementing a forward or reverse search, respectively.
    ///
    /// # Safety
    ///
    /// `cur` must be a valid pointer and it must be valid to do an unaligned
    /// load of size `V::BYTES` at `cur`.
    #[inline(always)]
    unsafe fn search_chunk(
        &self,
        cur: *const u8,
        mask_to_offset: impl Fn(V::Mask) -> usize,
    ) -> Option<*const u8> {
        let chunk = V::load_unaligned(cur);
        let mask = self.v1.cmpneq(chunk).movemask();
        if mask.has_non_zero() {
            Some(cur.add(mask_to_offset(mask)))
        } else {
            None
        }
    }
}

/// A trait for describing vector operations used by vectorized searchers.
///
/// The trait is highly constrained to low level vector operations needed.
/// In general, it was invented mostly to be generic over x86's __m128i and
/// __m256i types. At time of writing, it also supports wasm and aarch64
/// 128-bit vector types as well.
///
/// # Safety
///
/// All methods are not safe since they are intended to be implemented using
/// vendor intrinsics, which are also not safe. Callers must ensure that the
/// appropriate target features are enabled in the calling function, and that
/// the current CPU supports them. All implementations should avoid marking the
/// routines with #[target_feature] and instead mark them as #[inline(always)]
/// to ensure they get appropriately inlined. (inline(always) cannot be used
/// with target_feature.)
trait Vector: Copy + core::fmt::Debug {
    /// The number of bytes in the vector. That is, this is the size of the
    /// vector in memory.
    const BYTES: usize;
    /// The bits that must be zero in order for a `*const u8` pointer to be
    /// correctly aligned to read vector values.
    const ALIGN: usize;

    /// The type of the value returned by `Vector::movemask`.
    ///
    /// This supports abstracting over the specific representation used in
    /// order to accommodate different representations in different ISAs.
    type Mask: MoveMask;

    /// Create a vector with 8-bit lanes with the given byte repeated into each
    /// lane.
    unsafe fn splat(byte: u8) -> Self;

    /// Read a vector-size number of bytes from the given pointer. The pointer
    /// must be aligned to the size of the vector.
    ///
    /// # Safety
    ///
    /// Callers must guarantee that at least `BYTES` bytes are readable from
    /// `data` and that `data` is aligned to a `BYTES` boundary.
    unsafe fn load_aligned(data: *const u8) -> Self;

    /// Read a vector-size number of bytes from the given pointer. The pointer
    /// does not need to be aligned.
    ///
    /// # Safety
    ///
    /// Callers must guarantee that at least `BYTES` bytes are readable from
    /// `data`.
    unsafe fn load_unaligned(data: *const u8) -> Self;

    /// _mm_movemask_epi8 or _mm256_movemask_epi8
    unsafe fn movemask(self) -> Self::Mask;
    /// _mm_cmpeq_epi8 or _mm256_cmpeq_epi8
    unsafe fn cmpneq(self, vector2: Self) -> Self;
    /// _mm_or or _mm256_or_si256
    unsafe fn or(self, vector2: Self) -> Self;
    /// Returns true if and only if `Self::movemask` would return a mask that
    /// contains at least one non-zero bit.
    unsafe fn movemask_will_have_non_zero(self) -> bool {
        self.movemask().has_non_zero()
    }
}

/// A trait that abstracts over a vector-to-scalar operation called
/// "move mask."
///
/// On x86-64, this is `_mm_movemask_epi8` for SSE2 and `_mm256_movemask_epi8`
/// for AVX2. It takes a vector of `u8` lanes and returns a scalar where the
/// `i`th bit is set if and only if the most significant bit in the `i`th lane
/// of the vector is set. The simd128 ISA for wasm32 also supports this
/// exact same operation natively.
///
/// ... But aarch64 doesn't. So we have to fake it with more instructions and
/// a slightly different representation. We could do extra work to unify the
/// representations, but then would require additional costs in the hot path
/// for `memchr` and `packedpair`. So instead, we abstraction over the specific
/// representation with this trait and define the operations we actually need.
trait MoveMask: Copy + core::fmt::Debug {
    /// Returns true if and only if this mask has a a non-zero bit anywhere.
    fn has_non_zero(self) -> bool;

    /// Returns the offset of the first non-zero lane this mask represents.
    fn first_offset(self) -> usize;
}

/// This is a "sensible" movemask implementation where each bit represents
/// whether the most significant bit is set in each corresponding lane of a
/// vector. This is used on x86-64 and wasm, but such a mask is more expensive
/// to get on aarch64 so we use something a little different.
///
/// We call this "sensible" because this is what we get using native sse/avx
/// movemask instructions. But neon has no such native equivalent.
#[derive(Clone, Copy, Debug)]
struct SensibleMoveMask(u32);

impl SensibleMoveMask {
    /// Get the mask in a form suitable for computing offsets.
    ///
    /// Basically, this normalizes to little endian. On big endian, this swaps
    /// the bytes.
    #[inline(always)]
    fn get_for_offset(self) -> u32 {
        #[cfg(target_endian = "big")]
        {
            self.0.swap_bytes()
        }
        #[cfg(target_endian = "little")]
        {
            self.0
        }
    }
}

impl MoveMask for SensibleMoveMask {
    #[inline(always)]
    fn has_non_zero(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    fn first_offset(self) -> usize {
        // We are dealing with little endian here (and if we aren't, we swap
        // the bytes so we are in practice), where the most significant byte
        // is at a higher address. That means the least significant bit that
        // is set corresponds to the position of our first matching byte.
        // That position corresponds to the number of zeros after the least
        // significant bit.
        self.get_for_offset().trailing_zeros() as usize
    }
}

#[cfg(target_arch = "x86_64")]
mod x86avx2 {
    use core::arch::x86_64::*;

    use super::{SensibleMoveMask, Vector};

    impl Vector for __m256i {
        const BYTES: usize = 32;
        const ALIGN: usize = Self::BYTES - 1;

        type Mask = SensibleMoveMask;

        #[inline(always)]
        unsafe fn splat(byte: u8) -> __m256i {
            _mm256_set1_epi8(byte as i8)
        }

        #[inline(always)]
        unsafe fn load_aligned(data: *const u8) -> __m256i {
            _mm256_load_si256(data as *const __m256i)
        }

        #[inline(always)]
        unsafe fn load_unaligned(data: *const u8) -> __m256i {
            _mm256_loadu_si256(data as *const __m256i)
        }

        #[inline(always)]
        unsafe fn movemask(self) -> SensibleMoveMask {
            SensibleMoveMask(_mm256_movemask_epi8(self) as u32)
        }

        #[inline(always)]
        unsafe fn cmpneq(self, vector2: Self) -> __m256i {
            _mm256_xor_si256(_mm256_cmpeq_epi8(self, vector2), _mm256_set1_epi64x(-1))
        }

        #[inline(always)]
        unsafe fn or(self, vector2: Self) -> __m256i {
            _mm256_or_si256(self, vector2)
        }
    }
}

/// Finds all occurrences of a single byte in a haystack.
#[derive(Clone, Copy, Debug)]
pub struct OneInv {
    /// Used for haystacks bigger than 32 bytes.
    avx2: GenOneInv<__m256i>,
}

impl OneInv {
    /// Create a new finder specific to AVX2 vectors and routines without
    /// checking that either SSE2 or AVX2 is available.
    ///
    /// # Safety
    ///
    /// Callers must guarantee that it is safe to execute both `sse2` and
    /// `avx2` instructions in the current environment.
    ///
    /// Note that it is a common misconception that if one compiles for an
    /// `x86_64` target, then they therefore automatically have access to SSE2
    /// instructions. While this is almost always the case, it isn't true in
    /// 100% of cases.
    #[target_feature(enable = "sse2", enable = "avx2")]
    #[inline]
    pub unsafe fn new_unchecked(needle: u8) -> OneInv {
        OneInv {
            avx2: GenOneInv::new(needle),
        }
    }

    /// Like `find`, but accepts and returns raw pointers.
    ///
    /// When a match is found, the pointer returned is guaranteed to be
    /// `>= start` and `< end`.
    ///
    /// This routine is useful if you're already using raw pointers and would
    /// like to avoid converting back to a slice before executing a search.
    ///
    /// # Safety
    ///
    /// * Both `start` and `end` must be valid for reads.
    /// * Both `start` and `end` must point to an initialized value.
    /// * Both `start` and `end` must point to the same allocated object and
    /// must either be in bounds or at most one byte past the end of the
    /// allocated object.
    /// * Both `start` and `end` must be _derived from_ a pointer to the same
    /// object.
    /// * The distance between `start` and `end` must not overflow `isize`.
    /// * The distance being in bounds must not rely on "wrapping around" the
    /// address space.
    ///
    /// Note that callers may pass a pair of pointers such that `start >= end`.
    /// In that case, `None` will always be returned.
    #[inline]
    pub unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8> {
        if start >= end {
            return None;
        }
        let len = end.distance(start);
        if len < __m256i::BYTES {
            return fwd_byte_by_byte_inv(start, end, |b| b == self.avx2.needle1());
        }
        // SAFETY: Building a `One` means it's safe to call both 'sse2' and
        // 'avx2' routines. Also, we've checked that our haystack is big
        // enough to run on the vector routine. Pointer validity is caller's
        // responsibility.
        //
        // Note that we could call `self.avx2.find_raw` directly here. But that
        // means we'd have to annotate this routine with `target_feature`.
        // Which is fine, because this routine is `unsafe` anyway and the
        // `target_feature` obligation is met by virtue of building a `One`.
        // The real problem is that a routine with a `target_feature`
        // annotation generally can't be inlined into caller code unless
        // the caller code has the same target feature annotations. Namely,
        // the common case (at time of writing) is for calling code to not
        // have the `avx2` target feature enabled *at compile time*. Without
        // `target_feature` on this routine, it can be inlined which will
        // handle some of the short-haystack cases above without touching the
        // architecture specific code.
        self.find_raw_avx2(start, end)
    }

    /// Execute a search using AVX2 vectors and routines.
    ///
    /// # Safety
    ///
    /// Same as [`One::find_raw`], except the distance between `start` and
    /// `end` must be at least the size of an AVX2 vector (in bytes).
    ///
    /// (The target feature safety obligation is automatically fulfilled by
    /// virtue of being a method on `One`, which can only be constructed
    /// when it is safe to call `sse2`/`avx2` routines.)
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn find_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8> {
        self.avx2.find_raw(start, end)
    }

    /// Returns an iterator over all occurrences of the needle byte in the
    /// given haystack.
    ///
    /// The iterator returned implements `DoubleEndedIterator`. This means it
    /// can also be used to find occurrences in reverse order.
    #[inline]
    pub fn iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> OneInvIter<'a, 'h> {
        OneInvIter {
            searcher: self,
            it: Iter::new(haystack),
        }
    }
}

/// An iterator over all occurrences of a single byte in a haystack.
///
/// This iterator implements `DoubleEndedIterator`, which means it can also be
/// used to find occurrences in reverse order.
///
/// This iterator is created by the [`One::iter`] method.
///
/// The lifetime parameters are as follows:
///
/// * `'a` refers to the lifetime of the underlying [`One`] searcher.
/// * `'h` refers to the lifetime of the haystack being searched.
#[derive(Clone, Debug)]
pub struct OneInvIter<'a, 'h> {
    searcher: &'a OneInv,
    it: Iter<'h>,
}

impl<'a, 'h> Iterator for OneInvIter<'a, 'h> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        // SAFETY: We rely on the generic iterator to provide valid start
        // and end pointers, but we guarantee that any pointer returned by
        // 'find_raw' falls within the bounds of the start and end pointer.
        unsafe { self.it.next(|s, e| self.searcher.find_raw(s, e)) }
    }
}
