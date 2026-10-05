<!--
  Glorp: a green slime alien cat. All colors come from --glorp-* CSS variables,
  so themes can recolor him. The face is assembled from the shared "face kit"
  (src/lib/expression.ts): moods pick a default face, reactions override it.
-->
<script lang="ts">
  import { MOOD_FACES, type Expression } from "$lib/expression";
  import type { MascotProps } from "../index";

  let { mood, cps, look, expression = null, held = false }: MascotProps = $props();

  const face = $derived<Expression>({ ...MOOD_FACES[mood], ...(expression ?? {}) });
  const fx = $derived(new Set(expression?.fx ?? MOOD_FACES[mood].fx ?? []));
  const anim = $derived(expression?.anim ?? MOOD_FACES[mood].anim);

  // One bounce per click at low speeds, capped so fast clicking stays readable.
  const bounce = $derived(Math.min(0.6, Math.max(0.18, 1 / Math.max(cps, 0.1))));
  const lx = $derived(Math.max(-1, Math.min(1, look.x)) * 6);
  const ly = $derived(Math.max(-1, Math.min(1, look.y)) * 5);

  const uid = Math.random().toString(36).slice(2, 8);
  const BODY =
    "M44 176 C36 130 58 68 120 66 C182 68 204 130 196 176 C194 192 182 198 170 194 C168 208 150 212 148 196 C136 200 104 200 92 197 C90 214 70 214 70 196 C58 198 46 192 44 176 Z";
  const STAR = "M0 -15 L3.8 -5.3 L14.3 -4.6 L6.2 2 L8.8 12.1 L0 6.5 L-8.8 12.1 L-6.2 2 L-14.3 -4.6 L-3.8 -5.3 Z";
  const HEART = "M0 -4 C-6 -14 -20 -8 -14 4 C-11 10 -4 14 0 18 C4 14 11 10 14 4 C20 -8 6 -14 0 -4 Z";
  const SPARK = "M0 -9 Q1.5 -1.5 9 0 Q1.5 1.5 0 9 Q-1.5 1.5 -9 0 Q-1.5 -1.5 0 -9 Z";
  const EYES = [96, 144];
</script>

<div
  class="glorp mood-{mood} {anim ? `anim-${anim}` : ''}"
  class:held
  style="--bounce:{bounce}s; --lx:{lx}px; --ly:{ly}px; --blush:{face.blush ?? 0.8}"
