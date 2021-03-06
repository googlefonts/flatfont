// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum WoffMetadataCreditsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct WoffMetadataCredits<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for WoffMetadataCredits<'a> {
  type Inner = WoffMetadataCredits<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> WoffMetadataCredits<'a> {
  pub const VT_CREDITS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    WoffMetadataCredits { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args WoffMetadataCreditsArgs<'args>
  ) -> flatbuffers::WIPOffset<WoffMetadataCredits<'bldr>> {
    let mut builder = WoffMetadataCreditsBuilder::new(_fbb);
    if let Some(x) = args.credits { builder.add_credits(x); }
    builder.finish()
  }


  #[inline]
  pub fn credits(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataCredit<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataCredit>>>>(WoffMetadataCredits::VT_CREDITS, None)
  }
}

impl flatbuffers::Verifiable for WoffMetadataCredits<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<WoffMetadataCredit>>>>("credits", Self::VT_CREDITS, false)?
     .finish();
    Ok(())
  }
}
pub struct WoffMetadataCreditsArgs<'a> {
    pub credits: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataCredit<'a>>>>>,
}
impl<'a> Default for WoffMetadataCreditsArgs<'a> {
  #[inline]
  fn default() -> Self {
    WoffMetadataCreditsArgs {
      credits: None,
    }
  }
}
pub struct WoffMetadataCreditsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WoffMetadataCreditsBuilder<'a, 'b> {
  #[inline]
  pub fn add_credits(&mut self, credits: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<WoffMetadataCredit<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataCredits::VT_CREDITS, credits);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WoffMetadataCreditsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WoffMetadataCreditsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<WoffMetadataCredits<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for WoffMetadataCredits<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("WoffMetadataCredits");
      ds.field("credits", &self.credits());
      ds.finish()
  }
}
