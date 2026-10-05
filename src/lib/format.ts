/** 42s · 3m 12s · 2h 05m · 3d 4h */
export function fmtDuration(totalSecs: number): string {
  const s = Math.max(0, Math.floor(totalSecs));
  if (s < 60) return `${s}s`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m ${String(s % 60).padStart(2, "0")}s`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h ${String(m % 60).padStart(2, "0")}m`;
  return `${Math.floor(h / 24)}d ${h % 24}h`;
}

/** Countdown style: 4:05 · 1:02:09 */
export function fmtClock(totalSecs: number): string {
  const s = Math.max(0, Math.ceil(totalSecs));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${sec}` : `${m}:${sec}`;
}

/** 950 · 12.4k · 3.1M */
export function fmtCompact(n: number): string {
  if (n < 10_000) return Math.round(n).toLocaleString();
  return new Intl.NumberFormat(undefined, { notation: "compact", maximumFractionDigits: 1 }).format(n);
}

export function fmtCps(n: number): string {
  return n >= 10 ? Math.round(n).toString() : n.toFixed(1);
}

/** Local-time "YYYY-MM-DD", matching the keys the backend uses. */
export function dayKey(d: Date): string {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}
