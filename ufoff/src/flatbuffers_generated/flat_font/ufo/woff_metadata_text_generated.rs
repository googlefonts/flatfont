// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum WoffMetadataTextOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct WoffMetadataText<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for WoffMetadataText<'a> {
  type Inner = WoffMetadataText<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> WoffMetadataText<'a> {
  pub const VT_TEXT: flatbuffers::VOffsetT = 4;
  pub const VT_LANGUAGE: flatbuffers::VOffsetT = 6;
  pub const VT_DIR: flatbuffers::VOffsetT = 8;
  pub const VT_CLASS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    WoffMetadataText { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args WoffMetadataTextArgs<'args>
  ) -> flatbuffers::WIPOffset<WoffMetadataText<'bldr>> {
    let mut builder = WoffMetadataTextBuilder::new(_fbb);
    if let Some(x) = args.class { builder.add_class(x); }
    if let Some(x) = args.dir { builder.add_dir(x); }
    if let Some(x) = args.language { builder.add_language(x); }
    if let Some(x) = args.text { builder.add_text(x); }
    builder.finish()
  }


  #[inline]
  pub fn text(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataText::VT_TEXT, None).unwrap()
  }
  #[inline]
  pub fn language(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataText::VT_LANGUAGE, None)
  }
  #[inline]
  pub fn dir(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataText::VT_DIR, None)
  }
  #[inline]
  pub fn class(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataText::VT_CLASS, None)
  }
}

impl flatbuffers::Verifiable for WoffMetadataText<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("text", Self::VT_TEXT, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("language", Self::VT_LANGUAGE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("dir", Self::VT_DIR, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("class", Self::VT_CLASS, false)?
     .finish();
    Ok(())
  }
}
pub struct WoffMetadataTextArgs<'a> {
    pub text: Option<flatbuffers::WIPOffset<&'a str>>,
    pub language: Option<flatbuffers::WIPOffset<&'a str>>,
    pub dir: Option<flatbuffers::WIPOffset<&'a str>>,
    pub class: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for WoffMetadataTextArgs<'a> {
  #[inline]
  fn default() -> Self {
    WoffMetadataTextArgs {
      text: None, // required field
      language: None,
      dir: None,
      class: None,
    }
  }
}
pub struct WoffMetadataTextBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WoffMetadataTextBuilder<'a, 'b> {
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataText::VT_TEXT, text);
  }
  #[inline]
  pub fn add_language(&mut self, language: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataText::VT_LANGUAGE, language);
  }
  #[inline]
  pub fn add_dir(&mut self, dir: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataText::VT_DIR, dir);
  }
  #[inline]
  pub fn add_class(&mut self, class: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataText::VT_CLASS, class);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WoffMetadataTextBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WoffMetadataTextBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<WoffMetadataText<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, WoffMetadataText::VT_TEXT,"text");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for WoffMetadataText<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("WoffMetadataText");
      ds.field("text", &self.text());
      ds.field("language", &self.language());
      ds.field("dir", &self.dir());
      ds.field("class", &self.class());
      ds.finish()
  }
}