// automatically generated by the FlatBuffers compiler, do not modify

// @generated

// use core::cmp::Ordering;
// use core::mem;

// // extern crate flatbuffers;
// use flatbuffers::{EndianScalar, Follow};

// use flatbuffers::Verifiable;

pub enum SampleOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Sample<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Sample<'a> {
    type Inner = Sample<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> Sample<'a> {
    pub const VT_VALUES: flatbuffers::VOffsetT = 4;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Sample { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args SampleArgs<'args>,
    ) -> flatbuffers::WIPOffset<Sample<'bldr>> {
        let mut builder = SampleBuilder::new(_fbb);
        if let Some(x) = args.values {
            builder.add_values(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn values(&self) -> Option<flatbuffers::Vector<'a, f32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(
                    Sample::VT_VALUES,
                    None,
                )
        }
    }
}

impl flatbuffers::Verifiable for Sample<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>(
                "values",
                Self::VT_VALUES,
                false,
            )?
            .finish();
        Ok(())
    }
}
pub struct SampleArgs<'a> {
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
}
impl<'a> Default for SampleArgs<'a> {
    #[inline]
    fn default() -> Self {
        SampleArgs { values: None }
    }
}

pub struct SampleBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SampleBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b, f32>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Sample::VT_VALUES, values);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SampleBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        SampleBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Sample<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Sample<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Sample");
        ds.field("values", &self.values());
        ds.finish()
    }
}
