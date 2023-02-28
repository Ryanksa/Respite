import { createSignal } from "solid-js";
import { A } from "solid-start";
import TransitionIn from "~/components/TransitionIn";
import { apiClient } from "~/services/api";
import { ApiSignInRequest } from "~/services/proto/api";
import Alert from "~/components/Alert";

export default function Login() {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");

  const login = () => {
    const request: ApiSignInRequest = {
      email: email(),
      password: password(),
    };

    apiClient
      .signIn(request)
      .then((unaryCall) => {
        localStorage.setItem("jwt", unaryCall.response.jwt);
        localStorage.setItem("owner", JSON.stringify(unaryCall.response.owner));
        window.location.replace("/");
      })
      .catch(() => {
        setError("Invalid email or password!");
      });
  };

  return (
    <>
      <TransitionIn />
      <main data-theme="winter" class="p-8 flex flex-col gap-16">
        <h1 class="logo w-max text-2xl">Respite</h1>
        <div class="flex flex-col gap-8 items-center w-full">
          <h2 class="text-2xl">Login to your account</h2>
          <div class="form-control flex flex-col gap-3 w-full max-w-xs m-auto">
            <input
              type="text"
              placeholder="Email"
              class="input input-bordered input-md w-full max-w-xs"
              onChange={(e) => setEmail(e.currentTarget.value)}
              value={email()}
            />
            <input
              type="password"
              placeholder="Password"
              class="input input-bordered input-md w-full max-w-xs"
              onChange={(e) => setPassword(e.currentTarget.value)}
              value={password()}
            />
          </div>
          <button class="btn w-full max-w-xs m-auto opacity-90" onClick={login}>
            Login
          </button>
        </div>
        <div class="w-full max-w-xs m-auto">
          First time? Create an account{" "}
          <A href="/signup" class="text-primary">
            here
          </A>
          !
        </div>
        <Alert message={error()} dismiss={() => setError("")} />
      </main>
    </>
  );
}
