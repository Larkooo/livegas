/* eslint-disable */
import Long from "long";
import _m0 from "protobufjs/minimal";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

export const protobufPackage = "gas";

export enum Network {
  ETH_MAINNET = 0,
  UNRECOGNIZED = -1,
}

export function networkFromJSON(object: any): Network {
  switch (object) {
    case 0:
    case "ETH_MAINNET":
      return Network.ETH_MAINNET;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Network.UNRECOGNIZED;
  }
}

export function networkToJSON(object: Network): string {
  switch (object) {
    case Network.ETH_MAINNET:
      return "ETH_MAINNET";
    case Network.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

/** The subscription request message */
export interface SubscriptionRequest {
  /** required */
  network: Network;
}

/** The message containing update information for each new block. */
export interface BlockUpdate {
  /** The network for which the block update is being sent. */
  network: Network;
  /** The block number. */
  blockNumber: number;
  /** Hash of the new block. */
  blockHash: string;
  /** Gas fee for transactions in the new block. */
  gasFee: number;
}

/** The request message for retrieving gas fees for a specified range of blocks. */
export interface BlockRangeRequest {
  /** The network for which to retrieve gas fee information. */
  network: Network;
  /** The starting block number. */
  startBlock: number;
  /** The ending block number. */
  endBlock: number;
}

/** The response message containing a list of block updates within the requested range. */
export interface BlockRangeReply {
  /** A list of block updates for the specified range. */
  blockUpdates: BlockUpdate[];
}

function createBaseSubscriptionRequest(): SubscriptionRequest {
  return { network: 0 };
}

export const SubscriptionRequest = {
  encode(message: SubscriptionRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.network !== 0) {
      writer.uint32(8).int32(message.network);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SubscriptionRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSubscriptionRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.network = reader.int32() as any;
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SubscriptionRequest {
    return { network: isSet(object.network) ? networkFromJSON(object.network) : 0 };
  },

  toJSON(message: SubscriptionRequest): unknown {
    const obj: any = {};
    if (message.network !== 0) {
      obj.network = networkToJSON(message.network);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SubscriptionRequest>, I>>(base?: I): SubscriptionRequest {
    return SubscriptionRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SubscriptionRequest>, I>>(object: I): SubscriptionRequest {
    const message = createBaseSubscriptionRequest();
    message.network = object.network ?? 0;
    return message;
  },
};

function createBaseBlockUpdate(): BlockUpdate {
  return { network: 0, blockNumber: 0, blockHash: "", gasFee: 0 };
}

export const BlockUpdate = {
  encode(message: BlockUpdate, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.network !== 0) {
      writer.uint32(8).int32(message.network);
    }
    if (message.blockNumber !== 0) {
      writer.uint32(16).int64(message.blockNumber);
    }
    if (message.blockHash !== "") {
      writer.uint32(26).string(message.blockHash);
    }
    if (message.gasFee !== 0) {
      writer.uint32(33).double(message.gasFee);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BlockUpdate {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBlockUpdate();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.network = reader.int32() as any;
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.blockNumber = longToNumber(reader.int64() as Long);
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.blockHash = reader.string();
          continue;
        case 4:
          if (tag !== 33) {
            break;
          }

          message.gasFee = reader.double();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BlockUpdate {
    return {
      network: isSet(object.network) ? networkFromJSON(object.network) : 0,
      blockNumber: isSet(object.blockNumber) ? globalThis.Number(object.blockNumber) : 0,
      blockHash: isSet(object.blockHash) ? globalThis.String(object.blockHash) : "",
      gasFee: isSet(object.gasFee) ? globalThis.Number(object.gasFee) : 0,
    };
  },

  toJSON(message: BlockUpdate): unknown {
    const obj: any = {};
    if (message.network !== 0) {
      obj.network = networkToJSON(message.network);
    }
    if (message.blockNumber !== 0) {
      obj.blockNumber = Math.round(message.blockNumber);
    }
    if (message.blockHash !== "") {
      obj.blockHash = message.blockHash;
    }
    if (message.gasFee !== 0) {
      obj.gasFee = message.gasFee;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BlockUpdate>, I>>(base?: I): BlockUpdate {
    return BlockUpdate.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BlockUpdate>, I>>(object: I): BlockUpdate {
    const message = createBaseBlockUpdate();
    message.network = object.network ?? 0;
    message.blockNumber = object.blockNumber ?? 0;
    message.blockHash = object.blockHash ?? "";
    message.gasFee = object.gasFee ?? 0;
    return message;
  },
};

function createBaseBlockRangeRequest(): BlockRangeRequest {
  return { network: 0, startBlock: 0, endBlock: 0 };
}

export const BlockRangeRequest = {
  encode(message: BlockRangeRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.network !== 0) {
      writer.uint32(8).int32(message.network);
    }
    if (message.startBlock !== 0) {
      writer.uint32(16).int64(message.startBlock);
    }
    if (message.endBlock !== 0) {
      writer.uint32(24).int64(message.endBlock);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BlockRangeRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBlockRangeRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.network = reader.int32() as any;
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.startBlock = longToNumber(reader.int64() as Long);
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.endBlock = longToNumber(reader.int64() as Long);
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BlockRangeRequest {
    return {
      network: isSet(object.network) ? networkFromJSON(object.network) : 0,
      startBlock: isSet(object.startBlock) ? globalThis.Number(object.startBlock) : 0,
      endBlock: isSet(object.endBlock) ? globalThis.Number(object.endBlock) : 0,
    };
  },

  toJSON(message: BlockRangeRequest): unknown {
    const obj: any = {};
    if (message.network !== 0) {
      obj.network = networkToJSON(message.network);
    }
    if (message.startBlock !== 0) {
      obj.startBlock = Math.round(message.startBlock);
    }
    if (message.endBlock !== 0) {
      obj.endBlock = Math.round(message.endBlock);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BlockRangeRequest>, I>>(base?: I): BlockRangeRequest {
    return BlockRangeRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BlockRangeRequest>, I>>(object: I): BlockRangeRequest {
    const message = createBaseBlockRangeRequest();
    message.network = object.network ?? 0;
    message.startBlock = object.startBlock ?? 0;
    message.endBlock = object.endBlock ?? 0;
    return message;
  },
};

function createBaseBlockRangeReply(): BlockRangeReply {
  return { blockUpdates: [] };
}

export const BlockRangeReply = {
  encode(message: BlockRangeReply, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.blockUpdates) {
      BlockUpdate.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BlockRangeReply {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBlockRangeReply();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.blockUpdates.push(BlockUpdate.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BlockRangeReply {
    return {
      blockUpdates: globalThis.Array.isArray(object?.blockUpdates)
        ? object.blockUpdates.map((e: any) => BlockUpdate.fromJSON(e))
        : [],
    };
  },

  toJSON(message: BlockRangeReply): unknown {
    const obj: any = {};
    if (message.blockUpdates?.length) {
      obj.blockUpdates = message.blockUpdates.map((e) => BlockUpdate.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BlockRangeReply>, I>>(base?: I): BlockRangeReply {
    return BlockRangeReply.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BlockRangeReply>, I>>(object: I): BlockRangeReply {
    const message = createBaseBlockRangeReply();
    message.blockUpdates = object.blockUpdates?.map((e) => BlockUpdate.fromPartial(e)) || [];
    return message;
  },
};

export interface Gas {
  /** Subscribes to receive updates for upcoming blocks of a specified blockchain. */
  Subscribe(request: SubscriptionRequest): Observable<BlockUpdate>;
  /** Retrieves gas fee information for a specified span of past blocks. */
  Blocks(request: BlockRangeRequest): Promise<BlockRangeReply>;
}

export const GasServiceName = "gas.Gas";
export class GasClientImpl implements Gas {
  private readonly rpc: Rpc;
  private readonly service: string;
  constructor(rpc: Rpc, opts?: { service?: string }) {
    this.service = opts?.service || GasServiceName;
    this.rpc = rpc;
    this.Subscribe = this.Subscribe.bind(this);
    this.Blocks = this.Blocks.bind(this);
  }
  Subscribe(request: SubscriptionRequest): Observable<BlockUpdate> {
    const data = SubscriptionRequest.encode(request).finish();
    const result = this.rpc.serverStreamingRequest(this.service, "Subscribe", data);
    return result.pipe(map((data) => BlockUpdate.decode(_m0.Reader.create(data))));
  }

  Blocks(request: BlockRangeRequest): Promise<BlockRangeReply> {
    const data = BlockRangeRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "Blocks", data);
    return promise.then((data) => BlockRangeReply.decode(_m0.Reader.create(data)));
  }
}

interface Rpc {
  request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
  clientStreamingRequest(service: string, method: string, data: Observable<Uint8Array>): Promise<Uint8Array>;
  serverStreamingRequest(service: string, method: string, data: Uint8Array): Observable<Uint8Array>;
  bidirectionalStreamingRequest(service: string, method: string, data: Observable<Uint8Array>): Observable<Uint8Array>;
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function longToNumber(long: Long): number {
  if (long.gt(globalThis.Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
