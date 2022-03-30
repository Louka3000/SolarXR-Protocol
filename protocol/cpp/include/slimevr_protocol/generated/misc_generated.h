// automatically generated by the FlatBuffers compiler, do not modify


#ifndef FLATBUFFERS_GENERATED_MISC_SLIMEVR_PROTOCOL_MISC_H_
#define FLATBUFFERS_GENERATED_MISC_SLIMEVR_PROTOCOL_MISC_H_

#include "flatbuffers/flatbuffers.h"

namespace slimevr_protocol {
namespace misc {

struct Acknowledgement;
struct AcknowledgementBuilder;

struct Acknowledgement FLATBUFFERS_FINAL_CLASS : private flatbuffers::Table {
  typedef AcknowledgementBuilder Builder;
  enum FlatBuffersVTableOffset FLATBUFFERS_VTABLE_UNDERLYING_TYPE {
    VT_PACKET_ID = 4
  };
  uint64_t packet_id() const {
    return GetField<uint64_t>(VT_PACKET_ID, 0);
  }
  bool Verify(flatbuffers::Verifier &verifier) const {
    return VerifyTableStart(verifier) &&
           VerifyField<uint64_t>(verifier, VT_PACKET_ID, 8) &&
           verifier.EndTable();
  }
};

struct AcknowledgementBuilder {
  typedef Acknowledgement Table;
  flatbuffers::FlatBufferBuilder &fbb_;
  flatbuffers::uoffset_t start_;
  void add_packet_id(uint64_t packet_id) {
    fbb_.AddElement<uint64_t>(Acknowledgement::VT_PACKET_ID, packet_id, 0);
  }
  explicit AcknowledgementBuilder(flatbuffers::FlatBufferBuilder &_fbb)
        : fbb_(_fbb) {
    start_ = fbb_.StartTable();
  }
  flatbuffers::Offset<Acknowledgement> Finish() {
    const auto end = fbb_.EndTable(start_);
    auto o = flatbuffers::Offset<Acknowledgement>(end);
    return o;
  }
};

inline flatbuffers::Offset<Acknowledgement> CreateAcknowledgement(
    flatbuffers::FlatBufferBuilder &_fbb,
    uint64_t packet_id = 0) {
  AcknowledgementBuilder builder_(_fbb);
  builder_.add_packet_id(packet_id);
  return builder_.Finish();
}

}  // namespace misc
}  // namespace slimevr_protocol

#endif  // FLATBUFFERS_GENERATED_MISC_SLIMEVR_PROTOCOL_MISC_H_