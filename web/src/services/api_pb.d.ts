/**
 * Generated via:
 * protoc --grpc-web_out=import_style=typescript,mode=grpcwebtext:../web/src/services --proto_path=. api.proto
 */

import * as jspb from "google-protobuf";

export class ApiOwner extends jspb.Message {
  getId(): string;
  setId(value: string): ApiOwner;

  getEmail(): string;
  setEmail(value: string): ApiOwner;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiOwner.AsObject;
  static toObject(includeInstance: boolean, msg: ApiOwner): ApiOwner.AsObject;
  static serializeBinaryToWriter(
    message: ApiOwner,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiOwner;
  static deserializeBinaryFromReader(
    message: ApiOwner,
    reader: jspb.BinaryReader
  ): ApiOwner;
}

export namespace ApiOwner {
  export type AsObject = {
    id: string;
    email: string;
  };
}

export class ApiRestaurant extends jspb.Message {
  getId(): string;
  setId(value: string): ApiRestaurant;

  getName(): string;
  setName(value: string): ApiRestaurant;

  getDescription(): string;
  setDescription(value: string): ApiRestaurant;

  getLogo(): Uint8Array | string;
  getLogo_asU8(): Uint8Array;
  getLogo_asB64(): string;
  setLogo(value: Uint8Array | string): ApiRestaurant;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiRestaurant.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiRestaurant
  ): ApiRestaurant.AsObject;
  static serializeBinaryToWriter(
    message: ApiRestaurant,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiRestaurant;
  static deserializeBinaryFromReader(
    message: ApiRestaurant,
    reader: jspb.BinaryReader
  ): ApiRestaurant;
}

export namespace ApiRestaurant {
  export type AsObject = {
    id: string;
    name: string;
    description: string;
    logo: Uint8Array | string;
  };
}

export class ApiItem extends jspb.Message {
  getId(): string;
  setId(value: string): ApiItem;

  getName(): string;
  setName(value: string): ApiItem;

  getPrice(): number;
  setPrice(value: number): ApiItem;

  getDescription(): string;
  setDescription(value: string): ApiItem;

  getCategory(): string;
  setCategory(value: string): ApiItem;

  getImage(): Uint8Array | string;
  getImage_asU8(): Uint8Array;
  getImage_asB64(): string;
  setImage(value: Uint8Array | string): ApiItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiItem.AsObject;
  static toObject(includeInstance: boolean, msg: ApiItem): ApiItem.AsObject;
  static serializeBinaryToWriter(
    message: ApiItem,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiItem;
  static deserializeBinaryFromReader(
    message: ApiItem,
    reader: jspb.BinaryReader
  ): ApiItem;
}

export namespace ApiItem {
  export type AsObject = {
    id: string;
    name: string;
    price: number;
    description: string;
    category: string;
    image: Uint8Array | string;
  };
}

export class ApiOrder extends jspb.Message {
  getId(): string;
  setId(value: string): ApiOrder;

  getItemname(): string;
  setItemname(value: string): ApiOrder;

  getRequestedAt(): number;
  setRequestedAt(value: number): ApiOrder;

  getCompleted(): boolean;
  setCompleted(value: boolean): ApiOrder;

  getTablenumber(): number;
  setTablenumber(value: number): ApiOrder;

  getDescription(): string;
  setDescription(value: string): ApiOrder;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiOrder.AsObject;
  static toObject(includeInstance: boolean, msg: ApiOrder): ApiOrder.AsObject;
  static serializeBinaryToWriter(
    message: ApiOrder,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiOrder;
  static deserializeBinaryFromReader(
    message: ApiOrder,
    reader: jspb.BinaryReader
  ): ApiOrder;
}

export namespace ApiOrder {
  export type AsObject = {
    id: string;
    itemname: string;
    requestedAt: number;
    completed: boolean;
    tablenumber: number;
    description: string;
  };
}

export class ApiSignUpRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): ApiSignUpRequest;

  getPassword(): string;
  setPassword(value: string): ApiSignUpRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiSignUpRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiSignUpRequest
  ): ApiSignUpRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiSignUpRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiSignUpRequest;
  static deserializeBinaryFromReader(
    message: ApiSignUpRequest,
    reader: jspb.BinaryReader
  ): ApiSignUpRequest;
}

export namespace ApiSignUpRequest {
  export type AsObject = {
    email: string;
    password: string;
  };
}

export class ApiSignUpResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiSignUpResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiSignUpResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiSignUpResponse
  ): ApiSignUpResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiSignUpResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiSignUpResponse;
  static deserializeBinaryFromReader(
    message: ApiSignUpResponse,
    reader: jspb.BinaryReader
  ): ApiSignUpResponse;
}