>
  <svg viewBox="0 0 240 240" aria-hidden="true">
    <defs>
      <radialGradient id="body-{uid}" cx="40%" cy="30%" r="78%">
        <stop offset="0" style="stop-color: var(--glorp-light)" />
        <stop offset="0.55" style="stop-color: var(--glorp-mid)" />
        <stop offset="1" style="stop-color: var(--glorp-dark)" />
      </radialGradient>
      <radialGradient id="orb-{uid}" cx="40%" cy="35%" r="70%">
        <stop offset="0" stop-color="#ffffff" />
        <stop offset="0.45" style="stop-color: var(--glorp-orb)" />
        <stop offset="1" style="stop-color: var(--glorp-dark)" />
      </radialGradient>
      <radialGradient id="glow-{uid}">
        <stop offset="0" style="stop-color: var(--glorp-orb); stop-opacity: 0.9" />
        <stop offset="1" style="stop-color: var(--glorp-orb); stop-opacity: 0" />
      </radialGradient>
      <clipPath id="clip-{uid}"><path d={BODY} /></clipPath>
      <!-- lower half only: used for half-closed "annoyed" eyes -->
      <clipPath id="lid-{uid}"><rect x="0" y="125" width="240" height="40" /></clipPath>
    </defs>

    <ellipse class="shadow" cx="120" cy="214" rx="66" ry="9" />

    {#key expression?.id}
      <g class="jump">
        <g class="squish">
          <!-- antenna -->
          <g class="antenna">
            <path d="M120 70 C117 52 124 40 131 28" class="stroke" stroke-width="4.5" fill="none" stroke-linecap="round" />
            <circle class="orb-glow" cx="132" cy="24" r="20" fill="url(#glow-{uid})" />
            <circle cx="132" cy="24" r="10" fill="url(#orb-{uid})" class="stroke" stroke-width="3" />
          </g>

          <!-- ears -->
          <g class="ear ear-l">
            <path d="M58 106 C52 82 50 60 56 42 C58 35 65 34 69 38 C81 51 93 63 101 78 Z" class="fill-mid stroke" stroke-width="4" stroke-linejoin="round" />
            <path d="M65 94 C62 78 62 62 65 52 C75 62 83 70 89 80 Z" class="fill-ear" />
          </g>
          <g class="ear ear-r">
            <path d="M182 106 C188 82 190 60 184 42 C182 35 175 34 171 38 C159 51 147 63 139 78 Z" class="fill-mid stroke" stroke-width="4" stroke-linejoin="round" />
            <path d="M175 94 C178 78 178 62 175 52 C165 62 157 70 151 80 Z" class="fill-ear" />
          </g>

          <!-- body -->
          <path d={BODY} fill="url(#body-{uid})" class="stroke" stroke-width="4" stroke-linejoin="round" />

          <!-- little bubbles floating inside the slime -->
          <g clip-path="url(#clip-{uid})" class="inner-bubbles">
            <circle cx="156" cy="188" r="5" />
            <circle cx="174" cy="170" r="3" />
            <circle cx="70" cy="184" r="4" />
            <circle cx="112" cy="192" r="2.5" />
          </g>

          <!-- gloss -->
          <ellipse cx="82" cy="98" rx="17" ry="9" fill="#fff" opacity="0.6" transform="rotate(-32 82 98)" />
          <circle cx="104" cy="86" r="4.5" fill="#fff" opacity="0.6" />

          <g class="face">
            <!-- whiskers -->
            <g class="whiskers stroke" stroke-width="2.5" stroke-linecap="round" opacity="0.45">
              <path d="M58 146 L38 141" /><path d="M58 154 L37 157" />
              <path d="M182 146 L202 141" /><path d="M182 154 L203 157" />
            </g>

            <!-- brows -->
            {#if face.brows === "angry"}
              <g class="lines"><path d="M80 103 L107 113" /><path d="M160 103 L133 113" /></g>
            {:else if face.brows === "worried"}
              <g class="lines"><path d="M82 111 L106 102" /><path d="M158 111 L134 102" /></g>
            {:else if face.brows === "raised"}
              <g class="lines"><path d="M82 102 Q94 93 106 99" /><path d="M158 102 Q146 93 134 99" /></g>
            {/if}

            <!-- eyes -->
            {#if face.eyes === "normal" || face.eyes === "teary" || face.eyes === "shiny"}
              <g class="eyes follow">
                {#each EYES as x (x)}
                  <g class="eye">
                    <ellipse cx={x} cy="128" rx="16" ry="19" class="fill-eye" />
                    <circle cx={x - 5} cy="120" r={face.eyes === "normal" ? 6 : 7.5} fill="#fff" />
                    <circle cx={x + 5} cy="134" r={face.eyes === "normal" ? 2.6 : 3.6} fill="#fff" />
                    {#if face.eyes !== "normal"}<circle cx={x + 6} cy="121" r="2" fill="#fff" />{/if}
                  </g>
                {/each}
                {#if face.eyes === "teary"}
                  <path class="tear-line" d="M80 140 Q96 148 112 140" /><path class="tear-line" d="M128 140 Q144 148 160 140" />
                {/if}
              </g>
            {:else if face.eyes === "wide"}
              <g class="eyes follow">
                {#each EYES as x (x)}
                  <ellipse cx={x} cy="127" rx="18" ry="22" class="fill-eye" />
                  <circle cx={x - 6} cy="118" r="5" fill="#fff" />
                  <circle cx={x + 6} cy="135" r="2.2" fill="#fff" />
                {/each}
              </g>
            {:else if face.eyes === "happy"}
              <g class="lines thick"><path d="M82 132 Q96 114 110 132" /><path d="M130 132 Q144 114 158 132" /></g>
            {:else if face.eyes === "closed"}
              <g class="lines thick"><path d="M82 128 Q96 138 110 128" /><path d="M130 128 Q144 138 158 128" /></g>
            {:else if face.eyes === "wink"}
              <g class="eyes follow">
                <ellipse cx="96" cy="128" rx="16" ry="19" class="fill-eye" />
                <circle cx="91" cy="120" r="6" fill="#fff" /><circle cx="101" cy="134" r="2.6" fill="#fff" />
              </g>
              <g class="lines thick"><path d="M130 130 Q144 118 158 130" /></g>
            {:else if face.eyes === "squint"}
              <g class="lines thick"><path d="M84 118 L106 128 L84 138" /><path d="M156 118 L134 128 L156 138" /></g>
            {:else if face.eyes === "annoyed"}
              <g clip-path="url(#lid-{uid})">
                {#each EYES as x (x)}<ellipse cx={x} cy="128" rx="16" ry="19" class="fill-eye" />{/each}
                <circle cx="101" cy="134" r="2.6" fill="#fff" /><circle cx="149" cy="134" r="2.6" fill="#fff" />
              </g>
              <g class="lines"><path d="M79 125 L113 125" /><path d="M127 125 L161 125" /></g>
            {:else if face.eyes === "dots"}
              <circle cx="98" cy="130" r="5.5" class="fill-eye" /><circle cx="142" cy="130" r="5.5" class="fill-eye" />
            {:else if face.eyes === "heart"}
              {#each EYES as x (x)}
                <g transform="translate({x} 124)"><path class="heart-eye" d={HEART} /></g>
              {/each}
            {:else if face.eyes === "star"}
              {#each EYES as x (x)}
                <g transform="translate({x} 128) scale(1.15)"><path class="star-eye" d={STAR} /></g>
              {/each}
            {:else if face.eyes === "spiral"}
              {#each EYES as x (x)}
                <g transform="translate({x} 128)">
                  <path class="spiral" d="M0 0 a2 2 0 0 1 4 0 a4 4 0 0 1 -8 0 a6 6 0 0 1 12 0 a8 8 0 0 1 -16 0 a10 10 0 0 1 20 0" />
                </g>
              {/each}
            {/if}

            <!-- blush -->
            <ellipse class="blush" cx="76" cy="154" rx="11" ry="6.5" />
            <ellipse class="blush" cx="164" cy="154" rx="11" ry="6.5" />
            {#if fx.has("blushlines")}
              <g class="lines blushlines">
                <path d="M68 150 L64 157" /><path d="M75 150 L71 157" /><path d="M82 150 L78 157" />
                <path d="M158 150 L154 157" /><path d="M165 150 L161 157" /><path d="M172 150 L168 157" />
              </g>
            {/if}

            <!-- mouth -->
            {#if face.mouth === "open"}
              <path d="M107 149 Q120 168 133 149 Z" class="fill-eye stroke-eye" stroke-width="2.5" stroke-linejoin="round" />
              <path d="M113 156 Q120 152 127 156 Q120 163 113 156 Z" class="fill-blush" />
            {:else if face.mouth === "scream"}
              <ellipse cx="120" cy="156" rx="9" ry="11" class="fill-eye" />
              <ellipse cx="120" cy="162" rx="5.5" ry="3.5" class="fill-blush" />
            {:else if face.mouth === "o"}
              <ellipse cx="120" cy="154" rx="5" ry="6.5" class="fill-eye" />
            {:else if face.mouth === "small"}
              <path d="M114 151 Q117 155 120 151 Q123 155 126 151" class="mouth" />
            {:else if face.mouth === "wavy"}
              <path d="M106 153 Q111 148 116 153 Q120 158 124 153 Q129 148 134 153" class="mouth" />
            {:else if face.mouth === "frown"}
              <path d="M109 158 Q120 148 131 158" class="mouth" />
            {:else if face.mouth === "flat"}
              <path d="M112 153 L128 153" class="mouth" />
            {:else if face.mouth === "grin"}
              <path d="M104 148 Q120 163 136 148" class="mouth" />
            {:else if face.mouth === "tongue"}
              <path d="M115 152 Q115 164 120 164 Q125 164 125 152 Z" class="fill-blush stroke-eye" stroke-width="2" />
              <path d="M110 150 Q115 157 120 151 Q125 157 130 150" class="mouth" />
            {:else if face.mouth === "smirk"}
              <path d="M109 153 Q121 157 132 146" class="mouth" />
            {:else if face.mouth === "kiss"}
              <path d="M117 144 Q126 147 119 151 Q126 154 117 158" class="mouth" />
            {:else}
              <path d="M110 150 Q115 157 120 151 Q125 157 130 150" class="mouth" />
            {/if}
          </g>

          {#if fx.has("sweat")}
            <path class="sweat" d="M180 100 Q188 112 184 118 Q178 122 175 116 Q174 110 180 100 Z" />
          {/if}
          {#if fx.has("tears")}
            <ellipse class="tear t1" cx="84" cy="146" rx="4" ry="5.5" />
            <ellipse class="tear t2" cx="156" cy="146" rx="4" ry="5.5" />
          {/if}
          {#if fx.has("anger")}
            <g transform="translate(176 92)">
              <path class="anger" d="M-9 -3 Q-3 -3 -3 -9 M3 -9 Q3 -3 9 -3 M9 3 Q3 3 3 9 M-3 9 Q-3 3 -9 3" />
            </g>
          {/if}
          {#if fx.has("droplets")}
            <circle class="drop d1" cx="120" cy="158" r="4" />
            <circle class="drop d2" cx="120" cy="158" r="3" />
            <circle class="drop d3" cx="120" cy="158" r="3.5" />
          {/if}
        </g>
      </g>

      <!-- effects around glorp -->
      {#if fx.has("sparkles")}
        <g class="sparkles">
          <g transform="translate(30 90)"><path class="spark s1" d={SPARK} /></g>
          <g transform="translate(212 110)"><path class="spark s2" d={SPARK} /></g>
          <g transform="translate(196 50) scale(0.7)"><path class="spark s3" d={SPARK} /></g>
          <g transform="translate(44 40) scale(0.7)"><path class="spark s4" d={SPARK} /></g>
        </g>
      {/if}
      {#if fx.has("zzz")}
        <g class="zzz">
          <text x="178" y="70" class="z z1">z</text>
          <text x="192" y="52" class="z z2">z</text>
          <text x="206" y="34" class="z z3">Z</text>
        </g>
      {/if}
      {#if fx.has("hearts")}
        <g class="hearts">
          <g transform="translate(40 76) scale(0.55)"><path class="heart h1" d={HEART} /></g>
          <g transform="translate(202 86) scale(0.45)"><path class="heart h2" d={HEART} /></g>
          <g transform="translate(186 40) scale(0.35)"><path class="heart h3" d={HEART} /></g>
        </g>
      {/if}
      {#if fx.has("notes")}
        <g class="notes">
          <text x="182" y="76" class="note n1">♪</text>
          <text x="40" y="70" class="note n2">♫</text>
          <text x="200" y="44" class="note n3">♪</text>
        </g>
      {/if}
      {#if fx.has("question")}
        <text x="190" y="72" class="mark question">?</text>
      {/if}
      {#if fx.has("exclaim")}
        <text x="186" y="70" class="mark exclaim">!</text>
      {/if}
      {#if fx.has("dizzy")}
        <g transform="translate(120 46) scale(1 0.35)">
          <g class="orbit">
            <g transform="translate(-48 0)"><path class="orbit-star" d={STAR} /></g>
            <g transform="translate(48 0)"><path class="orbit-star" d={STAR} /></g>
            <g transform="translate(0 48)"><path class="orbit-star" d={STAR} /></g>
          </g>
        </g>
      {/if}
    {/key}
  </svg>
</div>

<style>
  .glorp {
    width: 100%;
    height: 100%;
  }
  svg {
    width: 100%;
    height: 100%;
    overflow: visible;
  }
  svg * {
    transform-box: view-box;
  }

  .stroke {
    stroke: var(--glorp-outline);
  }
  .stroke-eye {
    stroke: var(--glorp-eye);
  }
  .fill-mid {
    fill: var(--glorp-mid);
  }
  .fill-ear {
    fill: var(--glorp-ear);
  }
  .fill-eye {
    fill: var(--glorp-eye);
  }
  .fill-blush {
    fill: var(--glorp-blush);
  }
  .mouth {
    fill: none;
    stroke: var(--glorp-eye);
    stroke-width: 3.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .lines path {
    fill: none;
    stroke: var(--glorp-eye);
    stroke-width: 4.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .lines.thick path {
    stroke-width: 6;
  }
  .blushlines path {
    stroke: var(--glorp-blush);
    stroke-width: 2.5;
  }
  .blush {
    fill: var(--glorp-blush);
    opacity: var(--blush);
    transition: opacity 0.3s;
  }
  .tear-line {
    fill: none;
    stroke: #9fdcff;
    stroke-width: 3;
    stroke-linecap: round;
    opacity: 0.9;
  }
  .heart-eye {
    fill: #ff4f8b;
    stroke: var(--glorp-eye);
    stroke-width: 2;
    transform-box: fill-box;
    transform-origin: center;
    animation: heartbeat 0.7s ease-in-out infinite;
  }
  .star-eye {
    fill: var(--glorp-orb);
    stroke: var(--glorp-outline);
    stroke-width: 2.2;
    stroke-linejoin: round;
    transform-box: fill-box;
    transform-origin: center;
    animation: twinkle-eye 1.2s ease-in-out infinite;
  }
  .spiral {
    fill: none;
    stroke: var(--glorp-eye);
    stroke-width: 3.2;
    stroke-linecap: round;
    transform-box: fill-box;
    transform-origin: center;
    animation: spin360 0.9s linear infinite;
  }
  .shadow {
    fill: rgb(var(--shadow) / 0.16);
    transform-origin: 120px 214px;
  }
  .sweat {
    fill: #9fdcff;
    stroke: #5aa9dc;
    stroke-width: 2;
    animation: drip 1.4s ease-in infinite;
  }
  .tear {
    fill: #9fdcff;
    stroke: #5aa9dc;
    stroke-width: 1.5;
    animation: drip 1.1s ease-in infinite;
  }
  .t2 {
    animation-delay: 0.4s;
  }
  .anger {
    fill: none;
    stroke: #ff4f6d;
    stroke-width: 3.5;
    stroke-linecap: round;
    transform-box: fill-box;
    transform-origin: center;
    animation: pulse 0.6s ease-in-out infinite;
  }
  .drop {
    fill: var(--glorp-light);
    stroke: var(--glorp-outline);
    stroke-width: 1.5;
    opacity: 0;
  }
  .d1 {
    animation: splash-1 0.7s ease-out forwards;
  }
  .d2 {
    animation: splash-2 0.7s ease-out 0.05s forwards;
  }
  .d3 {
    animation: splash-3 0.7s ease-out 0.1s forwards;
  }

  /* --- base layers --- */
  .jump,
  .squish {
    transform-origin: 120px 208px;
    transition: transform 0.18s ease-out;
  }
  .face {
    transform: translate(calc(var(--lx) * 0.5), calc(var(--ly) * 0.5));
    transition: transform 0.3s ease-out;
  }
  .follow {
    transform: translate(calc(var(--lx) * 0.6), calc(var(--ly) * 0.6));
    transition: transform 0.2s ease-out;
  }
  .eye {
    transform-origin: 120px 128px;
    animation: blink 5.5s infinite;
  }
  .antenna {
    transform-origin: 120px 70px;
    animation: sway-antenna 3s ease-in-out infinite;
  }
  .orb-glow {
    animation: glow 2.4s ease-in-out infinite;
    transform-origin: 132px 24px;
  }
  .ear-l {
    transform-origin: 70px 100px;
    animation: twitch-l 7s ease-in-out infinite;
  }
  .ear-r {
    transform-origin: 170px 100px;
    animation: twitch-r 9s ease-in-out infinite 1.5s;
  }
  .inner-bubbles circle {
    fill: var(--glorp-light);
    opacity: 0.7;
    animation: rise 5s linear infinite;
  }
  .inner-bubbles circle:nth-child(2) {
    animation-duration: 6.5s;
    animation-delay: -2s;
  }
  .inner-bubbles circle:nth-child(3) {
    animation-duration: 5.8s;
    animation-delay: -3.5s;
  }
  .inner-bubbles circle:nth-child(4) {
    animation-duration: 4.4s;
    animation-delay: -1s;
  }

  /* --- moods (ongoing) --- */
  .mood-idle .squish,
  .mood-sad .squish {
    animation: breathe 3.4s ease-in-out infinite;
  }
  .mood-countdown .jump {
    animation: shiver 0.14s linear infinite;
  }
  .mood-countdown .antenna {
    animation-duration: 0.5s;
  }
  .mood-clicking .jump {
    animation: hop-loop var(--bounce) ease-in-out infinite;
  }
  .mood-clicking .squish {
    animation: squash var(--bounce) ease-in-out infinite;
  }
  .mood-clicking .shadow {
    animation: shadow var(--bounce) ease-in-out infinite;
  }
  .mood-clicking .antenna {
    animation: wiggle calc(var(--bounce) * 2) ease-in-out infinite;
  }
  .mood-clicking .orb-glow {
    animation-duration: var(--bounce);
  }
  .mood-celebrate .jump {
    animation: celebrate 0.55s cubic-bezier(0.3, 0, 0.4, 1) infinite;
  }
  .mood-celebrate .squish {
    animation: squash 0.55s ease-in-out infinite;
  }
  .mood-celebrate .shadow {
    animation: shadow 0.55s ease-in-out infinite;
  }
  .mood-sleepy .squish {
    animation: snooze 4.6s ease-in-out infinite;
  }
  .mood-sleepy .antenna {
    animation: droop 4.6s ease-in-out infinite;
  }
  .mood-sleepy .orb-glow {
    opacity: 0.25;
    animation: none;
  }
  .mood-sad .antenna {
    transform: rotate(-18deg);
    animation: none;
  }

  /* --- one-off reaction animations (override the mood's) --- */
  .anim-poke .squish {
    animation: poke 0.7s cubic-bezier(0.2, 0.9, 0.3, 1.4);
  }
  .anim-jiggle .squish {
    animation: jiggle 0.9s ease-in-out;
  }
  .anim-shiver .jump {
    animation: shiver 0.1s linear 10;
  }
  .anim-spin .jump {
    animation: spin-hop 0.85s cubic-bezier(0.3, 0, 0.3, 1);
  }
  .anim-hop .jump {
    animation: hop-once 0.55s cubic-bezier(0.3, 0, 0.4, 1);
  }
  .anim-hop .squish {
    animation: squash 0.55s ease-in-out;
  }
  .anim-tilt .jump {
    animation: tilt 2.4s ease-in-out forwards;
  }
  .anim-nod .jump {
    animation: nod 0.9s ease-in-out;
  }
  .anim-shake .jump {
    animation: shake 0.7s ease-in-out;
  }
  .anim-melt .squish {
    animation: melt 1.8s ease-in-out;
  }
  .anim-sway .jump {
    animation: sway 1.4s ease-in-out infinite;
  }
  .anim-boing .squish {
    animation: boing 0.9s cubic-bezier(0.2, 0.9, 0.3, 1.3);
  }
  .anim-boing .jump {
    animation: hop-once 0.6s cubic-bezier(0.3, 0, 0.4, 1);
  }
  .anim-stretch .squish {
    animation: stretch 1.6s ease-in-out;
  }
  .anim-sway .antenna,
  .anim-jiggle .antenna {
    animation: wiggle 0.6s ease-in-out infinite;
  }

  /* --- being squeezed (mouse held down) --- */
  .held .jump,
  .held .squish,
  .held .shadow {
    animation: none;
  }
  .held .squish {
    transform: scale(1.16, 0.8);
  }
  .held .antenna {
    animation: wiggle 0.25s ease-in-out infinite;
  }

  /* --- effects --- */
  .spark {
    fill: var(--glorp-orb);
    stroke: var(--glorp-outline);
    stroke-width: 1.2;
    transform-box: fill-box;
    transform-origin: center;
    animation: twinkle 0.9s ease-in-out infinite;
  }
  .s2 { animation-delay: -0.3s; }
  .s3 { animation-delay: -0.6s; }
  .s4 { animation-delay: -0.45s; }

  .z {
    font-family: var(--font-display);
    font-weight: 700;
    fill: var(--text-soft);
    font-size: 16px;
    animation: zfloat 3s ease-in-out infinite;
    opacity: 0;
  }
  .z2 { animation-delay: 1s; font-size: 20px; }
  .z3 { animation-delay: 2s; font-size: 24px; }

  .heart {
    fill: var(--glorp-blush);
    stroke: #fff;
    stroke-width: 2;
    animation: float-up 1.4s ease-out infinite;
    transform-box: fill-box;
    transform-origin: center;
    opacity: 0;
  }
  .h2 { animation-delay: 0.35s; }
  .h3 { animation-delay: 0.7s; }

  .note {
    font-size: 22px;
    font-weight: 700;
    fill: var(--text-soft);
    animation: float-up 1.6s ease-out infinite;
    transform-box: fill-box;
    transform-origin: center;
    opacity: 0;
  }
  .n2 { animation-delay: 0.5s; }
  .n3 { animation-delay: 1s; }

  .mark {
    font-family: var(--font-display);
    font-weight: 700;
    font-size: 34px;
    transform-box: fill-box;
    transform-origin: center bottom;
    animation: pop-in 0.4s cubic-bezier(0.34, 1.8, 0.5, 1);
  }
  .question {
    fill: var(--accent-2);
    animation: pop-in 0.4s cubic-bezier(0.34, 1.8, 0.5, 1), bob 1.2s ease-in-out 0.4s infinite;
  }
  .exclaim {
    fill: var(--stop);
  }

  .orbit {
    animation: spin360 1.1s linear infinite;
    transform-origin: 0 0;
  }
  .orbit-star {
    fill: var(--glorp-orb);
    stroke: var(--glorp-outline);
    stroke-width: 2;
  }

  /* --- keyframes --- */
  @keyframes breathe {
    0%, 100% { transform: scale(1, 1); }
    50% { transform: scale(1.03, 0.97); }
  }
  @keyframes snooze {
    0%, 100% { transform: scale(1.02, 0.95); }
    50% { transform: scale(1.06, 0.91); }
  }
  @keyframes blink {
    0%, 92%, 100% { transform: scaleY(1); }
    94% { transform: scaleY(0.08); }
    96% { transform: scaleY(1); }
  }
  @keyframes sway-antenna {
    0%, 100% { transform: rotate(-5deg); }
    50% { transform: rotate(6deg); }
  }
  @keyframes wiggle {
    0%, 100% { transform: rotate(-14deg); }
    50% { transform: rotate(14deg); }
  }
  @keyframes droop {
    0%, 100% { transform: rotate(-24deg); }
    50% { transform: rotate(-28deg); }
  }
  @keyframes glow {
    0%, 100% { opacity: 0.35; transform: scale(0.85); }
    50% { opacity: 0.9; transform: scale(1.15); }
  }
  @keyframes twitch-l {
    0%, 90%, 100% { transform: rotate(0); }
    93% { transform: rotate(-9deg); }
    96% { transform: rotate(3deg); }
  }
  @keyframes twitch-r {
    0%, 88%, 100% { transform: rotate(0); }
    91% { transform: rotate(9deg); }
    94% { transform: rotate(-3deg); }
  }
  @keyframes rise {
    0% { transform: translateY(0); opacity: 0; }
    15% { opacity: 0.7; }
    100% { transform: translateY(-90px); opacity: 0; }
  }
  @keyframes shiver {
    0%, 100% { transform: translateX(-1.4px); }
    50% { transform: translateX(1.4px); }
  }
  @keyframes hop-loop {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-12px); }
  }
  @keyframes hop-once {
    0%, 100% { transform: translateY(0); }
    45% { transform: translateY(-26px); }
  }
  @keyframes squash {
    0%, 100% { transform: scale(1.08, 0.9); }
    35% { transform: scale(0.97, 1.04); }
    50% { transform: scale(0.96, 1.05); }
    80% { transform: scale(1.01, 0.99); }
  }
  @keyframes shadow {
    0%, 100% { transform: scale(1.05); opacity: 1; }
    50% { transform: scale(0.8); opacity: 0.6; }
  }
  @keyframes celebrate {
    0%, 100% { transform: translateY(0) rotate(0); }
    50% { transform: translateY(-24px) rotate(-4deg); }
  }
  @keyframes poke {
    0% { transform: scale(1, 1); }
    25% { transform: scale(1.16, 0.82); }
    60% { transform: scale(0.95, 1.06); }
    100% { transform: scale(1, 1); }
  }
  @keyframes jiggle {
    0%, 100% { transform: scale(1, 1); }
    15% { transform: scale(1.1, 0.9); }
    30% { transform: scale(0.92, 1.08); }
    45% { transform: scale(1.06, 0.95); }
    60% { transform: scale(0.97, 1.03); }
    75% { transform: scale(1.02, 0.98); }
  }
  @keyframes spin-hop {
    0% { transform: translateY(0) rotate(0); }
    40% { transform: translateY(-30px) rotate(180deg); }
    80% { transform: translateY(-4px) rotate(360deg); }
    100% { transform: translateY(0) rotate(360deg); }
  }
  @keyframes tilt {
    0% { transform: rotate(0); }
    15%, 85% { transform: rotate(-9deg); }
    100% { transform: rotate(0); }
  }
  @keyframes nod {
    0%, 100% { transform: translateY(0); }
    25%, 75% { transform: translateY(5px); }
    50% { transform: translateY(0); }
  }
  @keyframes shake {
    0%, 100% { transform: rotate(0); }
    15%, 55% { transform: rotate(-8deg); }
    35%, 75% { transform: rotate(8deg); }
  }
  @keyframes melt {
    0%, 100% { transform: scale(1, 1); }
    35%, 65% { transform: scale(1.22, 0.7); }
  }
  @keyframes sway {
    0%, 100% { transform: rotate(-5deg); }
    50% { transform: rotate(5deg); }
  }
  @keyframes boing {
    0% { transform: scale(1.16, 0.8); }
    30% { transform: scale(0.88, 1.16); }
    55% { transform: scale(1.06, 0.94); }
    80% { transform: scale(0.98, 1.02); }
    100% { transform: scale(1, 1); }
  }
  @keyframes stretch {
    0%, 100% { transform: scale(1, 1); }
    40%, 70% { transform: scale(0.9, 1.14); }
  }
  @keyframes twinkle {
    0%, 100% { transform: scale(0.4) rotate(0deg); opacity: 0.3; }
    50% { transform: scale(1.1) rotate(45deg); opacity: 1; }
  }
  @keyframes twinkle-eye {
    0%, 100% { transform: scale(1) rotate(0deg); }
    50% { transform: scale(1.12) rotate(12deg); }
  }
  @keyframes heartbeat {
    0%, 100% { transform: scale(1); }
    40% { transform: scale(1.18); }
  }
  @keyframes spin360 {
    to { transform: rotate(360deg); }
  }
  @keyframes pulse {
    0%, 100% { transform: scale(0.85); }
    50% { transform: scale(1.15); }
  }
  @keyframes zfloat {
    0% { transform: translate(0, 8px); opacity: 0; }
    30% { opacity: 1; }
    100% { transform: translate(10px, -14px); opacity: 0; }
  }
  @keyframes float-up {
    0% { transform: translateY(0) scale(0.4); opacity: 0; }
    30% { opacity: 1; transform: translateY(-8px) scale(1.1); }
    100% { transform: translateY(-36px) scale(1); opacity: 0; }
  }
  @keyframes drip {
    0% { transform: translateY(0); opacity: 1; }
    100% { transform: translateY(14px); opacity: 0; }
  }
  @keyframes pop-in {
    0% { transform: scale(0); }
    100% { transform: scale(1); }
  }
  @keyframes bob {
    0%, 100% { transform: translateY(0) rotate(-6deg); }
    50% { transform: translateY(-5px) rotate(6deg); }
  }
  @keyframes splash-1 {
    0% { transform: translate(0, 0); opacity: 1; }
    100% { transform: translate(-46px, 10px); opacity: 0; }
  }
  @keyframes splash-2 {
    0% { transform: translate(0, 0); opacity: 1; }
    100% { transform: translate(4px, 30px); opacity: 0; }
  }
  @keyframes splash-3 {
    0% { transform: translate(0, 0); opacity: 1; }
    100% { transform: translate(48px, 14px); opacity: 0; }
  }

  @media (prefers-reduced-motion: reduce) {
    .glorp :global(*) {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
    }
  }
</style>
