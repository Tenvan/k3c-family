# GR9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Monarch-Animationen für Wurf, Treffer, Tod und Respawn, Kandidaten-Import ohne roten Test
- **Projekt:** GRA

## Ziel

Kandidaten lassen sich importieren, ohne `task check` zu brechen, und der Monarch hat eigene Animationen für Wurf, Treffer, Tod und Respawn. Am Ende sichtbar: Monarch-Seite ohne Lücken 1–4, Animationen im Spiel am TV, von 🧑 abgenommen.

## Ergebnis

- 2026-10-22, Review GR9.4 (Reviewer-Agent, Diff `sprint/gr8...sprint/gr9`): keine schweren Befunde; `task check` und `task check:go` grün. AC-01 (GR9.1) umgesetzt und geprüft. - 2026-10-24, GR9.5: AC-02 von Ralf am PC abgenommen (give, hit, death, respawn ok); B-505 und B-420 erledigt. - Hinweise des Reviews (kein Befund): Treffer im selben Frame wie `give` verwirft `give`; nach Neubau der View wird ein Treffer im ersten Frame nicht gezeigt. Beritten bleibt der Tod bei Deckkraft 0,25. - Version: v0.19.0 vorgeschlagen, gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Kandidaten unter `public/grafik/kandidaten/` mit Credit lassen `grafikPacks.test.ts` grün, … |
| AC-02 | geprüft | Der Monarch hat Animationen für Wurf, Treffer, Tod und Respawn in Sheet und `data/sprites. … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-420 | erledigt | AC-01 – | Mit k3c-workbench importierte Kandidaten unter public/grafik/kandidaten/ brechen … |
| B-505 | erledigt | AC-02 geprüft | Monarch hat Animationen für Wurf, Treffer, Tod und Respawn |
