import { createEffect } from "solid-js";
import { useNavigate } from "solid-start";

export default function Root() {
  const navigate = useNavigate();

  createEffect(() => {
    if (sessionStorage.getItem("jwt")) {
      navigate("/app");
    } else {
      navigate("/login");
    }
  });

  return <></>;
}
