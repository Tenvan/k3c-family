# SP05 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Port I – Welt, Zyklus, Truppen, Wirtschaft
- **Projekt:** –

## Ziel

Die Go-Simulation (`engine/sim`) rechnet Welt, Tag/Nacht, eigene Truppen und Wirtschaft Tick für Tick wie TypeScript. Am Ende sichtbar: `go test ./engine/sim/` vergleicht die Golden-Läufe ohne Gegner Feld für Feld und ist grün.

## Ergebnis

- 2026-10-01, leichtes Review über `f015b3c..main` per unabhängigem Subagent (auf Auftrag von 🧑 im selben Lauf). Kriterien: AC-01 (SP05.1), AC-03 (SP05.2), AC-02 (SP05.3), alle mit Gegentest belegt; B-043/AC-02 folgt in SP06. - Behobene Befunde: zwei Produkte ohne `float64(…)` (FMA) in `movePlayer` und `cycleAt`; `golden.Diff` übersah Schlüssel, die nur auf einer Seite stehen. - Neue Tickets: B-074 (Golden-Lücken der Wirtschaft, eingeplant für SP06).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Zustand, `createWorld`, `addPlayer` und Tag/Nacht: Der Snapshot bei Tick 0 aller Golden-Lä … |
| AC-02 | geprüft | Die Golden-Läufe ohne Gegner sind grün: `sim-forest-tag` vollständig, `sim-forest-nacht` u … |
| AC-03 | geprüft | Eigene Truppen: `sim-forest-ohne-spieler` ist in allen Snapshots grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-043 | erledigt | – | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
