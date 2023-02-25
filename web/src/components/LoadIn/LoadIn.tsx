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
  }, 2600);

  return (
    <Show when={inTransition()} fallback={c()}>
      <div data-theme="dracula" class="overflow-hidden h-screen w-full">
        <div class={`${styles.container} overflow-hidden h-full w-full`}>
          <div class={`${styles.box} ${styles.box1} ${styles.row1}`}>
            <img src={cheeseBoard} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.box2} ${styles.row1}`}>
            <img src={bread} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.box3} ${styles.row1}`}>
            <img src={bruchetta} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.box4} ${styles.row1}`}>
            <img src={steak} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.box5} ${styles.row1}`}>
            <img src={curry} class="h-full" />
          </div>

          <div class={`${styles.box} ${styles.row2} ${styles.box6}`}>
            <img src={kimchiJjigae} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row2} ${styles.box7}`}>
            <img src={hatYai} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row2} ${styles.box8}`}>{c()}</div>
          <div class={`${styles.box} ${styles.row2} ${styles.box9}`}>
            <img src={malvaPudding} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row2} ${styles.box10}`}>
            <img src={wuGeng} class="h-full" />
          </div>

          <div class={`${styles.box} ${styles.row3} ${styles.box1}`}>
            <img src={pho} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row3} ${styles.box2}`}>
            <img src={tsukemen} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row3} ${styles.box3}`}>
            <img src={milkTea} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row3} ${styles.box4}`}>
            <img src={shavedIce} class="h-full" />
          </div>
          <div class={`${styles.box} ${styles.row3} ${styles.box5}`}>
            <img src={macaron} class="h-full" />
          </div>
        </div>
      </div>
    </Show>
  );
}
