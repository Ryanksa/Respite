import { createSignal, createEffect, Show, For } from "solid-js";
import {
  useNavigate,
  A,
  Outlet,
  useRouteData,
  createRouteData,
} from "solid-start";
import { HiSolidMenuAlt3 } from "solid-icons/hi";
import { RiSystemMenuUnfoldLine } from "solid-icons/ri";
import { AiOutlineLogout } from "solid-icons/ai";
import { sessionStore, setSessionStore } from "~/lib/stores";
import { apiClient } from "~/services/api";
import {
  ApiGetRestaurantsRequest,
  ApiRestaurant,
  ApiOwner,
} from "~/services/proto/api";

export function routeData() {
  return createRouteData(async () => {
    const restaurants: ApiRestaurant[] = [];

    const request: ApiGetRestaurantsRequest = {
      ownerId: sessionStore.owner.id,
    };

    try {
      const call = apiClient.getRestaurants(request);
      for await (const restaurant of call.responses) {
        restaurants.push(restaurant);
      }
      await call.status;
      await call.trailers;
    } catch {}

    return restaurants;
  });
}

export default function AppLayout() {
  const restaurants = useRouteData<typeof routeData>();

  const [expanded, setExpanded] = createSignal(true);
  const navigate = useNavigate();

  createEffect(() => {
    const jwt = sessionStorage.getItem("jwt");
    const owner = sessionStorage.getItem("owner");

    if (jwt && owner) {
      const apiOwner = JSON.parse(owner) as ApiOwner;
      setSessionStore({
        jwt: jwt,
        owner: apiOwner,
      });
    }

    if (!sessionStore.jwt) {
      navigate("/login");
    }
  });

  const logout = () => {
    sessionStorage.removeItem("jwt");
    sessionStorage.removeItem("owner");
    setSessionStore({
      jwt: "",
      owner: { id: "", email: "" },
    });
    navigate("/");
  };

  return (
    <main data-theme="winter" class="p-8 flex flex-col gap-16">
      <div class="flex items-center justify-between gap-4 h-12">
        <A class="logo w-max text-2xl" href="/">
          Respite
        </A>
        <Show
          when={expanded()}
          fallback={
            <HiSolidMenuAlt3
              class="w-12 h-12 p-2 cursor-pointer rounded-full hover:bg-primary/10 hover:scale-125 transition-all"
              onClick={() => setExpanded(true)}
            />
          }
        >
          <div class="fixed top-0 right-0 h-screen w-full sm:w-64 flex flex-col gap-6 px-4 py-8 bg-gradient-to-r from-neutral-focus to-neutral/90 z-50">
            <h3 class="text-neutral-content text-center text-xl font-semibold">
              My Restaurants
            </h3>
            <For each={restaurants()}>
              {(restaurant) => {
                const imgUrl = URL.createObjectURL(
                  new Blob([restaurant.logo], {
                    type: "image/*",
                  })
                );
                return (
                  <A
                    href={`/app/${restaurant.id}`}
                    class="py-2 px-4 rounded-md hover:bg-primary-content/25 flex gap-3"
                    onClick={() => setExpanded(false)}
                  >
                    <img src={imgUrl} class="w-16 object-contain rounded" />
                    <div>
                      <h4 class="text-xl font-semibold text-primary">
                        {restaurant.name}
                      </h4>
                      <div class="text-xs text-neutral-content/75">
                        {restaurant.description}
                      </div>
                    </div>
                  </A>
                );
              }}
            </For>
            <A
              href="/app"
              class="btn btn-secondary btn-sm"
              onClick={() => setExpanded(false)}
            >
              new restaurant
            </A>
            <div class="mt-auto flex items-center justify-evenly">
              <RiSystemMenuUnfoldLine
                class="w-12 h-12 p-2 cursor-pointer rounded-full hover:bg-neutral-content/30 hover:scale-125 transition-all fill-neutral-content"
                onClick={() => setExpanded(false)}
              />
              <AiOutlineLogout
                class="w-12 h-12 p-2 cursor-pointer rounded-full hover:bg-neutral-content/30 hover:scale-125 transition-all fill-neutral-content"
                onClick={logout}
              />
            </div>
            <svg
              viewBox="0 0 1200 120"
              preserveAspectRatio="none"
              class="hidden sm:block rotate-90 fill-neutral-focus scale-150 absolute -left-[21.5rem] top-72"
            >
              <path d="M321.39,56.44c58-10.79,114.16-30.13,172-41.86,82.39-16.72,168.19-17.73,250.45-.39C823.78,31,906.67,72,985.66,92.83c70.05,18.48,146.53,26.09,214.34,3V0H0V27.35A600.21,600.21,0,0,0,321.39,56.44Z" />
            </svg>
          </div>
        </Show>
      </div>
      <Outlet />
    </main>
  );
}
