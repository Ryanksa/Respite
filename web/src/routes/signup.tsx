import { createSignal, Show } from "solid-js";
import { useNavigate, A } from "solid-start";
import { apiClient } from "~/services/api";
import { ApiSignUpRequest } from "~/services/proto/api";
import Alert from "~/components/Alert";

export default function SignUp() {
  const [step, setStep] = createSignal(0);
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");

  const navigate = useNavigate();

  const signUp = () => {
    const request: ApiSignUpRequest = {
      email: email(),
      password: password(),
    };

    apiClient
      .signUp(request)
      .then(() => {
        navigate("/login");
      })
      .catch(() => {
        setError("Invalid email or password!");
      });
  };

  return (
    <main data-theme="winter" class="p-8 flex flex-col gap-16">
      <A class="logo w-max text-2xl" href="/login">
        Respite
      </A>
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
      <Alert message={error()} dismiss={() => setError("")} />
    </main>
  );
}
