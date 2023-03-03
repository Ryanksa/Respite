import { useNavigate } from "solid-start";
import { sessionStore } from "~/lib/stores";

export default function Landing() {
  const navigate = useNavigate();

  if (sessionStore.jwt === "") {
    navigate("/login");
  } else {
    navigate("/app");
  }

  return <></>;
}