export namespace ApiSignUpResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiSignInRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): ApiSignInRequest;

  getPassword(): string;
  setPassword(value: string): ApiSignInRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiSignInRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiSignInRequest
  ): ApiSignInRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiSignInRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiSignInRequest;
  static deserializeBinaryFromReader(
    message: ApiSignInRequest,
    reader: jspb.BinaryReader
  ): ApiSignInRequest;
}

export namespace ApiSignInRequest {
  export type AsObject = {
    email: string;
    password: string;
  };
}

export class ApiSignInResponse extends jspb.Message {
  getOwner(): ApiOwner | undefined;
  setOwner(value?: ApiOwner): ApiSignInResponse;
  hasOwner(): boolean;
  clearOwner(): ApiSignInResponse;

  getJwt(): string;
  setJwt(value: string): ApiSignInResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiSignInResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiSignInResponse
  ): ApiSignInResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiSignInResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiSignInResponse;
  static deserializeBinaryFromReader(
    message: ApiSignInResponse,
    reader: jspb.BinaryReader
  ): ApiSignInResponse;
}

export namespace ApiSignInResponse {
  export type AsObject = {
    owner?: ApiOwner.AsObject;
    jwt: string;
  };
}

export class ApiCreateRestaurantRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiCreateRestaurantRequest;

  getName(): string;
  setName(value: string): ApiCreateRestaurantRequest;

  getDescription(): string;
  setDescription(value: string): ApiCreateRestaurantRequest;

  getImage(): Uint8Array | string;
  getImage_asU8(): Uint8Array;
  getImage_asB64(): string;
  setImage(value: Uint8Array | string): ApiCreateRestaurantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiCreateRestaurantRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiCreateRestaurantRequest
  ): ApiCreateRestaurantRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiCreateRestaurantRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiCreateRestaurantRequest;
  static deserializeBinaryFromReader(
    message: ApiCreateRestaurantRequest,
    reader: jspb.BinaryReader
  ): ApiCreateRestaurantRequest;
}

export namespace ApiCreateRestaurantRequest {
  export type AsObject = {
    jwt: string;
    name: string;
    description: string;
    image: Uint8Array | string;
  };
}

export class ApiCreateRestaurantResponse extends jspb.Message {
  getRestid(): string;
  setRestid(value: string): ApiCreateRestaurantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiCreateRestaurantResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiCreateRestaurantResponse
  ): ApiCreateRestaurantResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiCreateRestaurantResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiCreateRestaurantResponse;
  static deserializeBinaryFromReader(
    message: ApiCreateRestaurantResponse,
    reader: jspb.BinaryReader
  ): ApiCreateRestaurantResponse;
}

export namespace ApiCreateRestaurantResponse {
  export type AsObject = {
    restid: string;
  };
}

export class ApiDeleteRestaurantRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiDeleteRestaurantRequest;

  getRestid(): string;
  setRestid(value: string): ApiDeleteRestaurantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiDeleteRestaurantRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiDeleteRestaurantRequest
  ): ApiDeleteRestaurantRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiDeleteRestaurantRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiDeleteRestaurantRequest;
  static deserializeBinaryFromReader(
    message: ApiDeleteRestaurantRequest,
    reader: jspb.BinaryReader
  ): ApiDeleteRestaurantRequest;
}

export namespace ApiDeleteRestaurantRequest {
  export type AsObject = {
    jwt: string;
    restid: string;
  };
}

export class ApiDeleteRestaurantResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiDeleteRestaurantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiDeleteRestaurantResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiDeleteRestaurantResponse
  ): ApiDeleteRestaurantResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiDeleteRestaurantResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiDeleteRestaurantResponse;
  static deserializeBinaryFromReader(
    message: ApiDeleteRestaurantResponse,
    reader: jspb.BinaryReader
  ): ApiDeleteRestaurantResponse;
}

export namespace ApiDeleteRestaurantResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiUpdateRestaurantRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiUpdateRestaurantRequest;

  getRestid(): string;
  setRestid(value: string): ApiUpdateRestaurantRequest;

  getName(): string;
  setName(value: string): ApiUpdateRestaurantRequest;

  getDescription(): string;
  setDescription(value: string): ApiUpdateRestaurantRequest;

  getImage(): Uint8Array | string;
  getImage_asU8(): Uint8Array;
  getImage_asB64(): string;
  setImage(value: Uint8Array | string): ApiUpdateRestaurantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiUpdateRestaurantRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiUpdateRestaurantRequest
  ): ApiUpdateRestaurantRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiUpdateRestaurantRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiUpdateRestaurantRequest;
  static deserializeBinaryFromReader(
    message: ApiUpdateRestaurantRequest,
    reader: jspb.BinaryReader
  ): ApiUpdateRestaurantRequest;
}

