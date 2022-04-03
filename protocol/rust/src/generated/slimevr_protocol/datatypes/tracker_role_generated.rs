// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_TRACKER_ROLE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_TRACKER_ROLE: u8 = 21;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TRACKER_ROLE: [TrackerRole; 22] = [
  TrackerRole::NONE,
  TrackerRole::WAIST,
  TrackerRole::LEFT_FOOT,
  TrackerRole::RIGHT_FOOT,
  TrackerRole::CHEST,
  TrackerRole::LEFT_KNEE,
  TrackerRole::RIGHT_KNEE,
  TrackerRole::LEFT_ELBOW,
  TrackerRole::RIGHT_ELBOW,
  TrackerRole::LEFT_SHOULDER,
  TrackerRole::RIGHT_SHOULDER,
  TrackerRole::LEFT_HAND,
  TrackerRole::RIGHT_HAND,
  TrackerRole::LEFT_CONTROLLER,
  TrackerRole::RIGHT_CONTROLLER,
  TrackerRole::HEAD,
  TrackerRole::NECK,
  TrackerRole::CAMERA,
  TrackerRole::KEYBOARD,
  TrackerRole::HMD,
  TrackerRole::BEACON,
  TrackerRole::GENERIC_CONTROLLER,
];

/// Currently from SlimeVR server.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TrackerRole(pub u8);
#[allow(non_upper_case_globals)]
impl TrackerRole {
  pub const NONE: Self = Self(0);
  pub const WAIST: Self = Self(1);
  pub const LEFT_FOOT: Self = Self(2);
  pub const RIGHT_FOOT: Self = Self(3);
  pub const CHEST: Self = Self(4);
  pub const LEFT_KNEE: Self = Self(5);
  pub const RIGHT_KNEE: Self = Self(6);
  pub const LEFT_ELBOW: Self = Self(7);
  pub const RIGHT_ELBOW: Self = Self(8);
  pub const LEFT_SHOULDER: Self = Self(9);
  pub const RIGHT_SHOULDER: Self = Self(10);
  pub const LEFT_HAND: Self = Self(11);
  pub const RIGHT_HAND: Self = Self(12);
  pub const LEFT_CONTROLLER: Self = Self(13);
  pub const RIGHT_CONTROLLER: Self = Self(14);
  pub const HEAD: Self = Self(15);
  pub const NECK: Self = Self(16);
  pub const CAMERA: Self = Self(17);
  pub const KEYBOARD: Self = Self(18);
  pub const HMD: Self = Self(19);
  pub const BEACON: Self = Self(20);
  pub const GENERIC_CONTROLLER: Self = Self(21);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 21;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::WAIST,
    Self::LEFT_FOOT,
    Self::RIGHT_FOOT,
    Self::CHEST,
    Self::LEFT_KNEE,
    Self::RIGHT_KNEE,
    Self::LEFT_ELBOW,
    Self::RIGHT_ELBOW,
    Self::LEFT_SHOULDER,
    Self::RIGHT_SHOULDER,
    Self::LEFT_HAND,
    Self::RIGHT_HAND,
    Self::LEFT_CONTROLLER,
    Self::RIGHT_CONTROLLER,
    Self::HEAD,
    Self::NECK,
    Self::CAMERA,
    Self::KEYBOARD,
    Self::HMD,
    Self::BEACON,
    Self::GENERIC_CONTROLLER,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::WAIST => Some("WAIST"),
      Self::LEFT_FOOT => Some("LEFT_FOOT"),
      Self::RIGHT_FOOT => Some("RIGHT_FOOT"),
      Self::CHEST => Some("CHEST"),
      Self::LEFT_KNEE => Some("LEFT_KNEE"),
      Self::RIGHT_KNEE => Some("RIGHT_KNEE"),
      Self::LEFT_ELBOW => Some("LEFT_ELBOW"),
      Self::RIGHT_ELBOW => Some("RIGHT_ELBOW"),
      Self::LEFT_SHOULDER => Some("LEFT_SHOULDER"),
      Self::RIGHT_SHOULDER => Some("RIGHT_SHOULDER"),
      Self::LEFT_HAND => Some("LEFT_HAND"),
      Self::RIGHT_HAND => Some("RIGHT_HAND"),
      Self::LEFT_CONTROLLER => Some("LEFT_CONTROLLER"),
      Self::RIGHT_CONTROLLER => Some("RIGHT_CONTROLLER"),
      Self::HEAD => Some("HEAD"),
      Self::NECK => Some("NECK"),
      Self::CAMERA => Some("CAMERA"),
      Self::KEYBOARD => Some("KEYBOARD"),
      Self::HMD => Some("HMD"),
      Self::BEACON => Some("BEACON"),
      Self::GENERIC_CONTROLLER => Some("GENERIC_CONTROLLER"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for TrackerRole {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for TrackerRole {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for TrackerRole {
    type Output = TrackerRole;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for TrackerRole {
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

impl<'a> flatbuffers::Verifiable for TrackerRole {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for TrackerRole {}