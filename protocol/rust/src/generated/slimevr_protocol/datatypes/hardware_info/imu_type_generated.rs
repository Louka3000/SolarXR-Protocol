// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_IMU_TYPE: u16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_IMU_TYPE: u16 = 5;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_IMU_TYPE: [ImuType; 6] = [
  ImuType::Other,
  ImuType::BNO085,
  ImuType::BNO080,
  ImuType::MPU6050,
  ImuType::MPU9250,
  ImuType::MPU6500,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImuType(pub u16);
#[allow(non_upper_case_globals)]
impl ImuType {
  pub const Other: Self = Self(0);
  pub const BNO085: Self = Self(1);
  pub const BNO080: Self = Self(2);
  pub const MPU6050: Self = Self(3);
  pub const MPU9250: Self = Self(4);
  pub const MPU6500: Self = Self(5);

  pub const ENUM_MIN: u16 = 0;
  pub const ENUM_MAX: u16 = 5;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Other,
    Self::BNO085,
    Self::BNO080,
    Self::MPU6050,
    Self::MPU9250,
    Self::MPU6500,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Other => Some("Other"),
      Self::BNO085 => Some("BNO085"),
      Self::BNO080 => Some("BNO080"),
      Self::MPU6050 => Some("MPU6050"),
      Self::MPU9250 => Some("MPU9250"),
      Self::MPU6500 => Some("MPU6500"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for ImuType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ImuType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u16>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for ImuType {
    type Output = ImuType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u16>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for ImuType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u16::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u16::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ImuType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ImuType {}