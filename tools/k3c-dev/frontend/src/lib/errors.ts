/** Text einer Ablehnung: Wails lehnt mit dem Fehlertext aus Go als String ab, der Mock mit einem Error. */
export function errorText(e: unknown): string {
  return e instanceof Error ? e.message : String(e);
}
