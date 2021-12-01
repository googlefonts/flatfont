// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum WoffMetadataTrademarkOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct WoffMetadataTrademark<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for WoffMetadataTrademark<'a> {
  type Inner = WoffMetadataTrademark<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> WoffMetadataTrademark<'a> {
  pub const VT_TEXT: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    WoffMetadataTrademark { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args WoffMetadataTrademarkArgs<'args>
  ) -> flatbuffers::WIPOffset<WoffMetadataTrademark<'bldr>> {
    let mut builder = WoffMetadataTrademarkBuilder::new(_fbb);
    if let Some(x) = args.text { builder.add_text(x); }
    builder.finish()
  }


  #[inline]
  pub fn text(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataText<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataText>>>>(WoffMetadataTrademark::VT_TEXT, None)
  }
}

impl flatbuffers::Verifiable for WoffMetadataTrademark<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<WoffMetadataText>>>>("text", Self::VT_TEXT, false)?
     .finish();
    Ok(())
  }
}
pub struct WoffMetadataTrademarkArgs<'a> {
    pub text: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<WoffMetadataText<'a>>>>>,
}
impl<'a> Default for WoffMetadataTrademarkArgs<'a> {
  #[inline]
  fn default() -> Self {
    WoffMetadataTrademarkArgs {
      text: None,
    }
  }
}
pub struct WoffMetadataTrademarkBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WoffMetadataTrademarkBuilder<'a, 'b> {
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<WoffMetadataText<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataTrademark::VT_TEXT, text);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WoffMetadataTrademarkBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WoffMetadataTrademarkBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<WoffMetadataTrademark<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for WoffMetadataTrademark<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("WoffMetadataTrademark");
      ds.field("text", &self.text());
      ds.finish()
  }
}
