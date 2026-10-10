# GL1 · PLAT · Landingpage in der gewählten Sprache

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** B-369
- **Start-Commit:** 87f25c20
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-10, 🧑 im Chat (Vorschläge übernehmen + freigeben; nur Xbox-Symbole, B-339 verworfen)

## Ausgangslage

Die Landingpage bleibt nach dem Sprachwechsel deutsch, ihre Texte stehen als Literale in `src/landing/` und `index.html` (B-369). Spiel, Shell, Touch-Overlay und Werkzeug-Seiten folgen der Sprache schon (S5.3, PL1, PL2). Revision 2 (2026-10-10, 🧑): Die Plattformen sind nur Xbox, PC und Mobile (Touch), jedes Pad zeigt Xbox-Symbole; B-339 (Tastensymbole je Controller) ist verworfen, GL1 setzt nur noch B-369 um.

## Ziel

Die Landingpage folgt der gewählten Sprache, mit Rückfall Deutsch. Am Ende sichtbar: Nach dem Sprachwechsel auf English in den Optionen zeigt die Landingpage nach dem Neuladen englische Kacheln, Statuszeile, den Vollbild-Knopf und die Fußleiste.

## Beteiligte und Zielgruppen

Alle, die an PC, Handy, TV oder Xbox die Landingpage als Einstieg sehen. Der Agent arbeitet in PLAT (`src/landing/`, `index.html`, `src/core/texts.*.ts`). 🧑 nimmt den Sprachwechsel am Gerät ab.

## Anforderungen

B-369 › Anforderungen.

## Nicht-Ziele

Tastensymbole je Controller-Familie (B-339 verworfen, nur Xbox-Symbole); weitere Sprachen; Home-Button-Text und Touch-Overlay (B-215, erledigt in PL1); Übernahme der Sprache ohne Neuladen; Tasten von Spieler 2 an der Tastatur (B-371, AZ1); Spielname „Family Three Crowns“ bleibt unübersetzt.

## Regeln und Einschränkungen

- Nur Domäne PLAT. Was eine andere Domäne bräuchte, wird ein Ticket.
- `CLAUDE.md` › Seiten & Navigation (Landingpage bleibt offen, iframe). Texte über `t()` in de und en, Rückfall Deutsch.
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.

## Beispiele

B-369 › Beispiele.

## Ausnahme- und Fehlerfälle

B-369 › Ausnahme- und Fehlerfälle (unbekannte Sprache → Deutsch, Übernahme beim Neuladen).

## Akzeptanzkriterien

IDs bleiben stabil; die Kriterien aus B-339 entfallen mit Revision 2 (B-339 verworfen).

- **AC-01** Entfällt (war `B-339/AC-01`, B-339 verworfen).
- **AC-02** `src/landing/*.ts` und `index.html` enthalten kein deutsches Text-Literal mehr, die Texte stehen in den Textdateien für de und en (`B-369/AC-01`).
- **AC-03** Entfällt (war `B-339/AC-04`, B-339 verworfen).
- **AC-04** Entfällt (war `B-339/AC-02`, B-339 verworfen).
- **AC-05** Entfällt (war `B-339/AC-03`, B-339 verworfen).
- **AC-06** Nach Sprachwechsel in den Optionen zeigt die Landingpage nach dem Neuladen die gewählte Sprache (`B-369/AC-02`, 🧑 am Gerät).
- **AC-07** Entfällt (war `B-339/AC-05`, B-339 verworfen).

## Offene Fragen

Keine. Entschieden 2026-10-10 (🧑): Plattformen sind nur Xbox, PC und Mobile, nur Xbox-Symbole (B-339 verworfen); Symbole bleiben selbst gezeichnet; die Landingpage übernimmt die Sprache nur beim Neuladen; die Tastatur wird nicht je Layout beschriftet.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GL1.1 | `GL1.1-landing-texte.md` | Umsetzung | autonom | fertig |
| GL1.2 | `GL1.2-review.md` | Review | autonom | offen |
| GL1.3 | `GL1.3-abnahme-sprache.md` | Workshop | Mensch | offen |

## Abnahme

–
