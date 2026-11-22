/** Fehlertext eines abgelehnten Aufrufs (Server-Text unverändert). */
export const text = (e: unknown) => (e instanceof Error ? e.message : String(e));
