import TransitionIn from "~/components/TransitionIn";

export default function Home() {
  return (
    <>
      <TransitionIn />
      <main data-theme="winter" class="p-8 flex flex-col gap-16">
        <h1 class="logo w-max text-2xl">Respite</h1>
        <div class="flex flex-col gap-8 items-center w-full sm:m-auto sm:w-3/4 lg:w-1/2">
          <div class="form-control w-full">
            <label class="label text-lg">Sign up to get started!</label>
            <label class="input-group input-group-vertical">
              <span class="py-2 text-secondary font-bold">Email</span>
              <input
                type="text"
                placeholder="business@mail.com"
                class="input input-bordered"
              />
            </label>
          </div>
          <button class="btn w-full max-w-xs m-auto opacity-90">Next</button>
        </div>
      </main>
    </>
  );
}
