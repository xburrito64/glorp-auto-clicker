<!--
  Glorp: a green slime alien cat. All colors come from --glorp-* CSS variables,
  so themes can recolor him. Animations are driven by the `mood` class.
-->
<script lang="ts">
  import type { MascotProps } from "../index";

  let { mood, cps, look }: MascotProps = $props();

  // One bounce per click at low speeds, capped so fast clicking stays readable.
  const bounce = $derived(Math.min(0.6, Math.max(0.18, 1 / Math.max(cps, 0.1))));
  const lx = $derived(Math.max(-1, Math.min(1, look.x)) * 6);
  const ly = $derived(Math.max(-1, Math.min(1, look.y)) * 5);

  const uid = Math.random().toString(36).slice(2, 8);
</script>

<div class="glorp mood-{mood}" style="--bounce:{bounce}s; --lx:{lx}px; --ly:{ly}px">
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
      <clipPath id="clip-{uid}">
        <path d="M44 176 C36 130 58 68 120 66 C182 68 204 130 196 176 C194 192 182 198 170 194 C168 208 150 212 148 196 C136 200 104 200 92 197 C90 214 70 214 70 196 C58 198 46 192 44 176 Z" />
      </clipPath>
    </defs>

    <ellipse class="shadow" cx="120" cy="214" rx="66" ry="9" />

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
        <path
          d="M44 176 C36 130 58 68 120 66 C182 68 204 130 196 176 C194 192 182 198 170 194 C168 208 150 212 148 196 C136 200 104 200 92 197 C90 214 70 214 70 196 C58 198 46 192 44 176 Z"
          fill="url(#body-{uid})"
          class="stroke"
          stroke-width="4"
          stroke-linejoin="round"
        />

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

          <!-- eyes -->
          {#if mood === "celebrate"}
            <g class="eye-lines">
              <path d="M82 132 Q96 114 110 132" /><path d="M130 132 Q144 114 158 132" />
            </g>
          {:else if mood === "sleepy"}
            <g class="eye-lines">
              <path d="M82 128 Q96 138 110 128" /><path d="M130 128 Q144 138 158 128" />
            </g>
          {:else if mood === "poked"}
            <g class="eye-lines">
              <path d="M84 118 L106 128 L84 138" /><path d="M156 118 L134 128 L156 138" />
            </g>
          {:else}
            <g class="eyes">
              <g class="eye">
                <ellipse cx="96" cy="128" rx="16" ry="19" class="fill-eye" />
                <circle cx="91" cy="120" r="6" fill="#fff" />
                <circle cx="101" cy="134" r="2.6" fill="#fff" />
              </g>
              <g class="eye">
                <ellipse cx="144" cy="128" rx="16" ry="19" class="fill-eye" />
                <circle cx="139" cy="120" r="6" fill="#fff" />
                <circle cx="149" cy="134" r="2.6" fill="#fff" />
              </g>
            </g>
          {/if}

          <!-- blush -->
          <ellipse class="blush" cx="76" cy="154" rx="11" ry="6.5" />
          <ellipse class="blush" cx="164" cy="154" rx="11" ry="6.5" />

          <!-- mouth -->
          {#if mood === "clicking" || mood === "celebrate"}
            <path d="M107 149 Q120 168 133 149 Z" class="fill-eye stroke-eye" stroke-width="2.5" stroke-linejoin="round" />
            <path d="M113 156 Q120 152 127 156 Q120 163 113 156 Z" class="fill-blush" />
          {:else if mood === "countdown"}
            <ellipse cx="120" cy="154" rx="5" ry="6.5" class="fill-eye" />
          {:else if mood === "sad"}
            <path d="M109 158 Q120 148 131 158" class="stroke-eye mouth" />
          {:else if mood === "poked"}
            <path d="M106 153 Q111 148 116 153 Q120 158 124 153 Q129 148 134 153" class="stroke-eye mouth" />
          {:else}
            <path d="M110 150 Q115 157 120 151 Q125 157 130 150" class="stroke-eye mouth" />
          {/if}
        </g>

        {#if mood === "sad"}
          <path class="sweat" d="M180 100 Q188 112 184 118 Q178 122 175 116 Q174 110 180 100 Z" />
        {/if}
      </g>
    </g>

    <!-- effects around glorp -->
    {#if mood === "clicking" || mood === "celebrate"}
      <g class="sparkles">
        <g transform="translate(30 90)"><path class="spark s1" d="M0 -9 Q1.5 -1.5 9 0 Q1.5 1.5 0 9 Q-1.5 1.5 -9 0 Q-1.5 -1.5 0 -9 Z" /></g>
        <g transform="translate(212 110)"><path class="spark s2" d="M0 -7 Q1.2 -1.2 7 0 Q1.2 1.2 0 7 Q-1.2 1.2 -7 0 Q-1.2 -1.2 0 -7 Z" /></g>
        <g transform="translate(196 50)"><path class="spark s3" d="M0 -6 Q1 -1 6 0 Q1 1 0 6 Q-1 1 -6 0 Q-1 -1 0 -6 Z" /></g>
        <g transform="translate(44 40)"><path class="spark s4" d="M0 -6 Q1 -1 6 0 Q1 1 0 6 Q-1 1 -6 0 Q-1 -1 0 -6 Z" /></g>
      </g>
    {/if}
    {#if mood === "sleepy"}
      <g class="zzz">
        <text x="178" y="70" class="z z1">z</text>
        <text x="192" y="52" class="z z2">z</text>
        <text x="206" y="34" class="z z3">Z</text>
      </g>
    {/if}
    {#if mood === "poked"}
      <g class="hearts">
        <g transform="translate(40 70)"><path class="heart h1" d="M0 3 C-6 -3 -10 2 -6 6 L0 11 L6 6 C10 2 6 -3 0 3 Z" /></g>
        <g transform="translate(200 80) scale(0.8)"><path class="heart h2" d="M0 3 C-6 -3 -10 2 -6 6 L0 11 L6 6 C10 2 6 -3 0 3 Z" /></g>
      </g>
    {/if}
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
    stroke-width: 3.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .blush {
    fill: var(--glorp-blush);
    opacity: 0.8;
  }
  .eye-lines path {
    fill: none;
    stroke: var(--glorp-eye);
    stroke-width: 6;
    stroke-linecap: round;
    stroke-linejoin: round;
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

  /* --- base layers --- */
  .jump,
  .squish {
    transform-origin: 120px 208px;
  }
  .face {
    transform: translate(calc(var(--lx) * 0.5), calc(var(--ly) * 0.5));
    transition: transform 0.3s ease-out;
  }
  .eyes {
    transform: translate(calc(var(--lx) * 0.6), calc(var(--ly) * 0.6));
    transition: transform 0.2s ease-out;
  }
  .eye {
    transform-origin: 120px 128px;
    animation: blink 5.5s infinite;
  }
  .antenna {
    transform-origin: 120px 70px;
    animation: sway 3s ease-in-out infinite;
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

  /* --- moods --- */
  .mood-idle .squish {
    animation: breathe 3.4s ease-in-out infinite;
  }

  .mood-countdown .jump {
    animation: shiver 0.14s linear infinite;
  }
  .mood-countdown .eyes {
    transform: translate(calc(var(--lx) * 0.6), calc(var(--ly) * 0.6)) scale(1.08);
    transform-origin: 120px 128px;
  }
  .mood-countdown .antenna {
    animation-duration: 0.5s;
  }

  .mood-clicking .jump {
    animation: hop var(--bounce) ease-in-out infinite;
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

  .mood-poked .squish {
    animation: poke 0.7s cubic-bezier(0.2, 0.9, 0.3, 1.4);
  }
  .mood-sad .squish {
    animation: breathe 5s ease-in-out infinite;
  }
  .mood-sad .antenna {
    transform: rotate(-18deg);
    animation: none;
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
    animation: float-up 1.2s ease-out forwards;
    transform-box: fill-box;
    transform-origin: center;
  }
  .h2 { animation-delay: 0.15s; }

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
  @keyframes sway {
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
    0%, 100% { transform: translateX(-1.2px); }
    50% { transform: translateX(1.2px); }
  }
  @keyframes hop {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-12px); }
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
  @keyframes twinkle {
    0%, 100% { transform: scale(0.4) rotate(0deg); opacity: 0.3; }
    50% { transform: scale(1.1) rotate(45deg); opacity: 1; }
  }
  @keyframes zfloat {
    0% { transform: translate(0, 8px); opacity: 0; }
    30% { opacity: 1; }
    100% { transform: translate(10px, -14px); opacity: 0; }
  }
  @keyframes float-up {
    0% { transform: translateY(0) scale(0.4); opacity: 0; }
    30% { opacity: 1; transform: translateY(-6px) scale(1.2); }
    100% { transform: translateY(-30px) scale(1); opacity: 0; }
  }
  @keyframes drip {
    0% { transform: translateY(0); opacity: 1; }
    100% { transform: translateY(14px); opacity: 0; }
  }

  @media (prefers-reduced-motion: reduce) {
    .glorp :global(*) {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
    }
  }
</style>
