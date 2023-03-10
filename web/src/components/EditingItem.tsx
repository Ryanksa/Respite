import { createSignal } from "solid-js";
import { AiFillFileImage } from "solid-icons/ai";

type Props = {
  addItem: (
    name: string,
    price: number,
    description: string,
    category: string,
    image: Uint8Array
  ) => void;
};

export default function EditingItem({ addItem }: Props) {
  const [name, setName] = createSignal("");
  const [price, setPrice] = createSignal(0.0);
  const [description, setDescription] = createSignal("");
  const [category, setCategory] = createSignal("");
  const [image, setImage] = createSignal(new Uint8Array());
  let preview: HTMLImageElement;

  const submit = () => {
    addItem(name(), price(), description(), category(), image());
    setName("");
    setPrice(0.0);
    setDescription("");
    setCategory("");
    setImage(new Uint8Array());
  };

  return (
    <div class="bg-base-300 p-4 rounded-lg">
      <div class="flex gap-4 justify-between mb-4">
        <div class="flex flex-col gap-3 w-1/2">
          <input
            type="text"
            placeholder="Name"
            class="input w-full max-w-sm text-xl"
            value={name()}
            onChange={(e) => {
              setName(e.currentTarget.value);
            }}
          />
          <textarea
            class="textarea w-full max-w-sm h-24"
            placeholder="Description"
            value={description()}
            onChange={(e) => {
              setDescription(e.currentTarget.value);
            }}
          />
          <label class="input-group input-group-sm">
            <span class="bg-base-200">$</span>
            <input
              type="number"
              class="input input-sm w-24"
              value={price()}
              onChange={(e) => {
                setPrice(e.currentTarget.valueAsNumber);
              }}
            />
          </label>
        </div>
        <div class="flex flex-col items-center gap-4 w-1/2 relative -top-10">
          <label class="input-group input-group-vertical">
            <span class="bg-base-300">Category</span>
            <input
              type="text"
              class="input"
              value={category()}
              onChange={(e) => {
                setCategory(e.currentTarget.value);
              }}
            />
          </label>
          <div>
            <div class="grid place-items-center">
              {image().length > 0 ? (
                <img
                  ref={(el) => (preview = el)}
                  class="max-w-[12rem] object-contain"
                />
              ) : (
                <AiFillFileImage class="w-24 h-24 opacity-25" />
              )}
            </div>
            <input
              type="file"
              class="file-input file-input-bordered file-input-sm w-full"
              onChange={(e) => {
                if (e.currentTarget.files && e.currentTarget.files.length > 0) {
                  const file = e.currentTarget.files[0];
                  const reader = new FileReader();

                  reader.readAsArrayBuffer(file);
                  reader.onload = () => {
                    const imageUint8array = new Uint8Array(
                      reader.result as ArrayBuffer
                    );
                    setImage(imageUint8array);

                    reader.readAsDataURL(file);
                    reader.onload = () => {
                      preview.src = reader.result as string;
                    };
                  };
                }
              }}
            />
          </div>
        </div>
      </div>
      <button class="btn btn-outline btn-primary w-full" onClick={submit}>
        Add Item
      </button>
    </div>
  );
}
