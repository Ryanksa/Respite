import { createSignal } from "solid-js";
import { createRouteAction } from "solid-start";
import { apiClient } from "~/services/api";
import { ApiCreateRestaurantRequest } from "~/services/proto/api";
import { sessionStore } from "~/lib/stores";
import { AiFillFileImage } from "solid-icons/ai";

export default function Restaurant() {
  const [name, setName] = createSignal("");
  const [description, setDescription] = createSignal("");
  const [logo, setLogo] = createSignal<Uint8Array>(new Uint8Array());
  let preview: HTMLImageElement;

  const [creatingRestaurant, createRestaurant] = createRouteAction(async () => {
    const request: ApiCreateRestaurantRequest = {
      jwt: sessionStore.jwt,
      name: name(),
      description: description(),
      logo: logo(),
    };

    await apiClient.createRestaurant(request);
  });

  return (
    <div class="w-4/5 max-w-2xl m-auto flex flex-col gap-8">
      <h1 class="text-xl text-primary-focus font-semibold">
        Create a Restaurant
      </h1>
      <div class="flex gap-4 flex-col md:flex-row">
        <div class="flex flex-col items-center gap-4">
          <div class="w-11/12 max-w-xs aspect-square border-2 rounded-lg grid place-items-center">
            {logo().length > 0 ? (
              <img ref={(el) => (preview = el)} />
            ) : (
              <AiFillFileImage class="w-4/5 h-4/5 opacity-25" />
            )}
          </div>
          <input
            type="file"
            class="file-input file-input-bordered file-input-sm w-full max-w-xs"
            onChange={(e) => {
              if (e.currentTarget.files && e.currentTarget.files.length > 0) {
                const file = e.currentTarget.files[0];
                const reader = new FileReader();

                reader.readAsArrayBuffer(file);
                reader.onload = () => {
                  const logoUint8array = new Uint8Array(
                    reader.result as ArrayBuffer
                  );
                  setLogo(logoUint8array);

                  reader.readAsDataURL(file);
                  reader.onload = () => {
                    preview.src = reader.result as string;
                  };
                };
              }
            }}
          />
        </div>
        <div class="flex flex-col items-center gap-4 w-full">
          <div class="w-full">
            <label class="label pt-0">
              <span class="label-text">Name</span>
            </label>
            <input
              type="text"
              class="input input-bordered w-full"
              value={name()}
              onChange={(e) => setName(e.currentTarget.value)}
            />
          </div>
          <div class="w-full">
            <label class="label w-full">
              <span class="label-text">Description</span>
            </label>
            <textarea
              class="textarea textarea-bordered h-24 w-full"
              value={description()}
              onChange={(e) => setDescription(e.currentTarget.value)}
            />
          </div>
        </div>
      </div>
      <button
        class="btn btn-primary btn-wide self-center"
        onClick={() => createRestaurant()}
        disabled={creatingRestaurant.pending}
      >
        Create
      </button>
    </div>
  );
}
