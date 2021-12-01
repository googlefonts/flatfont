// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum OpenTypeNameRecordOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct OpenTypeNameRecord<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OpenTypeNameRecord<'a> {
  type Inner = OpenTypeNameRecord<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> OpenTypeNameRecord<'a> {
  pub const VT_NAMEID: flatbuffers::VOffsetT = 4;
  pub const VT_PLATFORMID: flatbuffers::VOffsetT = 6;
  pub const VT_ENCODINGID: flatbuffers::VOffsetT = 8;
  pub const VT_LANGUAGEID: flatbuffers::VOffsetT = 10;
  pub const VT_STRING: flatbuffers::VOffsetT = 12;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    OpenTypeNameRecord { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args OpenTypeNameRecordArgs<'args>
  ) -> flatbuffers::WIPOffset<OpenTypeNameRecord<'bldr>> {
    let mut builder = OpenTypeNameRecordBuilder::new(_fbb);
    if let Some(x) = args.string { builder.add_string(x); }
    builder.add_languageID(args.languageID);
    builder.add_encodingID(args.encodingID);
    builder.add_platformID(args.platformID);
    builder.add_nameID(args.nameID);
    builder.finish()
  }


  #[inline]
  pub fn nameID(&self) -> u16 {
    self._tab.get::<u16>(OpenTypeNameRecord::VT_NAMEID, Some(0)).unwrap()
  }
  #[inline]
  pub fn platformID(&self) -> u16 {
    self._tab.get::<u16>(OpenTypeNameRecord::VT_PLATFORMID, Some(0)).unwrap()
  }
  #[inline]
  pub fn encodingID(&self) -> u16 {
    self._tab.get::<u16>(OpenTypeNameRecord::VT_ENCODINGID, Some(0)).unwrap()
  }
  #[inline]
  pub fn languageID(&self) -> u16 {
    self._tab.get::<u16>(OpenTypeNameRecord::VT_LANGUAGEID, Some(0)).unwrap()
  }
  #[inline]
  pub fn string(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(OpenTypeNameRecord::VT_STRING, None)
  }
}

impl flatbuffers::Verifiable for OpenTypeNameRecord<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u16>("nameID", Self::VT_NAMEID, false)?
     .visit_field::<u16>("platformID", Self::VT_PLATFORMID, false)?
     .visit_field::<u16>("encodingID", Self::VT_ENCODINGID, false)?
     .visit_field::<u16>("languageID", Self::VT_LANGUAGEID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("string", Self::VT_STRING, false)?
     .finish();
    Ok(())
  }
}
pub struct OpenTypeNameRecordArgs<'a> {
    pub nameID: u16,
    pub platformID: u16,
    pub encodingID: u16,
    pub languageID: u16,
    pub string: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for OpenTypeNameRecordArgs<'a> {
  #[inline]
  fn default() -> Self {
    OpenTypeNameRecordArgs {
      nameID: 0,
      platformID: 0,
      encodingID: 0,
      languageID: 0,
      string: None,
    }
  }
}
pub struct OpenTypeNameRecordBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OpenTypeNameRecordBuilder<'a, 'b> {
  #[inline]
  pub fn add_nameID(&mut self, nameID: u16) {
    self.fbb_.push_slot::<u16>(OpenTypeNameRecord::VT_NAMEID, nameID, 0);
  }
  #[inline]
  pub fn add_platformID(&mut self, platformID: u16) {
    self.fbb_.push_slot::<u16>(OpenTypeNameRecord::VT_PLATFORMID, platformID, 0);
  }
  #[inline]
  pub fn add_encodingID(&mut self, encodingID: u16) {
    self.fbb_.push_slot::<u16>(OpenTypeNameRecord::VT_ENCODINGID, encodingID, 0);
  }
  #[inline]
  pub fn add_languageID(&mut self, languageID: u16) {
    self.fbb_.push_slot::<u16>(OpenTypeNameRecord::VT_LANGUAGEID, languageID, 0);
  }
  #[inline]
  pub fn add_string(&mut self, string: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(OpenTypeNameRecord::VT_STRING, string);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OpenTypeNameRecordBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OpenTypeNameRecordBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OpenTypeNameRecord<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for OpenTypeNameRecord<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("OpenTypeNameRecord");
      ds.field("nameID", &self.nameID());
      ds.field("platformID", &self.platformID());
      ds.field("encodingID", &self.encodingID());
      ds.field("languageID", &self.languageID());
      ds.field("string", &self.string());
      ds.finish()
  }
}
