import { createSignal } from "solid-js";
import { useNavigate, A, createRouteAction } from "solid-start";
import { apiClient } from "~/services/api";
import { ApiSignInRequest } from "~/services/proto/api";
import TransitionIn from "~/components/TransitionIn";
import Alert from "~/components/Alert";
import { setSessionStore } from "~/lib/stores";

export default function Login() {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");
  const navigate = useNavigate();

  const [loggingIn, login] = createRouteAction(async () => {
    const request: ApiSignInRequest = {
      email: email(),
      password: password(),
    };

    try {
      const unaryCall = await apiClient.signIn(request);
      const session = {
        jwt: unaryCall.response.jwt,
        owner: {
          id: unaryCall.response.owner!.id,
          email: unaryCall.response.owner!.email,
        },
      };

      sessionStorage.setItem("jwt", session.jwt);
      sessionStorage.setItem("owner", JSON.stringify(session.owner));
      setSessionStore(session);

      navigate("/app");
    } catch {
      setError("Invalid email or password!");
    }
  });

  return (
    <>
      <TransitionIn />
      <main data-theme="winter" class="p-8 flex flex-col gap-16">
        <A class="logo w-max text-2xl" href="">
          Respite
        </A>
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
          <button
            class="btn w-full max-w-xs m-auto opacity-90"
            onClick={() => login()}
            disabled={loggingIn.pending}
          >
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
