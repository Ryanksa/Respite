import { For } from "solid-js";
import { ApiRestaurant, ApiItem } from "~/services/proto/api";
import Item from "./Item";
import EditingItem from "./EditingItem";
import { toImageUrl } from "~/lib/utils";

type Props = {
  restaurant: ApiRestaurant;
  items: {
    [category: string]: ApiItem[];
  };
  addItem?: (
    name: string,
    price: number,
    description: string,
    category: string,
    image: Uint8Array
  ) => void;
};

export default function Menu(props: Props) {
  return (
    <div class="flex flex-col gap-8">
      <div class="flex gap-4">
        <img
          src={toImageUrl(props.restaurant.logo)}
          class="w-24 object-contain rounded"
        />
        <div>
          <h4 class="text-3xl font-bold text-base-content">
            {props.restaurant.name}
          </h4>
          <div class="text-sm text-base-content/75">
            {props.restaurant.description}
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-8 mb-8">
        <For each={Object.keys(props.items)}>
          {(category) => (
            <div>
              <h2 class="border-b-2 py-2 px-4 text-3xl font-semibold">
                {category}
              </h2>
              <For each={props.items[category]}>
                {(item) => <Item item={item} />}
              </For>
            </div>
          )}
        </For>
      </div>
      {props.addItem && <EditingItem addItem={props.addItem} />}
    </div>
  );
}
