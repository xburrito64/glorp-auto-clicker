// Decides how the mascot reacts to the user: picks fresh lines and tracks poke combos.
import type { Reaction } from "./expression";
import type { Lines } from "./themes/glorp/lines";

/** Pokes closer together than this count as one combo. */
const COMBO_GAP_MS = 750;
const GRUMPY_FOR_MS = 7000;

export class Brain {
  private lastPicked = new Map<string, Reaction>();
  private streak = 0;
  private lastPokeAt = 0;
  /** While grumpy, pokes get the cold shoulder. */
  grumpyUntil = 0;
  private needsForgiving = false;

  constructor(public lines: Lines) {}

  /** Random line from a category, never the same one twice in a row. */
  pick(category: keyof Lines | string, list: Reaction[]): Reaction {
    const last = this.lastPicked.get(category);
    const pool = list.length > 1 ? list.filter((r) => r !== last) : list;
    const choice = pool[Math.floor(Math.random() * pool.length)];
    this.lastPicked.set(category, choice);
    return choice;
  }

  poke(now: number, sleepy: boolean): Reaction {
    const l = this.lines;
    if (now < this.grumpyUntil) {
      this.grumpyUntil = Math.max(this.grumpyUntil, now + 2500);
      return this.pick("whileGrumpy", l.whileGrumpy);
    }
    this.streak = now - this.lastPokeAt < COMBO_GAP_MS ? this.streak + 1 : 1;
    this.lastPokeAt = now;

    if (sleepy && this.streak === 1) return this.pick("wakeUp", l.wakeUp);
    if (this.streak >= 10) {
      this.streak = 0;
      this.grumpyUntil = now + GRUMPY_FOR_MS;
      this.needsForgiving = true;
      return this.pick("pokeGrumpy", l.pokeGrumpy);
    }
    if (this.streak >= 7) return this.pick("pokeDizzy", l.pokeDizzy);
    if (this.streak >= 4) return this.pick("pokeAnnoyed", l.pokeAnnoyed);
    return this.pick("poke", l.poke);
  }

  isGrumpy(now: number) {
    return now < this.grumpyUntil;
  }

  /** Returns a "forgive" line once, after a grumpy spell wears off. */
  maybeForgive(now: number): Reaction | null {
    if (!this.needsForgiving || now < this.grumpyUntil) return null;
    this.needsForgiving = false;
    return this.pick("forgive", this.lines.forgive);
  }
}

/** How long a line stays on screen if it doesn't say otherwise. */
export function lineDuration(r: Reaction, text: string): number {
  return r.ms ?? Math.min(4000, Math.max(1800, 1300 + text.length * 45));
}
