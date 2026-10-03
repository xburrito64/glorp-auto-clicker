// Renders Glorp into the app icon source + tray icons.
// Run via `npm run icons` (which also runs `tauri icon` to make every app icon size).
import { Resvg } from "@resvg/resvg-js";
import { writeFileSync, mkdirSync } from "node:fs";

const C = {
  light: "#d4ff9c",
  mid: "#8fe866",
  dark: "#45b84f",
  outline: "#2a7f3c",
  eye: "#1d2433",
  blush: "#ff8fb1",
  earInner: "#c4f59a",
  orb: "#e9ff7a",
  orbActive: "#ffd84d",
};

function glorp({ happy = false, badge = false, bold = false } = {}) {
  const stroke = bold ? 7 : 4;
  const eyes = happy
    ? `<path d="M82 132 Q96 114 110 132" fill="none" stroke="${C.eye}" stroke-width="${stroke + 2}" stroke-linecap="round"/>
       <path d="M130 132 Q144 114 158 132" fill="none" stroke="${C.eye}" stroke-width="${stroke + 2}" stroke-linecap="round"/>`
    : `<ellipse cx="96" cy="128" rx="16" ry="19" fill="${C.eye}"/>
       <ellipse cx="144" cy="128" rx="16" ry="19" fill="${C.eye}"/>
       <circle cx="91" cy="120" r="6" fill="#fff"/><circle cx="139" cy="120" r="6" fill="#fff"/>
       <circle cx="101" cy="134" r="2.6" fill="#fff"/><circle cx="149" cy="134" r="2.6" fill="#fff"/>`;
  const orb = happy ? C.orbActive : C.orb;
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="16 10 208 208">
  <defs>
    <radialGradient id="body" cx="40%" cy="32%" r="75%">
      <stop offset="0" stop-color="${C.light}"/>
      <stop offset="0.55" stop-color="${C.mid}"/>
      <stop offset="1" stop-color="${C.dark}"/>
    </radialGradient>
    <radialGradient id="orb" cx="40%" cy="35%" r="70%">
      <stop offset="0" stop-color="#ffffff"/>
      <stop offset="0.45" stop-color="${orb}"/>
      <stop offset="1" stop-color="${C.dark}"/>
    </radialGradient>
  </defs>
  <path d="M120 70 C117 52 124 40 131 28" fill="none" stroke="${C.outline}" stroke-width="${stroke + 1}" stroke-linecap="round"/>
  <circle cx="132" cy="24" r="${bold ? 13 : 10}" fill="url(#orb)" stroke="${C.outline}" stroke-width="${stroke - 1}"/>
  <path d="M58 106 C52 82 50 60 56 42 C58 35 65 34 69 38 C81 51 93 63 101 78 Z" fill="${C.mid}" stroke="${C.outline}" stroke-width="${stroke}" stroke-linejoin="round"/>
  <path d="M182 106 C188 82 190 60 184 42 C182 35 175 34 171 38 C159 51 147 63 139 78 Z" fill="${C.mid}" stroke="${C.outline}" stroke-width="${stroke}" stroke-linejoin="round"/>
  <path d="M65 94 C62 78 62 62 65 52 C75 62 83 70 89 80 Z" fill="${C.earInner}"/>
  <path d="M175 94 C178 78 178 62 175 52 C165 62 157 70 151 80 Z" fill="${C.earInner}"/>
  <path d="M44 176 C36 130 58 68 120 66 C182 68 204 130 196 176 C194 192 182 198 170 194 C168 208 150 212 148 196 C136 200 104 200 92 197 C90 214 70 214 70 196 C58 198 46 192 44 176 Z"
        fill="url(#body)" stroke="${C.outline}" stroke-width="${stroke}" stroke-linejoin="round"/>
  <ellipse cx="82" cy="98" rx="17" ry="9" fill="#fff" opacity="0.6" transform="rotate(-32 82 98)"/>
  <circle cx="104" cy="86" r="4.5" fill="#fff" opacity="0.6"/>
  ${eyes}
  <ellipse cx="76" cy="154" rx="11" ry="6.5" fill="${C.blush}" opacity="0.6"/>
  <ellipse cx="164" cy="154" rx="11" ry="6.5" fill="${C.blush}" opacity="0.6"/>
  <path d="M110 150 Q115 157 120 151 Q125 157 130 150" fill="none" stroke="${C.eye}" stroke-width="${stroke - 0.5}" stroke-linecap="round" stroke-linejoin="round"/>
  ${badge ? `<circle cx="186" cy="186" r="26" fill="#ff5c9a" stroke="#fff" stroke-width="7"/>` : ""}
</svg>`;
}

function render(svg, size, file) {
  const png = new Resvg(svg, { fitTo: { mode: "width", value: size } }).render().asPng();
  writeFileSync(file, png);
  console.log("wrote", file);
}

mkdirSync("src-tauri/icons", { recursive: true });
mkdirSync("static", { recursive: true });

render(glorp(), 1024, "src-tauri/icons/app-icon-source.png");
render(glorp({ bold: true }), 32, "src-tauri/icons/tray-idle.png");
render(glorp({ bold: true, happy: true, badge: true }), 32, "src-tauri/icons/tray-active.png");
render(glorp(), 64, "static/favicon.png");
