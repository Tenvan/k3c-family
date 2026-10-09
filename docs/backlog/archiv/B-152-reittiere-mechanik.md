# B-152 · Jeder Monarch reitet von Anfang an auf einem Standard-Reittier

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** S1
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Reittiere sind nur als Grafik vorhanden (13 Tiere mit Sattelpunkten in `data/sprites.json` › `mounts`, Referenz in `src/tools/spriteReference.ts`); in `engine/sim/` gibt es kein Reittier, der Monarch läuft als Figur. Im Vorbild Kingdom Two Crowns sitzt der Monarch von Anfang an auf einem Reittier. 🧑 hat am 2026-10-03 entschieden (Fragenkatalog Q23): Reittiere von Anfang an, mit einem Standard-Reittier (Revision 2 ersetzt „nach Phase 3, als Politur“).

## Ziel

Jeder Monarch hat von Beginn an ein **Standard-Reittier** (Pferd), das seine Bewegung bestimmt. Die Daten kennen weitere Reittiere, die später als Auswahl oder Belohnung folgen können. Nutzen: Das Spiel fühlt sich wie das Vorbild an, und die Bewegung (Geschwindigkeit, Sprint) hängt an einem Datenwert statt am Monarchen.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); Werte pflegt REG (Workshop F1), Umsetzung durch Agent in Go.

## Anforderungen

- `data/monarch.json` › `mount`: Standard-Reittier (Name, Geschwindigkeitsfaktor, Sprintfaktor, Sprite-Schlüssel aus `sprites.json` › `mounts`); Werte laut Beschluss aus F1, Startwert Faktor 1,0 = heutige Geschwindigkeit, damit sich das Balancing nicht verschiebt.
- Geschwindigkeit des Monarchen = Basis × Reittier-Faktor; Sprint wie bisher auf dem Reittier.
- Jeder Monarch hat das Reittier beim Beitritt (auch späte Beitretende, Wiederverbinden, Stufenwechsel); es wird weder gekauft noch verloren (Standard-Reittier). Der Monarch ist immer beritten, es gibt kein Auf- und Absteigen.
- Das Reittier ist aus den Daten bekannt; ein Protokoll-Feld ist nur nötig, falls ein Monarch ein anderes als das Standard-Reittier hat (dann eigene Session nach `arbeitsweise.md` › Domänen).
- Reittier-Zustand im Spielstand nur, wenn ein anderes als das Standard-Reittier möglich ist (Versionssprung und Fixture nach Migrationsregel, B-137); bei reinem Standard kein Spielstandfeld.
- Deterministisch, mit 2+ Spielern gleichzeitig.

## Nicht-Ziele

Darstellung (B-173), weitere Reittiere als Auswahl, Kauf oder Fund neuer Reittiere, Reittiere für Truppen und Bürger, Ton.

## Regeln und Einschränkungen

`CLAUDE.md` › Regeln: keine Sprünge, nur `engine/rng`, Werte in `data/`. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Die Reittier-Regel gehört in `docs/rules/monarch.md` (Abschnitt „Reittier“), beschlossen in F1.

## Beispiele

Zwei Spieler treten bei → beide laufen mit dem Faktor des Standard-Reittiers aus den Daten; ändert 🧑 den Faktor in `monarch.json`, ändern sich beide Geschwindigkeiten ohne Codeänderung.

## Ausnahme- und Fehlerfälle

Unbekannter Sprite-Schlüssel in den Daten → Test schlägt fehl (Datenprüfung), kein Absturz im Spiel. Spielstand ohne Reittier-Feld (alle bisherigen) → Standard-Reittier.

## Akzeptanzkriterien

- **AC-01** `docs/rules/monarch.md` enthält den Abschnitt „Reittier“ mit Standard-Reittier, Geschwindigkeits- und Sprintfaktor (Beschluss von 🧑 in F1).
- **AC-02** Test in `engine/sim/`: Jeder Monarch hat beim Beitritt das Standard-Reittier aus den Daten, seine Geschwindigkeit entspricht Basis × Faktor; zweimal mit demselben Seed gleiches Ergebnis; mit 2 Spielern.
- **AC-03** Ein alter Spielstand lädt weiter (Fixture), Golden-Daten sind aktualisiert; `task check:go` grün.

## Offene Fragen

Faktoren des Standard-Reittiers und ob die Wahl des Reittiers später Spieloption wird: 🧑 im Workshop F1 (Q23 entschieden: Standard-Reittier von Anfang an).

## Notizen

Revision 2 am 2026-10-03: Beschluss Q23 (von Anfang an, Standard-Reittier) statt „nach Phase 3“; Sprint S1 statt „kein Sprint“.
