import { useNavigate, A, Outlet } from "solid-start";
import { HiSolidMenuAlt3 } from "solid-icons/hi";
import { sessionStore } from "~/lib/stores";

export default function AppLayout() {
  const navigate = useNavigate();

  if (sessionStore.jwt === "") {
    navigate("/login");
  }

  return (
    <main data-theme="winter" class="p-8 flex flex-col gap-16">
      <div class="flex items-center justify-between gap-4">
        <A class="logo w-max text-2xl" href="/">
          Respite
        </A>
        <HiSolidMenuAlt3 class="w-12 h-12 p-2 cursor-pointer rounded-full hover:bg-primary/10 hover:scale-125 transition-all" />
      </div>
      <Outlet />
    </main>
  );
}
