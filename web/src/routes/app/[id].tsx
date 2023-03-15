import { createSignal, createEffect, Show } from "solid-js";
import { createStore, reconcile } from "solid-js/store";
import { useParams } from "solid-start";
import Menu from "~/components/Menu";
import { sessionStore, restaurantsStore } from "~/lib/stores";
import { apiClient } from "~/services/api";
import {
  ApiGetItemRequest,
  ApiGetItemsRequest,
  ApiItem,
  ApiAddItemRequest,
  ApiRestaurant,
} from "~/services/proto/api";

export default function Restaurant() {
  const params = useParams<{ id: string }>();
  const [restaurant, setRestaurant] = createSignal(
    restaurantsStore.find((rest) => rest.id === params.id)
  );
  const [items, setItems] = createStore<{ [category: string]: ApiItem[] }>({});
  const [fetchItemId, setFetchItemId] = createSignal("");

  createEffect(async () => {
    setRestaurant(restaurantsStore.find((rest) => rest.id === params.id));

    setItems(reconcile({}));
    const request: ApiGetItemsRequest = {
      restId: params.id,
      category: "",
    };
    try {
      const call = apiClient.getItems(request);
      for await (const item of call.responses) {
        if (item.category in items) {
          setItems(item.category, (prev) => [...prev, item]);
        } else {
          setItems(item.category, [item]);
        }
      }
      await call.status;
      await call.trailers;
    } catch {}
  });

  createEffect(async () => {
    const request: ApiGetItemRequest = {
      itemId: fetchItemId(),
    };
    const call = await apiClient.getItem(request);
    const item = call.response;
    if (item.category in items) {
      setItems(item.category, (prev) => [...prev, item]);
    } else {
      setItems(item.category, [item]);
    }
  });

  const addItem = async (
    name: string,
    price: number,
    description: string,
    category: string,
    image: Uint8Array
  ) => {
    const request: ApiAddItemRequest = {
      jwt: sessionStore.jwt,
      restId: params.id,
      name: name,
      price: price,
      description: description,
      category: category,
      image: image,
    };

    try {
      const call = await apiClient.addItem(request);
      setFetchItemId(call.response.itemId);
    } catch {}
  };

  return (
    <Show when={restaurant()}>
      <Menu
        restaurant={restaurant() as ApiRestaurant}
        items={items}
        addItem={addItem}
      />
    </Show>
  );
}
