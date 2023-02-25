import { createSignal, children, JSX, Show } from "solid-js";
import styles from "./LoadIn.module.css";
import bread from "~/assets/bread.jpg";
import bruchetta from "~/assets/bruchetta.jpg";
import cheeseBoard from "~/assets/cheese-board.jpg";
import curry from "~/assets/curry.jpg";
import hatYai from "~/assets/hat-yai.jpg";
import kimchiJjigae from "~/assets/kimchi-jjigae.jpg";
import macaron from "~/assets/macaron.jpg";
import malvaPudding from "~/assets/malva-pudding.jpg";
import milkTea from "~/assets/milk-tea.jpg";
import pho from "~/assets/pho.jpg";
import shavedIce from "~/assets/shaved-ice.jpg";
import steak from "~/assets/steak.jpg";
import tsukemen from "~/assets/tsukemen.jpg";
import wuGeng from "~/assets/wu-geng.jpg";

type Props = {
  children: JSX.Element;
};

export default function LoadIn(props: Props) {
  const [inTransition, setInTransition] = createSignal(true);
  const c = children(() => props.children);

  setTimeout(() => {
    setInTransition(false);
  }, 5000);

  return (
    <Show when={inTransition()} fallback={c()}>
      <div data-theme="dracula" class="overflow-hidden h-screen w-full">
        <div class={`${styles.container} overflow-hidden h-full w-full`}>
          <div
            class={`${styles.box} ${styles.box1} ${styles.row1} flex rounded-lg overflow-hidden`}
          >
            <img src={cheeseBoard} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.box2} ${styles.row1} flex rounded-lg overflow-hidden`}
          >
            <img src={bread} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.box3} ${styles.row1} flex rounded-lg overflow-hidden`}
          >
            <img src={bruchetta} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.box4} ${styles.row1} flex rounded-lg overflow-hidden`}
          >
            <img src={steak} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.box5} ${styles.row1} flex rounded-lg overflow-hidden`}
          >
            <img src={malvaPudding} class="object-cover m-auto rounded-lg" />
          </div>

          <div
            class={`${styles.box} ${styles.row2} ${styles.box6} flex rounded-lg overflow-hidden`}
          >
            <img src={kimchiJjigae} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row2} ${styles.box7} flex rounded-lg overflow-hidden`}
          >
            <img src={hatYai} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row2} ${styles.box8} rounded-lg overflow-hidden`}
          >
            {c()}
          </div>
          <div
            class={`${styles.box} ${styles.row2} ${styles.box9} flex rounded-lg overflow-hidden`}
          >
            <img src={milkTea} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row2} ${styles.box10} flex rounded-lg overflow-hidden`}
          >
            <img src={shavedIce} class="object-cover m-auto rounded-lg" />
          </div>

          <div
            class={`${styles.box} ${styles.row3} ${styles.box1} flex rounded-lg overflow-hidden`}
          >
            <img src={pho} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row3} ${styles.box2} flex rounded-lg overflow-hidden`}
          >
            <img src={tsukemen} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row3} ${styles.box3} flex rounded-lg overflow-hidden`}
          >
            <img src={wuGeng} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row3} ${styles.box4} flex rounded-lg overflow-hidden`}
          >
            <img src={curry} class="object-cover m-auto rounded-lg" />
          </div>
          <div
            class={`${styles.box} ${styles.row3} ${styles.box5} flex rounded-lg overflow-hidden`}
          >
            <img src={macaron} class="object-cover m-auto rounded-lg" />
          </div>
        </div>
      </div>
    </Show>
  );
}
