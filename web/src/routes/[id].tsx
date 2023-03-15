import { createSignal, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { useParams } from "solid-start";
import Menu from "~/components/Menu";
import { apiClient } from "~/services/api";
import {
  ApiGetRestaurantRequest,
  ApiItem,
  ApiGetItemsRequest,
  ApiRestaurant,
} from "~/services/proto/api";

export default function RestaurantMenu() {
  const params = useParams<{ id: string }>();
  const [restaurant, setRestaurant] = createSignal<ApiRestaurant>();
  const [items, setItems] = createStore<{ [category: string]: ApiItem[] }>({});

  (async function () {
    const restRequest: ApiGetRestaurantRequest = {
      restId: params.id,
    };
    try {
      const restCall = await apiClient.getRestaurant(restRequest);
      setRestaurant(restCall.response);
    } catch {}

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
  })();

  return (
    <Show when={restaurant()}>
      <div class="p-4">
        <Menu restaurant={restaurant()!} items={items} />
      </div>
    </Show>
  );
}