export namespace ApiUpdateRestaurantRequest {
  export type AsObject = {
    jwt: string;
    restid: string;
    name: string;
    description: string;
    image: Uint8Array | string;
  };
}

export class ApiUpdateRestaurantResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiUpdateRestaurantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiUpdateRestaurantResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiUpdateRestaurantResponse
  ): ApiUpdateRestaurantResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiUpdateRestaurantResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiUpdateRestaurantResponse;
  static deserializeBinaryFromReader(
    message: ApiUpdateRestaurantResponse,
    reader: jspb.BinaryReader
  ): ApiUpdateRestaurantResponse;
}

export namespace ApiUpdateRestaurantResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiGetRestaurantRequest extends jspb.Message {
  getRestid(): string;
  setRestid(value: string): ApiGetRestaurantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetRestaurantRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetRestaurantRequest
  ): ApiGetRestaurantRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetRestaurantRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetRestaurantRequest;
  static deserializeBinaryFromReader(
    message: ApiGetRestaurantRequest,
    reader: jspb.BinaryReader
  ): ApiGetRestaurantRequest;
}

export namespace ApiGetRestaurantRequest {
  export type AsObject = {
    restid: string;
  };
}

export class ApiGetRestaurantResponse extends jspb.Message {
  getRestaurant(): ApiRestaurant | undefined;
  setRestaurant(value?: ApiRestaurant): ApiGetRestaurantResponse;
  hasRestaurant(): boolean;
  clearRestaurant(): ApiGetRestaurantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetRestaurantResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetRestaurantResponse
  ): ApiGetRestaurantResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetRestaurantResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetRestaurantResponse;
  static deserializeBinaryFromReader(
    message: ApiGetRestaurantResponse,
    reader: jspb.BinaryReader
  ): ApiGetRestaurantResponse;
}

export namespace ApiGetRestaurantResponse {
  export type AsObject = {
    restaurant?: ApiRestaurant.AsObject;
  };
}

export class ApiGetRestaurantsRequest extends jspb.Message {
  getOwnerid(): string;
  setOwnerid(value: string): ApiGetRestaurantsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetRestaurantsRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetRestaurantsRequest
  ): ApiGetRestaurantsRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetRestaurantsRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetRestaurantsRequest;
  static deserializeBinaryFromReader(
    message: ApiGetRestaurantsRequest,
    reader: jspb.BinaryReader
  ): ApiGetRestaurantsRequest;
}

export namespace ApiGetRestaurantsRequest {
  export type AsObject = {
    ownerid: string;
  };
}

export class ApiGetRestaurantsResponse extends jspb.Message {
  getRestaurantsList(): Array<ApiRestaurant>;
  setRestaurantsList(value: Array<ApiRestaurant>): ApiGetRestaurantsResponse;
  clearRestaurantsList(): ApiGetRestaurantsResponse;
  addRestaurants(value?: ApiRestaurant, index?: number): ApiRestaurant;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetRestaurantsResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetRestaurantsResponse
  ): ApiGetRestaurantsResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetRestaurantsResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetRestaurantsResponse;
  static deserializeBinaryFromReader(
    message: ApiGetRestaurantsResponse,
    reader: jspb.BinaryReader
  ): ApiGetRestaurantsResponse;
}

export namespace ApiGetRestaurantsResponse {
  export type AsObject = {
    restaurantsList: Array<ApiRestaurant.AsObject>;
  };
}

export class ApiAddItemRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiAddItemRequest;

  getRestid(): string;
  setRestid(value: string): ApiAddItemRequest;

  getName(): string;
  setName(value: string): ApiAddItemRequest;

  getPrice(): number;
  setPrice(value: number): ApiAddItemRequest;

  getDescription(): string;
  setDescription(value: string): ApiAddItemRequest;

  getCategory(): string;
  setCategory(value: string): ApiAddItemRequest;

  getImage(): Uint8Array | string;
  getImage_asU8(): Uint8Array;
  getImage_asB64(): string;
  setImage(value: Uint8Array | string): ApiAddItemRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiAddItemRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiAddItemRequest
  ): ApiAddItemRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiAddItemRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiAddItemRequest;
  static deserializeBinaryFromReader(
    message: ApiAddItemRequest,
    reader: jspb.BinaryReader
  ): ApiAddItemRequest;
}

