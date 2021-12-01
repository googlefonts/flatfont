// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum WoffMetadataExtensionValueOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct WoffMetadataExtensionValue<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for WoffMetadataExtensionValue<'a> {
  type Inner = WoffMetadataExtensionValue<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> WoffMetadataExtensionValue<'a> {
  pub const VT_TEXT: flatbuffers::VOffsetT = 4;
  pub const VT_LANGUAGE: flatbuffers::VOffsetT = 6;
  pub const VT_DIR: flatbuffers::VOffsetT = 8;
  pub const VT_CLASS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    WoffMetadataExtensionValue { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args WoffMetadataExtensionValueArgs<'args>
  ) -> flatbuffers::WIPOffset<WoffMetadataExtensionValue<'bldr>> {
    let mut builder = WoffMetadataExtensionValueBuilder::new(_fbb);
    if let Some(x) = args.class { builder.add_class(x); }
    if let Some(x) = args.dir { builder.add_dir(x); }
    if let Some(x) = args.language { builder.add_language(x); }
    if let Some(x) = args.text { builder.add_text(x); }
    builder.finish()
  }


  #[inline]
  pub fn text(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataExtensionValue::VT_TEXT, None).unwrap()
  }
  #[inline]
  pub fn language(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataExtensionValue::VT_LANGUAGE, None)
  }
  #[inline]
  pub fn dir(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataExtensionValue::VT_DIR, None)
  }
  #[inline]
  pub fn class(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(WoffMetadataExtensionValue::VT_CLASS, None)
  }
}

impl flatbuffers::Verifiable for WoffMetadataExtensionValue<'_> {
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
pub struct WoffMetadataExtensionValueArgs<'a> {
    pub text: Option<flatbuffers::WIPOffset<&'a str>>,
    pub language: Option<flatbuffers::WIPOffset<&'a str>>,
    pub dir: Option<flatbuffers::WIPOffset<&'a str>>,
    pub class: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for WoffMetadataExtensionValueArgs<'a> {
  #[inline]
  fn default() -> Self {
    WoffMetadataExtensionValueArgs {
      text: None, // required field
      language: None,
      dir: None,
      class: None,
    }
  }
}
pub struct WoffMetadataExtensionValueBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WoffMetadataExtensionValueBuilder<'a, 'b> {
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataExtensionValue::VT_TEXT, text);
  }
  #[inline]
  pub fn add_language(&mut self, language: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataExtensionValue::VT_LANGUAGE, language);
  }
  #[inline]
  pub fn add_dir(&mut self, dir: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataExtensionValue::VT_DIR, dir);
  }
  #[inline]
  pub fn add_class(&mut self, class: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(WoffMetadataExtensionValue::VT_CLASS, class);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WoffMetadataExtensionValueBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WoffMetadataExtensionValueBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<WoffMetadataExtensionValue<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, WoffMetadataExtensionValue::VT_TEXT,"text");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for WoffMetadataExtensionValue<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("WoffMetadataExtensionValue");
      ds.field("text", &self.text());
      ds.field("language", &self.language());
      ds.field("dir", &self.dir());
      ds.field("class", &self.class());
      ds.finish()
  }
}
