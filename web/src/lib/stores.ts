import { createStore } from "solid-js/store";
import { ApiRestaurant } from "~/services/proto/api";
import { Session } from "./types";

export const [sessionStore, setSessionStore] = createStore<Session>({
  jwt: "",
  owner: {
    id: "",
    email: "",
  },
});

export const [restaurantsStore, setRestaurantsStore] = createStore<
  ApiRestaurant[]
>([]);
