/* eslint-disable */
import { grpc } from "@improbable-eng/grpc-web";
import { BrowserHeaders } from "browser-headers";
import Long from "long";
import _m0 from "protobufjs/minimal";
import { Observable } from "rxjs";
import { share } from "rxjs/operators";

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
  /** The timestamp of the new block. */
  timestamp: number;
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
  return { network: 0, blockNumber: 0, blockHash: "", timestamp: 0, gasFee: 0 };
}

export const BlockUpdate = {
  encode(message: BlockUpdate, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.network !== 0) {
      writer.uint32(8).int32(message.network);
    }
    if (message.blockNumber !== 0) {
      writer.uint32(16).uint64(message.blockNumber);
    }
    if (message.blockHash !== "") {
      writer.uint32(26).string(message.blockHash);
    }
    if (message.timestamp !== 0) {
      writer.uint32(32).uint64(message.timestamp);
    }
    if (message.gasFee !== 0) {
      writer.uint32(41).double(message.gasFee);
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

          message.blockNumber = longToNumber(reader.uint64() as Long);
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.blockHash = reader.string();
          continue;
        case 4:
          if (tag !== 32) {
            break;
          }

          message.timestamp = longToNumber(reader.uint64() as Long);
          continue;
        case 5:
          if (tag !== 41) {
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
      timestamp: isSet(object.timestamp) ? globalThis.Number(object.timestamp) : 0,
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
    if (message.timestamp !== 0) {
      obj.timestamp = Math.round(message.timestamp);
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
    message.timestamp = object.timestamp ?? 0;
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
      writer.uint32(16).uint64(message.startBlock);
    }
    if (message.endBlock !== 0) {
      writer.uint32(24).uint64(message.endBlock);
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

          message.startBlock = longToNumber(reader.uint64() as Long);
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.endBlock = longToNumber(reader.uint64() as Long);
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
  Subscribe(request: DeepPartial<SubscriptionRequest>, metadata?: grpc.Metadata): Observable<BlockUpdate>;
  /** Retrieves gas fee information for a specified span of past blocks. */
  Blocks(request: DeepPartial<BlockRangeRequest>, metadata?: grpc.Metadata): Promise<BlockRangeReply>;
}

export class GasClientImpl implements Gas {
  private readonly rpc: Rpc;

  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.Subscribe = this.Subscribe.bind(this);
    this.Blocks = this.Blocks.bind(this);
  }

  Subscribe(request: DeepPartial<SubscriptionRequest>, metadata?: grpc.Metadata): Observable<BlockUpdate> {
    return this.rpc.invoke(GasSubscribeDesc, SubscriptionRequest.fromPartial(request), metadata);
  }

  Blocks(request: DeepPartial<BlockRangeRequest>, metadata?: grpc.Metadata): Promise<BlockRangeReply> {
    return this.rpc.unary(GasBlocksDesc, BlockRangeRequest.fromPartial(request), metadata);
  }
}

export const GasDesc = { serviceName: "gas.Gas" };

export const GasSubscribeDesc: UnaryMethodDefinitionish = {
  methodName: "Subscribe",
  service: GasDesc,
  requestStream: false,
  responseStream: true,
  requestType: {
    serializeBinary() {
      return SubscriptionRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = BlockUpdate.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const GasBlocksDesc: UnaryMethodDefinitionish = {
  methodName: "Blocks",
  service: GasDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return BlockRangeRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = BlockRangeReply.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
  requestStream: any;
  responseStream: any;
}

type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

interface Rpc {
  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any>;
  invoke<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Observable<any>;
}

export class GrpcWebImpl {
  private host: string;
  private options: {
    transport?: grpc.TransportFactory;
    streamingTransport?: grpc.TransportFactory;
    debug?: boolean;
    metadata?: grpc.Metadata;
    upStreamRetryCodes?: number[];
  };

  constructor(
    host: string,
    options: {
      transport?: grpc.TransportFactory;
      streamingTransport?: grpc.TransportFactory;
      debug?: boolean;
      metadata?: grpc.Metadata;
      upStreamRetryCodes?: number[];
    },
  ) {
    this.host = host;
    this.options = options;
  }

  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any> {
    const request = { ..._request, ...methodDesc.requestType };
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Promise((resolve, reject) => {
      grpc.unary(methodDesc, {
        request,
        host: this.host,
        metadata: maybeCombinedMetadata ?? {},
        ...(this.options.transport !== undefined ? { transport: this.options.transport } : {}),
        debug: this.options.debug ?? false,
        onEnd: function (response) {
          if (response.status === grpc.Code.OK) {
            resolve(response.message!.toObject());
          } else {
            const err = new GrpcWebError(response.statusMessage, response.status, response.trailers);
            reject(err);
          }
        },
      });
    });
  }

  invoke<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Observable<any> {
    const upStreamCodes = this.options.upStreamRetryCodes ?? [];
    const DEFAULT_TIMEOUT_TIME: number = 3_000;
    const request = { ..._request, ...methodDesc.requestType };
    const transport = this.options.streamingTransport ?? this.options.transport;
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Observable((observer) => {
      const upStream = () => {
        const client = grpc.invoke(methodDesc, {
          host: this.host,
          request,
          ...(transport !== undefined ? { transport } : {}),
          metadata: maybeCombinedMetadata ?? {},
          debug: this.options.debug ?? false,
          onMessage: (next) => observer.next(next),
          onEnd: (code: grpc.Code, message: string, trailers: grpc.Metadata) => {
            if (code === 0) {
              observer.complete();
            } else if (upStreamCodes.includes(code)) {
              setTimeout(upStream, DEFAULT_TIMEOUT_TIME);
            } else {
              const err = new Error(message) as any;
              err.code = code;
              err.metadata = trailers;
              observer.error(err);
            }
          },
        });
        observer.add(() => client.close());
      };
      upStream();
    }).pipe(share());
  }
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

export class GrpcWebError extends globalThis.Error {
  constructor(message: string, public code: grpc.Code, public metadata: grpc.Metadata) {
    super(message);
  }
}
