import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { ApiClient } from "./proto/api.client";

const transport = new GrpcWebFetchTransport({
  baseUrl: import.meta.env.VITE_API_URL,
});

export const apiClient = new ApiClient(transport);
