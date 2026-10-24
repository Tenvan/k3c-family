/** Abstand zu jetzt in Worten: „gerade“, „vor 5 min“, „vor 3 h“, „vor 2 Tagen“; 0 = „nie“. */
export function ago(ms: number, now = Date.now()): string {
  if (ms <= 0) return 'nie';
  const min = Math.max(0, Math.floor((now - ms) / 60_000));
  if (min < 1) return 'gerade';
  if (min < 60) return `vor ${min} min`;
  const h = Math.round(min / 60);
  if (h < 36) return `vor ${h} h`;
  const d = Math.round(h / 24);
  return d === 1 ? 'vor 1 Tag' : `vor ${d} Tagen`;
}