export namespace ApiAddItemRequest {
  export type AsObject = {
    jwt: string;
    restid: string;
    name: string;
    price: number;
    description: string;
    category: string;
    image: Uint8Array | string;
  };
}

export class ApiAddItemResponse extends jspb.Message {
  getItemid(): string;
  setItemid(value: string): ApiAddItemResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiAddItemResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiAddItemResponse
  ): ApiAddItemResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiAddItemResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiAddItemResponse;
  static deserializeBinaryFromReader(
    message: ApiAddItemResponse,
    reader: jspb.BinaryReader
  ): ApiAddItemResponse;
}

export namespace ApiAddItemResponse {
  export type AsObject = {
    itemid: string;
  };
}

export class ApiRemoveItemRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiRemoveItemRequest;

  getItemid(): string;
  setItemid(value: string): ApiRemoveItemRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiRemoveItemRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiRemoveItemRequest
  ): ApiRemoveItemRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiRemoveItemRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiRemoveItemRequest;
  static deserializeBinaryFromReader(
    message: ApiRemoveItemRequest,
    reader: jspb.BinaryReader
  ): ApiRemoveItemRequest;
}

export namespace ApiRemoveItemRequest {
  export type AsObject = {
    jwt: string;
    itemid: string;
  };
}

export class ApiRemoveItemResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiRemoveItemResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiRemoveItemResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiRemoveItemResponse
  ): ApiRemoveItemResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiRemoveItemResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiRemoveItemResponse;
  static deserializeBinaryFromReader(
    message: ApiRemoveItemResponse,
    reader: jspb.BinaryReader
  ): ApiRemoveItemResponse;
}

export namespace ApiRemoveItemResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiUpdateItemRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiUpdateItemRequest;

  getItemid(): string;
  setItemid(value: string): ApiUpdateItemRequest;

  getName(): string;
  setName(value: string): ApiUpdateItemRequest;

  getPrice(): number;
  setPrice(value: number): ApiUpdateItemRequest;

  getDescription(): string;
  setDescription(value: string): ApiUpdateItemRequest;

  getCategory(): string;
  setCategory(value: string): ApiUpdateItemRequest;

  getImage(): Uint8Array | string;
  getImage_asU8(): Uint8Array;
  getImage_asB64(): string;
  setImage(value: Uint8Array | string): ApiUpdateItemRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiUpdateItemRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiUpdateItemRequest
  ): ApiUpdateItemRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiUpdateItemRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiUpdateItemRequest;
  static deserializeBinaryFromReader(
    message: ApiUpdateItemRequest,
    reader: jspb.BinaryReader
  ): ApiUpdateItemRequest;
}

export namespace ApiUpdateItemRequest {
  export type AsObject = {
    jwt: string;
    itemid: string;
    name: string;
    price: number;
    description: string;
    category: string;
    image: Uint8Array | string;
  };
}

export class ApiUpdateItemResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiUpdateItemResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiUpdateItemResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiUpdateItemResponse
  ): ApiUpdateItemResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiUpdateItemResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiUpdateItemResponse;
  static deserializeBinaryFromReader(
    message: ApiUpdateItemResponse,
    reader: jspb.BinaryReader
  ): ApiUpdateItemResponse;
}

export namespace ApiUpdateItemResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiGetItemRequest extends jspb.Message {
  getItemid(): string;
  setItemid(value: string): ApiGetItemRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetItemRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetItemRequest
  ): ApiGetItemRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetItemRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetItemRequest;
  static deserializeBinaryFromReader(
    message: ApiGetItemRequest,
    reader: jspb.BinaryReader
  ): ApiGetItemRequest;
}

export namespace ApiGetItemRequest {
  export type AsObject = {
    itemid: string;
  };
}

export class ApiGetItemResponse extends jspb.Message {
  getItem(): ApiItem | undefined;
  setItem(value?: ApiItem): ApiGetItemResponse;
  hasItem(): boolean;
  clearItem(): ApiGetItemResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetItemResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetItemResponse
  ): ApiGetItemResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetItemResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetItemResponse;
  static deserializeBinaryFromReader(
    message: ApiGetItemResponse,
    reader: jspb.BinaryReader
  ): ApiGetItemResponse;
}

export namespace ApiGetItemResponse {
  export type AsObject = {
    item?: ApiItem.AsObject;
  };
}

export class ApiGetItemsRequest extends jspb.Message {
  getRestid(): string;
  setRestid(value: string): ApiGetItemsRequest;

