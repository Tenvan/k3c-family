# B-531 · Wald hat mehr Bäume je Abschnitt, Höhle bekommt Deko-Büsche

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB10 hat die Bäume 11,25 Kacheln hoch und 2–4 Kacheln breit gemacht; dichter wirkt der Wald nur über die breiteren Kronen. Wie viele Bäume ein Abschnitt hat, steht in `data/biomes/forest.json` › `resourcesPerChunk` (z. B. `forest: tree [4, 7]`). Der Level-Generator setzt Büsche nur im Wald (`clearing`), der `hoehlenbusch` aus GEB10 erscheint deshalb im Spiel nicht.

## Ziel

Der Wald steht sichtbar dichter (mehr Bäume je Abschnitt), die Höhle zeigt Deko-Büsche (Höhlenbusch).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** 🧑 legt die Baumzahl je Abschnitt im Wald fest (Werte in `data/biomes/forest.json`), Balancing (Holz) bleibt im Rahmen.
- **AC-02** Höhle (und auf Wunsch Mine) bekommt `bush` als Deko in `resourcesPerChunk`; der Client zeigt dort den Höhlenbusch (GEB10.3).

## Offene Fragen

🧑: Wie viel dichter (Holz-Balancing)? Höhlenbüsche auch in der Mine?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
