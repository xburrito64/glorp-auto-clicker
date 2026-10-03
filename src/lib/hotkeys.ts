// Turning keyboard events into accelerator strings the Rust side understands
// (e.g. "Ctrl+Shift+KeyK"), and back into something pretty for display.

const MODIFIER_CODES = /^(Control|Shift|Alt|Meta|OS)(Left|Right)?$/;

// Keys that are fine as a hotkey on their own because nobody types with them.
const SOLO_OK = /^(F([1-9]|1\d|2[0-4])|Pause|ScrollLock|Insert|Home|End|PageUp|PageDown|NumLock|PrintScreen)$/;

const SUPPORTED =
  /^(Key[A-Z]|Digit\d|F([1-9]|1\d|2[0-4])|Numpad(\d|Add|Decimal|Divide|Enter|Equal|Multiply|Subtract)|Arrow(Up|Down|Left|Right)|Space|Enter|Tab|Backspace|Delete|End|Home|Insert|PageUp|PageDown|PrintScreen|ScrollLock|Pause|Backquote|Backslash|BracketLeft|BracketRight|Comma|Equal|Minus|Period|Quote|Semicolon|Slash|CapsLock|NumLock)$/;

export type HotkeyResult = { ok: true; accel: string } | { ok: false; reason: string } | null;

/** Returns null while only modifiers are held. */
export function hotkeyFromEvent(e: KeyboardEvent): HotkeyResult {
  if (MODIFIER_CODES.test(e.code)) return null;
  if (!SUPPORTED.test(e.code)) return { ok: false, reason: "Glorp can't use that key, try another" };
  const mods: string[] = [];
  if (e.ctrlKey) mods.push("Ctrl");
  if (e.altKey) mods.push("Alt");
  if (e.shiftKey) mods.push("Shift");
  if (e.metaKey) mods.push("Super");
  if (mods.length === 0 && !SOLO_OK.test(e.code)) {
    return { ok: false, reason: "Add Ctrl, Alt or Shift, or use an F-key" };
  }
  return { ok: true, accel: [...mods, e.code].join("+") };
}

const NAMES: Record<string, string> = {
  Super: "Win",
  Backquote: "`",
  Backslash: "\\",
  BracketLeft: "[",
  BracketRight: "]",
  Comma: ",",
  Equal: "=",
  Minus: "-",
  Period: ".",
  Quote: "'",
  Semicolon: ";",
  Slash: "/",
  ArrowUp: "↑",
  ArrowDown: "↓",
  ArrowLeft: "←",
  ArrowRight: "→",
  PageUp: "PgUp",
  PageDown: "PgDn",
  ScrollLock: "ScrLk",
  PrintScreen: "PrtSc",
  Backspace: "⌫",
};

export function hotkeyParts(accel: string): string[] {
  return accel.split("+").map((p) => {
    if (NAMES[p]) return NAMES[p];
    if (p.startsWith("Key")) return p.slice(3);
    if (p.startsWith("Digit")) return p.slice(5);
    if (p.startsWith("Numpad")) return "Num " + p.slice(6);
    return p;
  });
}

export const prettyHotkey = (accel: string) => hotkeyParts(accel).join(" + ");
