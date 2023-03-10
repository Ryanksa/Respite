import { createSignal, Show } from "solid-js";
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
  const [items, setItems] = createSignal<ApiItem[]>();

  (async function () {
    try {
      const restRequest: ApiGetRestaurantRequest = {
        restId: params.id,
      };
      const restCall = await apiClient.getRestaurant(restRequest);
      setRestaurant(restCall.response);

      const apiItems: ApiItem[] = [];

      const request: ApiGetItemsRequest = {
        restId: params.id,
        category: "",
      };

      try {
        const call = apiClient.getItems(request);
        for await (const item of call.responses) {
          apiItems.push(item);
        }
        await call.status;
        await call.trailers;
      } catch {}

      setItems(apiItems);
    } catch {}
  })();

  return (
    <Show when={restaurant() && items()}>
      <div class="p-4">
        <Menu restaurant={restaurant()!} items={items()!} />
      </div>
    </Show>
  );
}
