// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class MessageBundleT {
  private slimevr_protocol.data_feed.DataFeedMessageHeaderT[] dataFeedMsgs;
  private slimevr_protocol.rpc.RpcMessageHeaderT[] rpcMsgs;

  public slimevr_protocol.data_feed.DataFeedMessageHeaderT[] getDataFeedMsgs() { return dataFeedMsgs; }

  public void setDataFeedMsgs(slimevr_protocol.data_feed.DataFeedMessageHeaderT[] dataFeedMsgs) { this.dataFeedMsgs = dataFeedMsgs; }

  public slimevr_protocol.rpc.RpcMessageHeaderT[] getRpcMsgs() { return rpcMsgs; }

  public void setRpcMsgs(slimevr_protocol.rpc.RpcMessageHeaderT[] rpcMsgs) { this.rpcMsgs = rpcMsgs; }


  public MessageBundleT() {
    this.dataFeedMsgs = null;
    this.rpcMsgs = null;
  }
}
