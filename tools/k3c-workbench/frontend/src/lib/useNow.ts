import { useEffect, useState } from 'react';

/** Aktuelle Zeit in ms, alle ms neu (Laufzeit, wanderndes Zeitfenster der Live-Monitore). */
export function useNow(everyMs: number): number {
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    const t = setInterval(() => setNow(Date.now()), everyMs);
    return () => clearInterval(t);
  }, [everyMs]);
  return now;
}
