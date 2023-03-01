import { createEffect } from "solid-js";
import { useNavigate } from "solid-start";

export default function Landing() {
  const navigate = useNavigate();

  createEffect(() => {
    const jwt = sessionStorage.getItem("jwt");
    if (jwt) navigate("/app");
    else navigate("/login");
  });

  return <></>;
}
