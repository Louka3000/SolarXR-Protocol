// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol.data_feed.device_data;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class DeviceDataMaskT {
  private slimevr_protocol.data_feed.tracker.TrackerDataMaskT trackerData;
  private boolean deviceData;

  public slimevr_protocol.data_feed.tracker.TrackerDataMaskT getTrackerData() { return trackerData; }

  public void setTrackerData(slimevr_protocol.data_feed.tracker.TrackerDataMaskT trackerData) { this.trackerData = trackerData; }

  public boolean getDeviceData() { return deviceData; }

  public void setDeviceData(boolean deviceData) { this.deviceData = deviceData; }


  public DeviceDataMaskT() {
    this.trackerData = null;
    this.deviceData = false;
  }
}
