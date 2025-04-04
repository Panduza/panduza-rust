// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

// struct Timestamp, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Timestamp(pub [u8; 16]);
impl Default for Timestamp {
    fn default() -> Self {
        Self([0; 16])
    }
}
impl core::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timestamp")
            .field("secs", &self.secs())
            .field("nanos", &self.nanos())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for Timestamp {}
impl<'a> flatbuffers::Follow<'a> for Timestamp {
    type Inner = &'a Timestamp;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a Timestamp>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a Timestamp {
    type Inner = &'a Timestamp;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<Timestamp>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for Timestamp {
    type Output = Timestamp;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(
            self as *const Timestamp as *const u8,
            <Self as flatbuffers::Push>::size(),
        );
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(8)
    }
}

impl<'a> flatbuffers::Verifiable for Timestamp {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}

impl<'a> Timestamp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(secs: u64, nanos: u32) -> Self {
        let mut s = Self([0; 16]);
        s.set_secs(secs);
        s.set_nanos(nanos);
        s
    }

    pub fn secs(&self) -> u64 {
        let mut mem = core::mem::MaybeUninit::<<u64 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[0..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_secs(&mut self, x: u64) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[0..].as_mut_ptr(),
                core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
            );
        }
    }

    pub fn nanos(&self) -> u32 {
        let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[8..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_nanos(&mut self, x: u32) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[8..].as_mut_ptr(),
                core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
            );
        }
    }
}

// struct Range, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Range(pub [u8; 12]);
impl Default for Range {
    fn default() -> Self {
        Self([0; 12])
    }
}
impl core::fmt::Debug for Range {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Range")
            .field("used", &self.used())
            .field("min", &self.min())
            .field("max", &self.max())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for Range {}
impl<'a> flatbuffers::Follow<'a> for Range {
    type Inner = &'a Range;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a Range>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a Range {
    type Inner = &'a Range;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<Range>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for Range {
    type Output = Range;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(
            self as *const Range as *const u8,
            <Self as flatbuffers::Push>::size(),
        );
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for Range {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}

impl<'a> Range {
    #[allow(clippy::too_many_arguments)]
    pub fn new(used: bool, min: f32, max: f32) -> Self {
        let mut s = Self([0; 12]);
        s.set_used(used);
        s.set_min(min);
        s.set_max(max);
        s
    }

    pub fn used(&self) -> bool {
        let mut mem = core::mem::MaybeUninit::<<bool as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[0..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<bool as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_used(&mut self, x: bool) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[0..].as_mut_ptr(),
                core::mem::size_of::<<bool as EndianScalar>::Scalar>(),
            );
        }
    }

    pub fn min(&self) -> f32 {
        let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[4..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_min(&mut self, x: f32) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[4..].as_mut_ptr(),
                core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
            );
        }
    }

    pub fn max(&self) -> f32 {
        let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        EndianScalar::from_little_endian(unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[8..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
            );
            mem.assume_init()
        })
    }

    pub fn set_max(&mut self, x: f32) {
        let x_le = x.to_little_endian();
        // Safety:
        // Created from a valid Table for this object
        // Which contains a valid value in this slot
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const _ as *const u8,
                self.0[8..].as_mut_ptr(),
                core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
            );
        }
    }
}

pub enum OptionsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Options<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Options<'a> {
    type Inner = Options<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Options<'a> {
    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_RANGE: flatbuffers::VOffsetT = 6;
    pub const VT_WHITELIST: flatbuffers::VOffsetT = 8;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Options { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args OptionsArgs<'args>,
    ) -> flatbuffers::WIPOffset<Options<'bldr>> {
        let mut builder = OptionsBuilder::new(_fbb);
        if let Some(x) = args.whitelist {
            builder.add_whitelist(x);
        }
        if let Some(x) = args.range {
            builder.add_range(x);
        }
        builder.add_id(args.id);
        builder.finish()
    }

    #[inline]
    pub fn id(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u8>(Options::VT_ID, Some(0)).unwrap() }
    }
    #[inline]
    pub fn range(&self) -> Option<&'a Range> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<Range>(Options::VT_RANGE, None) }
    }
    #[inline]
    pub fn whitelist(&self) -> Option<flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(
                    Options::VT_WHITELIST,
                    None,
                )
        }
    }
}

