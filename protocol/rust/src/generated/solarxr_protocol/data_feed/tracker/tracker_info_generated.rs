// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum TrackerInfoOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Static description of a tracker
pub struct TrackerInfo<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrackerInfo<'a> {
  type Inner = TrackerInfo<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> TrackerInfo<'a> {
  pub const VT_IMU_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_BODY_PART: flatbuffers::VOffsetT = 6;
  pub const VT_POLL_RATE: flatbuffers::VOffsetT = 8;
  pub const VT_MOUNTING_ORIENTATION: flatbuffers::VOffsetT = 10;
  pub const VT_EDITABLE: flatbuffers::VOffsetT = 12;
  pub const VT_COMPUTED: flatbuffers::VOffsetT = 14;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TrackerInfo { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TrackerInfoArgs<'args>
  ) -> flatbuffers::WIPOffset<TrackerInfo<'bldr>> {
    let mut builder = TrackerInfoBuilder::new(_fbb);
    if let Some(x) = args.mounting_orientation { builder.add_mounting_orientation(x); }
    if let Some(x) = args.poll_rate { builder.add_poll_rate(x); }
    builder.add_imu_type(args.imu_type);
    builder.add_computed(args.computed);
    builder.add_editable(args.editable);
    builder.add_body_part(args.body_part);
    builder.finish()
  }


  #[inline]
  pub fn imu_type(&self) -> super::super::datatypes::hardware_info::ImuType {
    self._tab.get::<super::super::datatypes::hardware_info::ImuType>(TrackerInfo::VT_IMU_TYPE, Some(super::super::datatypes::hardware_info::ImuType::Other)).unwrap()
  }
  /// The user-assigned role of the tracker.
  #[inline]
  pub fn body_part(&self) -> super::super::datatypes::BodyPart {
    self._tab.get::<super::super::datatypes::BodyPart>(TrackerInfo::VT_BODY_PART, Some(super::super::datatypes::BodyPart::NONE)).unwrap()
  }
  /// average samples per second
  #[inline]
  pub fn poll_rate(&self) -> Option<&'a super::super::datatypes::HzF32> {
    self._tab.get::<super::super::datatypes::HzF32>(TrackerInfo::VT_POLL_RATE, None)
  }
  /// The orientation of the tracker when mounted on the body
  #[inline]
  pub fn mounting_orientation(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    self._tab.get::<super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_ORIENTATION, None)
  }
  #[inline]
  pub fn editable(&self) -> bool {
    self._tab.get::<bool>(TrackerInfo::VT_EDITABLE, Some(false)).unwrap()
  }
  #[inline]
  pub fn computed(&self) -> bool {
    self._tab.get::<bool>(TrackerInfo::VT_COMPUTED, Some(false)).unwrap()
  }
}

impl flatbuffers::Verifiable for TrackerInfo<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::super::datatypes::hardware_info::ImuType>("imu_type", Self::VT_IMU_TYPE, false)?
     .visit_field::<super::super::datatypes::BodyPart>("body_part", Self::VT_BODY_PART, false)?
     .visit_field::<super::super::datatypes::HzF32>("poll_rate", Self::VT_POLL_RATE, false)?
     .visit_field::<super::super::datatypes::math::Quat>("mounting_orientation", Self::VT_MOUNTING_ORIENTATION, false)?
     .visit_field::<bool>("editable", Self::VT_EDITABLE, false)?
     .visit_field::<bool>("computed", Self::VT_COMPUTED, false)?
     .finish();
    Ok(())
  }
}
pub struct TrackerInfoArgs<'a> {
    pub imu_type: super::super::datatypes::hardware_info::ImuType,
    pub body_part: super::super::datatypes::BodyPart,
    pub poll_rate: Option<&'a super::super::datatypes::HzF32>,
    pub mounting_orientation: Option<&'a super::super::datatypes::math::Quat>,
    pub editable: bool,
    pub computed: bool,
}
impl<'a> Default for TrackerInfoArgs<'a> {
  #[inline]
  fn default() -> Self {
    TrackerInfoArgs {
      imu_type: super::super::datatypes::hardware_info::ImuType::Other,
      body_part: super::super::datatypes::BodyPart::NONE,
      poll_rate: None,
      mounting_orientation: None,
      editable: false,
      computed: false,
    }
  }
}

pub struct TrackerInfoBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TrackerInfoBuilder<'a, 'b> {
  #[inline]
  pub fn add_imu_type(&mut self, imu_type: super::super::datatypes::hardware_info::ImuType) {
    self.fbb_.push_slot::<super::super::datatypes::hardware_info::ImuType>(TrackerInfo::VT_IMU_TYPE, imu_type, super::super::datatypes::hardware_info::ImuType::Other);
  }
  #[inline]
  pub fn add_body_part(&mut self, body_part: super::super::datatypes::BodyPart) {
    self.fbb_.push_slot::<super::super::datatypes::BodyPart>(TrackerInfo::VT_BODY_PART, body_part, super::super::datatypes::BodyPart::NONE);
  }
  #[inline]
  pub fn add_poll_rate(&mut self, poll_rate: &super::super::datatypes::HzF32) {
    self.fbb_.push_slot_always::<&super::super::datatypes::HzF32>(TrackerInfo::VT_POLL_RATE, poll_rate);
  }
  #[inline]
  pub fn add_mounting_orientation(&mut self, mounting_orientation: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_ORIENTATION, mounting_orientation);
  }
  #[inline]
  pub fn add_editable(&mut self, editable: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_EDITABLE, editable, false);
  }
  #[inline]
  pub fn add_computed(&mut self, computed: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_COMPUTED, computed, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrackerInfoBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TrackerInfoBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TrackerInfo<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for TrackerInfo<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("TrackerInfo");
      ds.field("imu_type", &self.imu_type());
      ds.field("body_part", &self.body_part());
      ds.field("poll_rate", &self.poll_rate());
      ds.field("mounting_orientation", &self.mounting_orientation());
      ds.field("editable", &self.editable());
      ds.field("computed", &self.computed());
      ds.finish()
  }
}
