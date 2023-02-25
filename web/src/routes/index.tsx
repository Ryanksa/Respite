import { createSignal, Show } from "solid-js";
import LoadIn from "~/components/LoadIn";

export default function Home() {
  const [loadedIn, setLoadedIn] = createSignal(false);

  return (
    <>
      <main data-theme="winter"></main>
      <Show when={!loadedIn()}>
        <LoadIn onComplete={() => setLoadedIn(true)} />
      </Show>
    </>
  );
}
