# B-296 · Die Verlust-Kaskade (W4.3a) lässt sich ohne Änderung an `engine/net` nicht grün umsetzen

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W4.3a (Sprint W4, Domäne SIM) baut die Verlust-Kaskade (Q67): Bürger sterben nicht mehr, Ausrüstung fällt als
`World.Drops` (JSON `drops`, `omitempty`) zu Boden. Der Stand liegt auf `wip/w4.3a-krieger` (ohne Tests und Golden).
Damit wird `engine/net` › `TestDeltaErgibtJedenVollenZustand` rot, aus zwei Gründen außerhalb der erlaubten Dateien:

1. **Delta entfernt keine Felder** (`engine/net/delta.go`, `docs/protocol.md` › Zustand und Delta): Verschwindet
   `drops` aus dem Zustand (letzte Ausrüstung aufgehoben), behält der Client die alte Liste. Dasselbe gilt schon für
   `merchant` aus W4.2 (Händler reist ab) und für `storms`; dort fängt es kein Test. Ohne `omitempty` stünden die
   Felder immer im Zustand, dann scheitert `TestFormWieBeispiele` an den Beispielen in `testdata/protocol/` (W5).
2. **Testfenster ohne Burgschaden:** Der Test spielt `sim-cave-belagerung` ab Tick 2400 und verlangt, dass sich
   `castle` ändert. Mit der Kaskade binden zurückgestufte Bürger die Gegner länger, die Burg nimmt im Fenster keinen
   Schaden mehr.

Außerdem braucht `disarmed.cause` eine Zeile in `engine/sim/common.go` (`applyDamageBy` merkt sich die Gegnerart an
der Truppe); `common.go` steht nicht in den Erlaubten Dateien von W4.3a.

## Ziel

W4.3a kann die Kaskade mit grünem `task check:go` abschließen.

## Beteiligte und Zielgruppen

Entwickler (SIM, SRV); 🧑 entscheidet über Reihenfolge und Domänen-Ausnahme.

## Anforderungen

- Das Delta überträgt das Verschwinden eines optionalen Zustandsfelds (z. B. `null` senden und im Client löschen) oder
  die Felder `drops` und `merchant` stehen immer im Zustand und in den Protokoll-Beispielen.
- `TestDeltaErgibtJedenVollenZustand` deckt Burgschaden weiter ab (anderes Fenster oder anderer Lauf).

## Nicht-Ziele

Anzeige von Ausrüstung und Händler (W6, B-126).

## Regeln und Einschränkungen

Protokoll-Änderungen bekommen eine eigene Session, die nur das Protokoll und beide Enden anpasst
(`docs/arbeitsweise.md` › Domänen). Ein Sprint bleibt in seiner Domäne.

## Beispiele

Ein Bauer hebt das letzte Schwert am Boden auf → `drops` fehlt im Zustand → der Client zeigt kein Schwert mehr.

## Ausnahme- und Fehlerfälle

Händler reist ab → `merchant` fehlt → der Client zeigt keinen Händler mehr.

## Akzeptanzkriterien

- **AC-01** Mit dem Stand von `wip/w4.3a-krieger` (nach Golden-Update) ist `task check:go` grün, ohne dass W4.3a
  Dateien außerhalb von SIM ändert.

## Offene Fragen

Welcher Weg (entscheidet 🧑)? (a) Eine SRV-Session vor W4.3a (Delta-Entfernen plus neues Testfenster), dann W4.3a
weiter; (b) W4.3a darf ausnahmsweise `engine/net/delta_test.go` und `engine/sim/common.go` (eine Zeile) ändern, das
Delta-Entfernen folgt mit W5; (c) Felder ohne `omitempty`, W5 zieht `testdata/protocol/` nach.

## Notizen

Gefunden in W4.3a (Sprint W4). Messung: `go test ./engine/net -run TestDelta` meldet „Tick 3057: Zustand weicht ab“;
ohne `drops` im JSON „Feld castle hat sich im Lauf nie geändert“.
