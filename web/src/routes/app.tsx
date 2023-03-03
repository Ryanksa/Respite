import { createSignal, Show } from "solid-js";
import { useNavigate, A, Outlet } from "solid-start";
import { HiSolidMenuAlt3 } from "solid-icons/hi";
import { RiSystemMenuUnfoldLine } from "solid-icons/ri";
import { AiOutlineLogout } from "solid-icons/ai";
import { sessionStore, setSessionStore } from "~/lib/stores";

export default function AppLayout() {
  const [expanded, setExpanded] = createSignal(false);
  const navigate = useNavigate();

  if (sessionStore.jwt === "") {
    navigate("/login");
  }

  const logout = () => {
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
          <div class="fixed top-0 right-0 h-screen w-full sm:w-64 flex flex-col gap-4 px-4 py-8 bg-gradient-to-r from-neutral to-neutral/90">
            <h3 class="text-neutral-content text-center text-xl font-semibold">
              My Restaurants
            </h3>
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
              class="hidden sm:block rotate-90 fill-neutral scale-150 absolute -left-[21.5rem] top-72"
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
