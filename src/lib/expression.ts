// The "face kit" every mascot understands. A reaction picks one of each and the mascot
// draws it, so new lines can get fitting faces without touching any drawing code.
import type { Mood } from "./types";

export type Eyes =
  | "normal" // big round eyes that follow the cursor
  | "wide" // surprised
  | "shiny" // sparkly pleading eyes
  | "happy" // ^ ^
  | "closed" // ‿ ‿ (content / asleep)
  | "wink"
  | "heart"
  | "star"
  | "spiral" // dizzy
  | "squint" // > <
  | "annoyed" // half-closed
  | "teary"
  | "dots"; // deadpan · ·

export type Mouth =
  | "cat" // ω
  | "small"
  | "open" // big happy D
  | "o"
  | "scream"
  | "wavy"
  | "frown"
  | "flat"
  | "grin"
  | "tongue"
  | "smirk"
  | "kiss";

export type Brows = "angry" | "worried" | "raised";

export type Fx =
  | "hearts"
  | "sweat"
  | "sparkles"
  | "notes"
  | "zzz"
  | "question"
  | "exclaim"
  | "anger"
  | "tears"
  | "dizzy"
  | "droplets"
  | "blushlines";

export type Anim =
  | "poke"
  | "jiggle"
  | "shiver"
  | "spin"
  | "hop"
  | "tilt"
  | "nod"
  | "shake"
  | "melt"
  | "sway"
  | "boing"
  | "stretch";

export interface Expression {
  eyes: Eyes;
  mouth: Mouth;
  brows?: Brows;
  fx?: Fx[];
  /** One-off body animation played when the expression appears. */
  anim?: Anim;
  /** Cheek blush strength, 0..1 (default 0.8). */
  blush?: number;
  /** Changes whenever a new reaction starts, so the same animation can replay. */
  id?: number;
}

/** What a mascot looks like in each mood when no reaction is playing. */
export const MOOD_FACES: Record<Mood, Expression> = {
  idle: { eyes: "normal", mouth: "cat" },
  countdown: { eyes: "wide", mouth: "o" },
  clicking: { eyes: "normal", mouth: "open", fx: ["sparkles"] },
  celebrate: { eyes: "happy", mouth: "open", fx: ["sparkles"] },
  sleepy: { eyes: "closed", mouth: "small", fx: ["zzz"] },
  poked: { eyes: "squint", mouth: "wavy", fx: ["hearts"], anim: "poke" },
  sad: { eyes: "normal", mouth: "frown", brows: "worried", fx: ["sweat"] },
};

/** Context lines can use, e.g. to mention the hotkey or click counts. */
export interface LineContext {
  hotkey: string;
  cps: number;
  clicks: number;
  total: number;
  pokes: number;
  milestone: number;
}

export interface Reaction {
  text: string | ((c: LineContext) => string);
  face: Expression;
  /** How long the line stays up (ms). Defaults to a length-based duration. */
  ms?: number;
}

/** Shorthand used by the line files: F("happy", "open", { fx: ["hearts"] }). */
export function F(eyes: Eyes, mouth: Mouth, extra: Partial<Expression> = {}): Expression {
  return { eyes, mouth, ...extra };
}
