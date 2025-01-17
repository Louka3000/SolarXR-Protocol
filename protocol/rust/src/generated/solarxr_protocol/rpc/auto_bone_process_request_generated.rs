// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum AutoBoneProcessRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AutoBoneProcessRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AutoBoneProcessRequest<'a> {
  type Inner = AutoBoneProcessRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> AutoBoneProcessRequest<'a> {
  pub const VT_PROCESS_TYPE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AutoBoneProcessRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AutoBoneProcessRequestArgs
  ) -> flatbuffers::WIPOffset<AutoBoneProcessRequest<'bldr>> {
    let mut builder = AutoBoneProcessRequestBuilder::new(_fbb);
    builder.add_process_type(args.process_type);
    builder.finish()
  }


  #[inline]
  pub fn process_type(&self) -> AutoBoneProcessType {
    self._tab.get::<AutoBoneProcessType>(AutoBoneProcessRequest::VT_PROCESS_TYPE, Some(AutoBoneProcessType::NONE)).unwrap()
  }
}

impl flatbuffers::Verifiable for AutoBoneProcessRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<AutoBoneProcessType>("process_type", Self::VT_PROCESS_TYPE, false)?
     .finish();
    Ok(())
  }
}
pub struct AutoBoneProcessRequestArgs {
    pub process_type: AutoBoneProcessType,
}
impl<'a> Default for AutoBoneProcessRequestArgs {
  #[inline]
  fn default() -> Self {
    AutoBoneProcessRequestArgs {
      process_type: AutoBoneProcessType::NONE,
    }
  }
}

pub struct AutoBoneProcessRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AutoBoneProcessRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_process_type(&mut self, process_type: AutoBoneProcessType) {
    self.fbb_.push_slot::<AutoBoneProcessType>(AutoBoneProcessRequest::VT_PROCESS_TYPE, process_type, AutoBoneProcessType::NONE);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AutoBoneProcessRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AutoBoneProcessRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AutoBoneProcessRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for AutoBoneProcessRequest<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("AutoBoneProcessRequest");
      ds.field("process_type", &self.process_type());
      ds.finish()
  }
}