impl flatbuffers::Verifiable for Options<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u8>("id", Self::VT_ID, false)?
            .visit_field::<Range>("range", Self::VT_RANGE, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>(
                "whitelist",
                Self::VT_WHITELIST,
                false,
            )?
            .finish();
        Ok(())
    }
}
pub struct OptionsArgs<'a> {
    pub id: u8,
    pub range: Option<&'a Range>,
    pub whitelist: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
}
impl<'a> Default for OptionsArgs<'a> {
    #[inline]
    fn default() -> Self {
        OptionsArgs {
            id: 0,
            range: None,
            whitelist: None,
        }
    }
}

pub struct OptionsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> OptionsBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_id(&mut self, id: u8) {
        self.fbb_.push_slot::<u8>(Options::VT_ID, id, 0);
    }
    #[inline]
    pub fn add_range(&mut self, range: &Range) {
        self.fbb_
            .push_slot_always::<&Range>(Options::VT_RANGE, range);
    }
    #[inline]
    pub fn add_whitelist(
        &mut self,
        whitelist: flatbuffers::WIPOffset<flatbuffers::Vector<'b, f32>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Options::VT_WHITELIST, whitelist);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> OptionsBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        OptionsBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Options<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Options<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Options");
        ds.field("id", &self.id());
        ds.field("range", &self.range());
        ds.field("whitelist", &self.whitelist());
        ds.finish()
    }
}
pub enum TriggerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Trigger<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Trigger<'a> {
    type Inner = Trigger<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Trigger<'a> {
    pub const VT_REFRESH: flatbuffers::VOffsetT = 4;
    pub const VT_TIMESTAMP: flatbuffers::VOffsetT = 6;
    pub const VT_OPTIONS: flatbuffers::VOffsetT = 8;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Trigger { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args TriggerArgs<'args>,
    ) -> flatbuffers::WIPOffset<Trigger<'bldr>> {
        let mut builder = TriggerBuilder::new(_fbb);
        if let Some(x) = args.options {
            builder.add_options(x);
        }
        if let Some(x) = args.timestamp {
            builder.add_timestamp(x);
        }
        builder.add_refresh(args.refresh);
        builder.finish()
    }

    #[inline]
    pub fn refresh(&self) -> f32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<f32>(Trigger::VT_REFRESH, Some(0.0))
                .unwrap()
        }
    }
    #[inline]
    pub fn timestamp(&self) -> Option<&'a Timestamp> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<Timestamp>(Trigger::VT_TIMESTAMP, None) }
    }
    #[inline]
    pub fn options(&self) -> Option<Options<'a>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<Options>>(Trigger::VT_OPTIONS, None)
        }
    }
}

impl flatbuffers::Verifiable for Trigger<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<f32>("refresh", Self::VT_REFRESH, false)?
            .visit_field::<Timestamp>("timestamp", Self::VT_TIMESTAMP, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<Options>>(
                "options",
                Self::VT_OPTIONS,
                false,
            )?
            .finish();
        Ok(())
    }
}
pub struct TriggerArgs<'a> {
    pub refresh: f32,
    pub timestamp: Option<&'a Timestamp>,
    pub options: Option<flatbuffers::WIPOffset<Options<'a>>>,
}
impl<'a> Default for TriggerArgs<'a> {
    #[inline]
    fn default() -> Self {
        TriggerArgs {
            refresh: 0.0,
            timestamp: None,
            options: None,
        }
    }
}

pub struct TriggerBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> TriggerBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_refresh(&mut self, refresh: f32) {
        self.fbb_
            .push_slot::<f32>(Trigger::VT_REFRESH, refresh, 0.0);
    }
    #[inline]
    pub fn add_timestamp(&mut self, timestamp: &Timestamp) {
        self.fbb_
            .push_slot_always::<&Timestamp>(Trigger::VT_TIMESTAMP, timestamp);
    }
    #[inline]
    pub fn add_options(&mut self, options: flatbuffers::WIPOffset<Options<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<Options>>(Trigger::VT_OPTIONS, options);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TriggerBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        TriggerBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Trigger<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Trigger<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Trigger");
        ds.field("refresh", &self.refresh());
        ds.field("timestamp", &self.timestamp());
        ds.field("options", &self.options());
        ds.finish()
    }
}
