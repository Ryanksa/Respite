import { createSignal, createEffect, createResource, Show } from "solid-js";
import { useParams } from "solid-start";
import Menu from "~/components/Menu";
import { sessionStore, restaurantsStore } from "~/lib/stores";
import { apiClient } from "~/services/api";
import {
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

  createEffect(() => {
    setRestaurant(restaurantsStore.find((rest) => rest.id === params.id));
  });

  const [items, { mutate, refetch }] = createResource(
    () => params.id,
    async (restId) => {
      const apiItems: ApiItem[] = [];

      const request: ApiGetItemsRequest = {
        restId: restId,
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

      return apiItems;
    }
  );

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
      await apiClient.addItem(request);
      await refetch();
    } catch {}
  };

  return (
    <Show when={restaurant() && items()}>
      <Menu
        restaurant={restaurant() as ApiRestaurant}
        items={items() as ApiItem[]}
        addItem={addItem}
      />
    </Show>
  );
}
