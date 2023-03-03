import { createStore } from "solid-js/store";
import { Session } from "./types";

export const [sessionStore, setSessionStore] = createStore<Session>({
  jwt: "",
  owner: {
    id: "",
    email: "",
  },
});
