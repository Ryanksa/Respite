import { createSignal, createResource } from "solid-js";
import { useParams, createRouteAction } from "solid-start";
import { sessionStore } from "~/lib/stores";
import { apiClient } from "~/services/api";
import {
  ApiGetItemsRequest,
  ApiItem,
  ApiAddItemRequest,
} from "~/services/proto/api";

export default function Restaurant() {
  const params = useParams<{ id: string }>();
  const [name, setName] = createSignal("");
  const [price, setPrice] = createSignal(0);
  const [description, setDescription] = createSignal("");
  const [category, setCategory] = createSignal("");
  const [image, setImage] = createSignal(new Uint8Array());

  const [menuItems, { mutate, refetch }] = createResource(
    () => params.id,
    async () => {
      const items: ApiItem[] = [];

      const request: ApiGetItemsRequest = {
        restId: params.id,
        category: "",
      };

      try {
        const call = apiClient.getItems(request);
        for await (const item of call.responses) {
          items.push(item);
        }
        await call.status;
        await call.trailers;
      } catch {}

      return items;
    }
  );

  const [addingItem, addItem] = createRouteAction(async () => {
    const request: ApiAddItemRequest = {
      jwt: sessionStore.jwt,
      restId: params.id,
      name: name(),
      price: price(),
      description: description(),
      category: category(),
      image: image(),
    };

    try {
      await apiClient.addItem(request);
      refetch();
    } catch {}
  });

  return <></>;
}
