import { createSignal, Show } from "solid-js";
import { apiClient } from "~/services/api";
import { ApiSignUpRequest } from "~/services/proto/api";

export default function SignUp() {
  const [step, setStep] = createSignal(0);
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  const signUp = () => {
    const request: ApiSignUpRequest = {
      email: email(),
      password: password(),
    };

    apiClient
      .signUp(request)
      .then((unaryCall) => {
        console.log(unaryCall.status);
        console.log(unaryCall.response.success);
      })
      .catch((err) => {
        console.error(err);
      });
  };

  return (
    <main data-theme="winter" class="p-8 flex flex-col gap-16">
      <h1 class="logo w-max text-2xl">Respite</h1>
      <Show when={step() === 0}>
        <div class="flex flex-col gap-8 items-center w-full max-w-sm m-auto">
          <div class="form-control w-full">
            <label class="label text-lg mb-3">Sign up to get started.</label>
            <label class="input-group input-group-vertical">
              <span class="py-2 text-secondary font-bold">Email</span>
              <input
                type="text"
                placeholder="business@mail.com"
                class="input input-bordered"
                onChange={(e) => setEmail(e.currentTarget.value)}
                value={email()}
              />
            </label>
          </div>
          <button
            class="btn w-full max-w-xs m-auto opacity-90"
            onClick={() => setStep(step() + 1)}
          >
            Next
          </button>
        </div>
      </Show>
      <Show when={step() === 1}>
        <div class="flex flex-col gap-8 items-center w-full max-w-sm m-auto">
          <div class="form-control w-full">
            <label class="label text-lg mb-3">
              Almost done! Just need a password now.
            </label>
            <label class="input-group input-group-vertical">
              <span class="py-2 text-secondary font-bold">Password</span>
              <input
                type="password"
                class="input input-bordered"
                placeholder="Type here"
                onChange={(e) => setPassword(e.currentTarget.value)}
                value={password()}
              />
            </label>
          </div>
          <button
            class="btn w-full max-w-xs m-auto opacity-90"
            onClick={signUp}
          >
            Finish
          </button>
        </div>
      </Show>
    </main>
  );
}
