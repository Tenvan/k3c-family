// Fehlerarten der CLI: Aufruf → Exit 2 (Text auf stderr), Abgelehnt → Exit 1 mit {feld, regel, text} (paper-spec.md › 8).

/** Aufruf- oder Systemfehler (Exit 2). */
export class Aufruf extends Error {}

export type CliFehler = { feld: string; regel: string; text: string };

/** Fachlich abgelehnt (Exit 1); Regeln über check.ts hinaus: animation-unbekannt, reittier-unbekannt. */
export class Abgelehnt extends Error {
  constructor(readonly fehler: CliFehler[]) {
    super(fehler.map((f) => `${f.feld}: ${f.text}`).join('; '));
  }
}
