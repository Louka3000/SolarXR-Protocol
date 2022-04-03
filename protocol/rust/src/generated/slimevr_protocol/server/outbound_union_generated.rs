// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_OUTBOUND_UNION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_OUTBOUND_UNION: u8 = 3;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_OUTBOUND_UNION: [OutboundUnion; 4] = [
  OutboundUnion::NONE,
  OutboundUnion::TrackersList,
  OutboundUnion::SettingsResponse,
  OutboundUnion::slimevr_protocol_misc_Acknowledgement,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct OutboundUnion(pub u8);
#[allow(non_upper_case_globals)]
impl OutboundUnion {
  pub const NONE: Self = Self(0);
  pub const TrackersList: Self = Self(1);
  pub const SettingsResponse: Self = Self(2);
  pub const slimevr_protocol_misc_Acknowledgement: Self = Self(3);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 3;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::TrackersList,
    Self::SettingsResponse,
    Self::slimevr_protocol_misc_Acknowledgement,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::TrackersList => Some("TrackersList"),
      Self::SettingsResponse => Some("SettingsResponse"),
      Self::slimevr_protocol_misc_Acknowledgement => Some("slimevr_protocol_misc_Acknowledgement"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for OutboundUnion {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for OutboundUnion {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for OutboundUnion {
    type Output = OutboundUnion;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for OutboundUnion {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for OutboundUnion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for OutboundUnion {}
pub struct OutboundUnionUnionTableOffset {}
