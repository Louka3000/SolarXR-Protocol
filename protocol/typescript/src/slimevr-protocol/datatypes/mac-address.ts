// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class MacAddress {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):MacAddress {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

byte0():number {
  return this.bb!.readUint8(this.bb_pos);
}

byte1():number {
  return this.bb!.readUint8(this.bb_pos + 1);
}

byte2():number {
  return this.bb!.readUint8(this.bb_pos + 2);
}

byte3():number {
  return this.bb!.readUint8(this.bb_pos + 3);
}

byte4():number {
  return this.bb!.readUint8(this.bb_pos + 4);
}

byte5():number {
  return this.bb!.readUint8(this.bb_pos + 5);
}

static sizeOf():number {
  return 6;
}

static createMacAddress(builder:flatbuffers.Builder, byte_0: number, byte_1: number, byte_2: number, byte_3: number, byte_4: number, byte_5: number):flatbuffers.Offset {
  builder.prep(1, 6);
  builder.writeInt8(byte_5);
  builder.writeInt8(byte_4);
  builder.writeInt8(byte_3);
  builder.writeInt8(byte_2);
  builder.writeInt8(byte_1);
  builder.writeInt8(byte_0);
  return builder.offset();
}

}