  getCategory(): string;
  setCategory(value: string): ApiGetItemsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetItemsRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetItemsRequest
  ): ApiGetItemsRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetItemsRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetItemsRequest;
  static deserializeBinaryFromReader(
    message: ApiGetItemsRequest,
    reader: jspb.BinaryReader
  ): ApiGetItemsRequest;
}

export namespace ApiGetItemsRequest {
  export type AsObject = {
    restid: string;
    category: string;
  };
}

export class ApiGetItemsResponse extends jspb.Message {
  getItemsList(): Array<ApiItem>;
  setItemsList(value: Array<ApiItem>): ApiGetItemsResponse;
  clearItemsList(): ApiGetItemsResponse;
  addItems(value?: ApiItem, index?: number): ApiItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetItemsResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetItemsResponse
  ): ApiGetItemsResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetItemsResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetItemsResponse;
  static deserializeBinaryFromReader(
    message: ApiGetItemsResponse,
    reader: jspb.BinaryReader
  ): ApiGetItemsResponse;
}

export namespace ApiGetItemsResponse {
  export type AsObject = {
    itemsList: Array<ApiItem.AsObject>;
  };
}

export class ApiMakeOrderRequest extends jspb.Message {
  getItemid(): string;
  setItemid(value: string): ApiMakeOrderRequest;

  getTablenumber(): number;
  setTablenumber(value: number): ApiMakeOrderRequest;

  getDescription(): string;
  setDescription(value: string): ApiMakeOrderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiMakeOrderRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiMakeOrderRequest
  ): ApiMakeOrderRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiMakeOrderRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiMakeOrderRequest;
  static deserializeBinaryFromReader(
    message: ApiMakeOrderRequest,
    reader: jspb.BinaryReader
  ): ApiMakeOrderRequest;
}

export namespace ApiMakeOrderRequest {
  export type AsObject = {
    itemid: string;
    tablenumber: number;
    description: string;
  };
}

export class ApiMakeOrderResponse extends jspb.Message {
  getOrderid(): string;
  setOrderid(value: string): ApiMakeOrderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiMakeOrderResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiMakeOrderResponse
  ): ApiMakeOrderResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiMakeOrderResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiMakeOrderResponse;
  static deserializeBinaryFromReader(
    message: ApiMakeOrderResponse,
    reader: jspb.BinaryReader
  ): ApiMakeOrderResponse;
}

export namespace ApiMakeOrderResponse {
  export type AsObject = {
    orderid: string;
  };
}

export class ApiCompleteOrderRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiCompleteOrderRequest;

  getOrderid(): string;
  setOrderid(value: string): ApiCompleteOrderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiCompleteOrderRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiCompleteOrderRequest
  ): ApiCompleteOrderRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiCompleteOrderRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiCompleteOrderRequest;
  static deserializeBinaryFromReader(
    message: ApiCompleteOrderRequest,
    reader: jspb.BinaryReader
  ): ApiCompleteOrderRequest;
}

export namespace ApiCompleteOrderRequest {
  export type AsObject = {
    jwt: string;
    orderid: string;
  };
}

export class ApiCompleteOrderResponse extends jspb.Message {
  getSuccess(): boolean;
  setSuccess(value: boolean): ApiCompleteOrderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiCompleteOrderResponse.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiCompleteOrderResponse
  ): ApiCompleteOrderResponse.AsObject;
  static serializeBinaryToWriter(
    message: ApiCompleteOrderResponse,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiCompleteOrderResponse;
  static deserializeBinaryFromReader(
    message: ApiCompleteOrderResponse,
    reader: jspb.BinaryReader
  ): ApiCompleteOrderResponse;
}

export namespace ApiCompleteOrderResponse {
  export type AsObject = {
    success: boolean;
  };
}

export class ApiGetOrdersRequest extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): ApiGetOrdersRequest;

  getRestid(): string;
  setRestid(value: string): ApiGetOrdersRequest;

  getSince(): number;
  setSince(value: number): ApiGetOrdersRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiGetOrdersRequest.AsObject;
  static toObject(
    includeInstance: boolean,
    msg: ApiGetOrdersRequest
  ): ApiGetOrdersRequest.AsObject;
  static serializeBinaryToWriter(
    message: ApiGetOrdersRequest,
    writer: jspb.BinaryWriter
  ): void;
  static deserializeBinary(bytes: Uint8Array): ApiGetOrdersRequest;
  static deserializeBinaryFromReader(
    message: ApiGetOrdersRequest,
    reader: jspb.BinaryReader
  ): ApiGetOrdersRequest;
}

export namespace ApiGetOrdersRequest {
  export type AsObject = {
    jwt: string;
    restid: string;
    since: number;
  };
}
