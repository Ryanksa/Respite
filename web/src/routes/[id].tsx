import { Show } from "solid-js";
import { createStore } from "solid-js/store";
import { RouteDataArgs, useParams, useRouteData } from "solid-start";
import { createServerData$ } from "solid-start/server";
import Menu from "~/components/Menu";
import { apiClient } from "~/services/api";
import {
  ApiGetRestaurantRequest,
  ApiItem,
  ApiGetItemsRequest,
  ApiRestaurant,
} from "~/services/proto/api";

export function routeData({ params }: RouteDataArgs) {
  return createServerData$(
    async (restId) => {
      const request: ApiGetRestaurantRequest = {
        restId: restId,
      };
      const call = await apiClient.getRestaurant(request);
      return call.response;
    },
    {
      key: () => params.id,
    }
  );
}

export default function RestaurantMenu() {
  const params = useParams<{ id: string }>();
  const restaurant = useRouteData<typeof routeData>();
  const [items, setItems] = createStore<{ [category: string]: ApiItem[] }>({});

  (async function () {
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
      <div class="p-8">
        <Menu restaurant={restaurant() as ApiRestaurant} items={items} />
      </div>
    </Show>
  );
}
