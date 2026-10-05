// Theme registry. A theme is a set of CSS variables plus a mascot component.
// To add a new theme: copy one below, change the colors, and (optionally) point
// `mascot` at a different character component in its own folder.
import type { Component } from "svelte";
import Glorp from "./glorp/Glorp.svelte";
import { glorpLines, type Lines } from "./glorp/lines";
import type { Expression } from "$lib/expression";
import type { Mood } from "$lib/types";

export interface MascotProps {
  mood: Mood;
  /** Current clicks per second, used to speed up animations. */
  cps: number;
  /** Where the pointer is relative to the mascot, each axis in -1..1. */
  look: { x: number; y: number };
  /** A reaction's face, overriding the mood's default face. */
  expression?: Expression | null;
  /** True while the mascot is being squeezed (mouse held down on it). */
  held?: boolean;
}

export interface Theme {
  id: string;
  name: string;
  blurb: string;
  dark: boolean;
  mascot: Component<MascotProps>;
  /** What the mascot says (and the faces it makes). */
  lines: Lines;
  /** CSS custom properties (without the leading `--`). */
  vars: Record<string, string>;
}

const glorpGreen = {
  "glorp-light": "#d4ff9c",
  "glorp-mid": "#8fe866",
  "glorp-dark": "#45b84f",
  "glorp-outline": "#2a7f3c",
  "glorp-ear": "#c4f59a",
  "glorp-eye": "#1d2433",
  "glorp-blush": "#ff8fb1",
  "glorp-orb": "#e9ff7a",
};

export const themes: Theme[] = [
  {
    id: "glorp",
    name: "Slime Meadow",
    blurb: "fresh, minty, very glorp",
    dark: false,
    mascot: Glorp,
    lines: glorpLines,
    vars: {
      ...glorpGreen,
      "bg-1": "#f3fbe6",
      "bg-2": "#e6f6ff",
      "hero-1": "#e3f8cf",
      "hero-2": "#d8f0ff",
      surface: "#ffffff",
      "surface-2": "#f2f8ec",
      "surface-3": "#e6f2dc",
      text: "#24402c",
      "text-soft": "#5b7563",
      muted: "#93a898",
      primary: "#6fd65a",
      "primary-2": "#3fae4a",
      "primary-text": "#ffffff",
      accent: "#ff8fb1",
      "accent-2": "#b9a4ff",
      stop: "#ff6f91",
      "stop-2": "#f0487a",
      border: "#dcebd2",
      shadow: "36 90 44",
      bubble: "#ffffff",
      particle: "#9be58a",
    },
  },
  {
    id: "glorp-night",
    name: "Space Goo",
    blurb: "glorp's home planet, after dark",
    dark: true,
    mascot: Glorp,
    lines: glorpLines,
    vars: {
      ...glorpGreen,
      "glorp-light": "#e2ffae",
      "glorp-mid": "#96f06a",
      "glorp-dark": "#3dae57",
      "glorp-outline": "#1d5a36",
      "glorp-orb": "#fff27a",
      "bg-1": "#1a1535",
      "bg-2": "#241c4a",
      "hero-1": "#2b2160",
      "hero-2": "#1d2a55",
      surface: "#2a2352",
      "surface-2": "#332b61",
      "surface-3": "#3d3470",
      text: "#f1efff",
      "text-soft": "#bdb6e6",
      muted: "#8b83b8",
      primary: "#86e45e",
      "primary-2": "#48b851",
      "primary-text": "#12240f",
      accent: "#ff7eb6",
      "accent-2": "#a996ff",
      stop: "#ff6f9f",
      "stop-2": "#e84584",
      border: "#3e3677",
      shadow: "5 3 20",
      bubble: "#3a3170",
      particle: "#fff27a",
    },
  },
  {
    id: "bubblegum",
    name: "Bubblegum",
    blurb: "glorp's pink cousin, glorpina",
    dark: false,
    mascot: Glorp,
    lines: glorpLines,
    vars: {
      "glorp-light": "#ffe0f0",
      "glorp-mid": "#ffa6d2",
      "glorp-dark": "#ec6fae",
      "glorp-outline": "#b3437d",
      "glorp-ear": "#ffd0e7",
      "glorp-eye": "#3a1d33",
      "glorp-blush": "#ff5f9a",
      "glorp-orb": "#b9f3ff",
      "bg-1": "#fff4f9",
      "bg-2": "#f3efff",
      "hero-1": "#ffe3f1",
      "hero-2": "#ebe4ff",
      surface: "#ffffff",
      "surface-2": "#fdf1f7",
      "surface-3": "#f8e2ee",
      text: "#4a2340",
      "text-soft": "#86607b",
      muted: "#b99bb0",
      primary: "#ff86bf",
      "primary-2": "#e8559c",
      "primary-text": "#ffffff",
      accent: "#8fd8ff",
      "accent-2": "#c3a8ff",
      stop: "#9b7bff",
      "stop-2": "#7a57f0",
      border: "#f5dbe8",
      shadow: "120 40 90",
      bubble: "#ffffff",
      particle: "#ffb3d7",
    },
  },
];

export function getTheme(id: string): Theme {
  return themes.find((t) => t.id === id) ?? themes[0];
}

export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  for (const [k, v] of Object.entries(theme.vars)) root.style.setProperty(`--${k}`, v);
  root.dataset.theme = theme.id;
  root.style.colorScheme = theme.dark ? "dark" : "light";
}
