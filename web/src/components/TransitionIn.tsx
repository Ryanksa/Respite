import styles from "~/styles/TransitionIn.module.css";

export default function TransitionIn() {
  return (
    <div
      data-theme="dracula"
      class={`${styles.backdrop} overflow-hidden h-screen w-full hidden lg:block`}
    >
      <div class={`${styles.container} overflow-hidden h-full w-full`}>
        <div
          class={`${styles.box} ${styles.box1} ${styles.row1} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/cheese-board.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.box2} ${styles.row1} flex rounded-lg overflow-hidden`}
        >
          <img src="/images/bread.jpg" class="object-cover m-auto rounded-lg" />
        </div>
        <div
          class={`${styles.box} ${styles.box3} ${styles.row1} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/bruchetta.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.box4} ${styles.row1} flex rounded-lg overflow-hidden`}
        >
          <img src="/images/steak.jpg" class="object-cover m-auto rounded-lg" />
        </div>
        <div
          class={`${styles.box} ${styles.box5} ${styles.row1} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/malva-pudding.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>

        <div
          class={`${styles.box} ${styles.row2} ${styles.box6} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/kimchi-jjigae.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row2} ${styles.box7} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/hat-yai.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row2} ${styles.box8} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/restaurant.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row2} ${styles.box9} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/milk-tea.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row2} ${styles.box10} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/shaved-ice.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>

        <div
          class={`${styles.box} ${styles.row3} ${styles.box1} flex rounded-lg overflow-hidden`}
        >
          <img src="/images/pho.jpg" class="object-cover m-auto rounded-lg" />
        </div>
        <div
          class={`${styles.box} ${styles.row3} ${styles.box2} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/tsukemen.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row3} ${styles.box3} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/wu-geng.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
        <div
          class={`${styles.box} ${styles.row3} ${styles.box4} flex rounded-lg overflow-hidden`}
        >
          <img src="/images/curry.jpg" class="object-cover m-auto rounded-lg" />
        </div>
        <div
          class={`${styles.box} ${styles.row3} ${styles.box5} flex rounded-lg overflow-hidden`}
        >
          <img
            src="/images/macaron.jpg"
            class="object-cover m-auto rounded-lg"
          />
        </div>
      </div>
    </div>
  );
}
