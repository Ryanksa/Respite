import { ApiItem } from "~/services/proto/api";

type Props = {
  item: ApiItem;
};

export default function Item(props: Props) {
  return (
    <div class="px-4 py-2 flex gap-4 justify-between">
      <div>
        <h4 class="text-xl">{props.item.name}</h4>
        <p class="text-xs">{props.item.description}</p>
        <h6>
          <span class="text-sm">$</span>
          <span class="text-lg">{props.item.price}</span>
        </h6>
      </div>
      {props.item.image.length > 0 && (
        <div class="grid place-items-center">
          <img
            src={URL.createObjectURL(
              new Blob([props.item.image], {
                type: "image/*",
              })
            )}
            class="w-24 object-contain rounded"
          />
        </div>
      )}
    </div>
  );
}
