// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class ChangeSettingsRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static ChangeSettingsRequest getRootAsChangeSettingsRequest(ByteBuffer _bb) { return getRootAsChangeSettingsRequest(_bb, new ChangeSettingsRequest()); }
  public static ChangeSettingsRequest getRootAsChangeSettingsRequest(ByteBuffer _bb, ChangeSettingsRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ChangeSettingsRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers() { return steamVrTrackers(new solarxr_protocol.rpc.SteamVRTrackersSetting()); }
  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers(solarxr_protocol.rpc.SteamVRTrackersSetting obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.FilteringSettings filtering() { return filtering(new solarxr_protocol.rpc.FilteringSettings()); }
  public solarxr_protocol.rpc.FilteringSettings filtering(solarxr_protocol.rpc.FilteringSettings obj) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }

  public static int createChangeSettingsRequest(FlatBufferBuilder builder,
      int steamVrTrackersOffset,
      int filteringOffset) {
    builder.startTable(2);
    ChangeSettingsRequest.addFiltering(builder, filteringOffset);
    ChangeSettingsRequest.addSteamVrTrackers(builder, steamVrTrackersOffset);
    return ChangeSettingsRequest.endChangeSettingsRequest(builder);
  }

  public static void startChangeSettingsRequest(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addSteamVrTrackers(FlatBufferBuilder builder, int steamVrTrackersOffset) { builder.addOffset(0, steamVrTrackersOffset, 0); }
  public static void addFiltering(FlatBufferBuilder builder, int filteringOffset) { builder.addOffset(1, filteringOffset, 0); }
  public static int endChangeSettingsRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ChangeSettingsRequest get(int j) { return get(new ChangeSettingsRequest(), j); }
    public ChangeSettingsRequest get(ChangeSettingsRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ChangeSettingsRequestT unpack() {
    ChangeSettingsRequestT _o = new ChangeSettingsRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ChangeSettingsRequestT _o) {
    if (steamVrTrackers() != null) _o.setSteamVrTrackers(steamVrTrackers().unpack());
    else _o.setSteamVrTrackers(null);
    if (filtering() != null) _o.setFiltering(filtering().unpack());
    else _o.setFiltering(null);
  }
  public static int pack(FlatBufferBuilder builder, ChangeSettingsRequestT _o) {
    if (_o == null) return 0;
    int _steamVrTrackers = _o.getSteamVrTrackers() == null ? 0 : solarxr_protocol.rpc.SteamVRTrackersSetting.pack(builder, _o.getSteamVrTrackers());
    int _filtering = _o.getFiltering() == null ? 0 : solarxr_protocol.rpc.FilteringSettings.pack(builder, _o.getFiltering());
    return createChangeSettingsRequest(
      builder,
      _steamVrTrackers,
      _filtering);
  }
}

