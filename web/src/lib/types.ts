import { ApiOwner } from "~/services/proto/api";

export type Session = {
  jwt: string;
  owner: ApiOwner;
};
