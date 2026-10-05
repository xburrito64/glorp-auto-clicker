// Everything Glorp says, and the face he makes while saying it.
// Add lines freely: each entry is a text (or a function of the context) plus a face.
import { F, type LineContext, type Reaction } from "$lib/expression";

const L = (text: Reaction["text"], face: Reaction["face"], ms?: number): Reaction => ({ text, face, ms });
const n = (x: number) => Math.round(x).toLocaleString();

export interface Lines {
  poke: Reaction[];
  pokeAnnoyed: Reaction[];
  pokeDizzy: Reaction[];
  pokeGrumpy: Reaction[];
  whileGrumpy: Reaction[];
  forgive: Reaction[];
  wakeUp: Reaction[];
  pet: Reaction[];
  holdLong: Reaction[];
  holdRelease: Reaction[];
  idle: Reaction[];
  clicking: Reaction[];
  turbo: Reaction[];
  slow: Reaction[];
  done: Reaction[];
  timeUp: Reaction[];
  stopped: Reaction[];
  milestone: Reaction[];
  welcomeBack: Reaction[];
  themeChange: Reaction[];
  greeting: (hour: number) => Reaction[];
}

export const glorpLines: Lines = {
  poke: [
    L("blorp!", F("wide", "o", { anim: "poke", fx: ["exclaim"] })),
    L("hehe, that tickles", F("happy", "open", { anim: "poke", blush: 1 })),
    L("i'm 97% slime", F("closed", "smirk", { anim: "nod", fx: ["sparkles"] })),
    L("*jiggles*", F("normal", "wavy", { anim: "jiggle" })),
    L("boop received", F("wink", "tongue", { anim: "poke" })),
    L("meow? i mean… glorp.", F("wide", "wavy", { anim: "tilt", fx: ["sweat"], blush: 1 })),
    L("greetings from planet glorpulon", F("star", "grin", { anim: "hop", fx: ["sparkles"] })),
    L("please do not eat me", F("teary", "wavy", { anim: "shiver", brows: "worried", fx: ["sweat"] })),
    L("i love you too!", F("heart", "open", { anim: "hop", fx: ["hearts"] })),
    L("squish!", F("squint", "wavy", { anim: "poke" })),
    L("hey, that's my face", F("dots", "flat", { anim: "tilt" })),
    L("*happy wiggle*", F("happy", "cat", { anim: "jiggle", fx: ["hearts"] })),
    L("you found my boop button", F("wink", "grin", { anim: "nod" })),
    L("i am a very serious alien", F("dots", "flat", { brows: "raised" })),
    L("uwu", F("shiny", "cat", { fx: ["blushlines"], blush: 1, anim: "poke" })),
    L("ow! jk, i'm squishy", F("squint", "tongue", { anim: "poke" })),
    L("the slime is strong with this one", F("star", "smirk", { fx: ["sparkles"] })),
    L("do it again!", F("shiny", "open", { anim: "hop" })),
    L("*suspicious glorp noises*", F("annoyed", "smirk", { anim: "tilt" })),
    L("i'm telling my mom", F("teary", "frown", { anim: "shake", brows: "worried" })),
    L("beep boop, i am glorp", F("dots", "o", { anim: "nod" })),
    L("*purrlorp*", F("closed", "cat", { fx: ["hearts"], blush: 1, anim: "poke" })),
    L((c) => `poke #${n(c.pokes)}! i'm counting`, F("normal", "smirk", { anim: "poke", brows: "raised" })),
    L("my ears are sensitive!", F("squint", "o", { anim: "shiver", blush: 1 })),
  ],
  pokeAnnoyed: [
    L("okay okay!", F("wide", "o", { anim: "shake", brows: "raised", fx: ["exclaim"] })),
    L("one poke at a time!", F("annoyed", "frown", { anim: "shake", brows: "angry" })),
    L("my slime is wobbling!", F("squint", "wavy", { anim: "jiggle", fx: ["sweat"] })),
    L("so many pokes!", F("wide", "scream", { anim: "shiver", fx: ["exclaim"] })),
    L("easy there, human", F("annoyed", "flat", { anim: "tilt", brows: "raised" })),
  ],
  pokeDizzy: [
    L("@_@ the room is spinning", F("spiral", "wavy", { anim: "spin", fx: ["dizzy"] })),
    L("whoaaa… dizzy…", F("spiral", "o", { anim: "sway", fx: ["dizzy"] })),
    L("which way is up?", F("spiral", "tongue", { anim: "spin", fx: ["dizzy", "question"] })),
  ],
  pokeGrumpy: [
    L("hmph. no more pokes.", F("annoyed", "frown", { anim: "shake", brows: "angry", fx: ["anger"] }), 3200),
    L("i'm a slime, not a button!", F("annoyed", "scream", { anim: "shake", brows: "angry", fx: ["anger", "exclaim"] }), 3200),
  ],
  whileGrumpy: [
    L("…", F("annoyed", "flat", { brows: "angry", fx: ["anger"] })),
    L("not talking to you", F("annoyed", "frown", { anim: "tilt", brows: "angry" })),
    L("i said no more pokes!", F("annoyed", "scream", { anim: "shake", brows: "angry", fx: ["anger", "exclaim"] })),
    L("*grumpy slime noises*", F("annoyed", "wavy", { brows: "angry", fx: ["anger"] })),
  ],
  forgive: [
    L("…okay, i forgive you", F("shiny", "small", { anim: "nod", fx: ["hearts"], blush: 1 }), 3000),
    L("fine. friends again?", F("shiny", "cat", { anim: "tilt", fx: ["hearts"] }), 3000),
  ],
  wakeUp: [
    L("huh?! i wasn't sleeping!", F("wide", "o", { anim: "hop", fx: ["exclaim"] })),
    L("five more minutes…", F("closed", "small", { anim: "melt", fx: ["zzz"] })),
    L("*snort* who's there?", F("wide", "wavy", { anim: "shiver", fx: ["question"] })),
  ],
  pet: [
    L("purrrlorp~", F("closed", "cat", { anim: "sway", fx: ["hearts"], blush: 1 })),
    L("mmm, more pets please", F("happy", "cat", { anim: "sway", fx: ["hearts"] })),
    L("*happy slime noises*", F("happy", "open", { anim: "jiggle", fx: ["hearts", "notes"] })),
    L("you're my favorite human", F("heart", "grin", { anim: "sway", fx: ["hearts"] })),
    L("pets! pets! pets!", F("shiny", "open", { anim: "hop", fx: ["hearts"] })),
    L("i could get used to this", F("closed", "smirk", { anim: "sway", fx: ["hearts"], blush: 1 })),
    L("*melts happily*", F("closed", "grin", { anim: "melt", fx: ["hearts"] })),
    L("right behind the ears, yes", F("happy", "tongue", { anim: "tilt", fx: ["hearts"] })),
  ],
  holdLong: [
    L("squish… squish…", F("squint", "wavy")),
    L("you're squishing me!", F("squint", "scream", { fx: ["sweat"] })),
    L("i am flat now", F("dots", "flat")),
    L("*compressed glorp noises*", F("squint", "wavy", { fx: ["sweat"] })),
  ],
  holdRelease: [
    L("boing!", F("wide", "open", { anim: "boing", fx: ["sparkles"] })),
    L("sproing!", F("happy", "open", { anim: "boing" })),
    L("back to full glorp!", F("star", "grin", { anim: "boing", fx: ["sparkles"] })),
    L("wheee!", F("happy", "open", { anim: "boing" })),
  ],
  idle: [
    L("*hums a slime song*", F("closed", "o", { anim: "sway", fx: ["notes"] }), 3200),
    L("do you think clouds are slime?", F("normal", "cat", { anim: "tilt", fx: ["question"] }), 3400),
    L("achoo!", F("squint", "scream", { anim: "shake", fx: ["droplets"] })),
    L("*yawns*", F("closed", "scream", { anim: "stretch" })),
    L("i could click all day", F("happy", "grin")),
    L("blorp.", F("dots", "flat")),
    L("is it snack time?", F("shiny", "tongue", { fx: ["question"] })),
    L("my antenna picks up good vibes", F("star", "grin", { anim: "nod", fx: ["sparkles"] }), 3200),
    L("fun fact: i have no bones", F("normal", "smirk", { anim: "jiggle" }), 3000),
    L("*wiggles ears*", F("happy", "cat", { anim: "jiggle" })),
    L((c) => `psst… press ${c.hotkey}`, F("wink", "smirk")),
    L("i miss planet glorpulon sometimes", F("teary", "small", { brows: "worried" }), 3400),
    L("hehe", F("happy", "cat", { blush: 1 })),
    L("*stares at your cursor*", F("wide", "flat")),
    L("are you winning, human?", F("normal", "cat", { anim: "tilt", fx: ["question"] })),
    L("not lazy, just energy-saving mode", F("closed", "smirk"), 3200),
    L("slime time!", F("happy", "open", { anim: "hop", fx: ["sparkles"] })),
    L("i wonder what clicks taste like", F("normal", "tongue", { anim: "tilt", fx: ["question"] }), 3200),
    L("*practices clicking in the air*", F("squint", "grin", { anim: "nod" }), 3200),
    L("boop me if you need me", F("wink", "cat")),
    L("my slime is extra shiny today", F("shiny", "grin", { fx: ["sparkles"] })),
    L("i dreamed i was a jelly bean", F("closed", "small", { fx: ["hearts"] }), 3200),
    L("glorp glorp! that's alien for hi", F("happy", "open", { anim: "hop" }), 3200),
    L("*blows a slime bubble*", F("normal", "kiss")),
    L("beep. beep. still here.", F("dots", "o")),
    L("you can pet me, you know", F("shiny", "cat", { blush: 1 }), 3000),
    L((c) => (c.total > 0 ? `${n(c.total)} clicks together so far!` : "ready for our first click?"), F("happy", "grin", { fx: ["sparkles"] }), 3200),
  ],
  clicking: [
    L((c) => `glorp glorp! ${n(c.cps)}/sec`, F("normal", "open")),
    L("clickity clack!", F("happy", "open")),
    L("i am speed", F("star", "grin")),
    L("*intense slime noises*", F("squint", "open")),
    L("this is my cardio", F("normal", "grin", { fx: ["sweat"] })),
    L("click click click", F("happy", "open")),
    L("nyoom", F("wide", "open")),
    L("steady like a metronome", F("closed", "grin")),
    L((c) => `${n(c.clicks)} and counting!`, F("normal", "open")),
  ],
  turbo: [
    L((c) => `TURBO GLORP! ${n(c.cps)}/sec`, F("star", "scream", { fx: ["sparkles"] })),
    L("my antenna is smoking!", F("wide", "scream", { fx: ["sweat"] })),
    L("MAXIMUM GLORP", F("star", "open", { fx: ["sparkles"] })),
    L("faster than light (almost)", F("squint", "grin")),
  ],
  slow: [
    L("glorp… … glorp…", F("closed", "small")),
    L("taking it slow and steady", F("happy", "cat")),
    L("one… click… at a time…", F("closed", "cat")),
  ],
  done: [
    L((c) => `all done! ${n(c.clicks)} clicks`, F("happy", "open", { anim: "spin", fx: ["sparkles"] }), 3200),
    L((c) => `nailed it! ${n(c.clicks)} clicks`, F("star", "grin", { anim: "hop", fx: ["sparkles"] }), 3200),
    L((c) => `${n(c.clicks)} clicks, zero sweat`, F("closed", "smirk", { anim: "nod", fx: ["sparkles"] }), 3200),
  ],
  timeUp: [
    L((c) => `time's up! ${n(c.clicks)} clicks`, F("happy", "open", { anim: "hop", fx: ["sparkles"] }), 3200),
    L((c) => `ding! ${n(c.clicks)} clicks on the clock`, F("star", "open", { anim: "spin", fx: ["sparkles"] }), 3200),
  ],
  stopped: [
    L((c) => `phew! ${n(c.clicks)} clicks`, F("happy", "cat", { anim: "melt" })),
    L("good session!", F("happy", "grin", { anim: "nod" })),
    L((c) => `${n(c.clicks)} clicks. not bad!`, F("wink", "grin")),
    L("*catches breath*", F("closed", "o", { anim: "melt", fx: ["sweat"] })),
  ],
  milestone: [
    L((c) => `WOW! ${n(c.milestone)} clicks total!`, F("star", "scream", { anim: "spin", fx: ["sparkles", "hearts"] }), 4200),
    L((c) => `${n(c.milestone)} lifetime clicks! party!`, F("heart", "open", { anim: "hop", fx: ["sparkles", "hearts"] }), 4200),
  ],
  welcomeBack: [
    L("welcome back!", F("happy", "open", { anim: "hop" })),
    L("you're back!", F("shiny", "open", { anim: "hop", fx: ["hearts"] })),
    L("i missed you", F("heart", "cat", { anim: "sway", fx: ["hearts"] })),
    L("oh hi! i was just… glorping", F("wide", "grin", { anim: "tilt", fx: ["sweat"] })),
  ],
  themeChange: [
    L("ooh, new look!", F("shiny", "open", { anim: "spin", fx: ["sparkles"] })),
    L("do i look good?", F("wink", "grin", { anim: "tilt", blush: 1 })),
    L("fresh slime!", F("star", "grin", { anim: "hop", fx: ["sparkles"] })),
  ],
  greeting: (hour: number) => {
    if (hour >= 5 && hour < 11)
      return [
        L("good morning, human!", F("happy", "open", { anim: "hop" }), 3000),
        L("*yawns* morning…", F("closed", "scream", { anim: "stretch" }), 3000),
      ];
    if (hour >= 11 && hour < 17)
      return [
        L("hi hi! ready to glorp?", F("happy", "open", { anim: "hop" }), 3000),
        L("oh hey, it's you!", F("shiny", "grin", { anim: "tilt", fx: ["hearts"] }), 3000),
      ];
    if (hour >= 17 && hour < 22)
      return [
        L("good evening~", F("happy", "cat", { anim: "sway" }), 3000),
        L("evening glorp session?", F("wink", "grin"), 3000),
      ];
    return [
      L("late night glorping?", F("dots", "small", { anim: "tilt" }), 3000),
      L("shouldn't you be asleep?", F("normal", "smirk", { fx: ["question"], brows: "raised" }), 3000),
    ];
  },
};

export function lineText(r: Reaction, c: LineContext): string {
  return typeof r.text === "function" ? r.text(c) : r.text;
}
