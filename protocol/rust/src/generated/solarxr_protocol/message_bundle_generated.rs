// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum MessageBundleOffset {}
#[derive(Copy, Clone, PartialEq)]

/// MessageBundle contains all of the messages for the data feed system and the
/// rpc system that will be sent in one buffer.
pub struct MessageBundle<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MessageBundle<'a> {
  type Inner = MessageBundle<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> MessageBundle<'a> {
  pub const VT_DATA_FEED_MSGS: flatbuffers::VOffsetT = 4;
  pub const VT_RPC_MSGS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    MessageBundle { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args MessageBundleArgs<'args>
  ) -> flatbuffers::WIPOffset<MessageBundle<'bldr>> {
    let mut builder = MessageBundleBuilder::new(_fbb);
    if let Some(x) = args.rpc_msgs { builder.add_rpc_msgs(x); }
    if let Some(x) = args.data_feed_msgs { builder.add_data_feed_msgs(x); }
    builder.finish()
  }


  #[inline]
  pub fn data_feed_msgs(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<data_feed::DataFeedMessageHeader<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<data_feed::DataFeedMessageHeader>>>>(MessageBundle::VT_DATA_FEED_MSGS, None)
  }
  #[inline]
  pub fn rpc_msgs(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<rpc::RpcMessageHeader<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<rpc::RpcMessageHeader>>>>(MessageBundle::VT_RPC_MSGS, None)
  }
}

impl flatbuffers::Verifiable for MessageBundle<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<data_feed::DataFeedMessageHeader>>>>("data_feed_msgs", Self::VT_DATA_FEED_MSGS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<rpc::RpcMessageHeader>>>>("rpc_msgs", Self::VT_RPC_MSGS, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageBundleArgs<'a> {
    pub data_feed_msgs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<data_feed::DataFeedMessageHeader<'a>>>>>,
    pub rpc_msgs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<rpc::RpcMessageHeader<'a>>>>>,
}
impl<'a> Default for MessageBundleArgs<'a> {
  #[inline]
  fn default() -> Self {
    MessageBundleArgs {
      data_feed_msgs: None,
      rpc_msgs: None,
    }
  }
}

pub struct MessageBundleBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBundleBuilder<'a, 'b> {
  #[inline]
  pub fn add_data_feed_msgs(&mut self, data_feed_msgs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<data_feed::DataFeedMessageHeader<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MessageBundle::VT_DATA_FEED_MSGS, data_feed_msgs);
  }
  #[inline]
  pub fn add_rpc_msgs(&mut self, rpc_msgs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<rpc::RpcMessageHeader<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MessageBundle::VT_RPC_MSGS, rpc_msgs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBundleBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBundleBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MessageBundle<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for MessageBundle<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("MessageBundle");
      ds.field("data_feed_msgs", &self.data_feed_msgs());
      ds.field("rpc_msgs", &self.rpc_msgs());
      ds.finish()
  }
}
