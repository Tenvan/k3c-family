# B-297 · Das Delta überträgt, dass ein Feld aus dem Zustand verschwindet

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** DL1
- **Erstellt:** 2026-10-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat (Ralf: „B-296: Option (a), erst SRV-Session fürs Delta“), Revision 1, aus dem Auftrag abgeleitet; mit Sprint DL1

## Ausgangslage

`engine/net/delta.go` (`deltaOf`) schickt nur Felder, die in `cur` stehen. Fehlt ein optionales Feld (`omitempty`) im
neuen Zustand, behält der Client (`src/online/clientDelta.ts`, `applyDelta`) den alten Wert. Betroffen: `merchant`
(W4.2, Händler reist ab), `storms` und `drops` (W4.3a, letzte Ausrüstung aufgehoben). Gefunden in W4.3a (Frage B-296
auf `sprint/w4`, Entscheidung 🧑: zuerst eine SRV-Session). Dazu verlangt `TestDeltaErgibtJedenVollenZustand`
Burgschaden im Fenster ab Tick 2400 von `sim-cave-belagerung`; mit der Verlust-Kaskade aus W4.3a gibt es dort keinen mehr.

## Ziel

Ein Feld, das aus dem Zustand verschwindet, verschwindet auch beim Client; der Delta-Test bleibt mit W4.3a aussagekräftig.

## Beteiligte und Zielgruppen

Spieler (kein Geister-Händler, keine Geister-Ausrüstung), Entwickler SIM (W4.3a), SRV und CLI (Protokoll).

## Anforderungen

- Der Server nennt im `delta` die Namen der Felder, die im vorigen Zustand standen und jetzt fehlen (`events` ausgenommen),
  in einer Liste `unset` (sortiert, fehlt, wenn keine); der Client entfernt diese Felder.
- `null` bleibt ein Wert (kein Löschen), wie in `docs/protocol.md` › Zustand und Delta.
- `docs/protocol.md` beschreibt `unset`; ein Beispiel in `testdata/protocol/` zeigt es, falls die Form-Prüfung das verlangt.
- `TestDeltaErgibtJedenVollenZustand` prüft weiter, dass sich `castle` im Lauf ändert, auch mit dem Stand von W4.3a.

## Nicht-Ziele

Anzeige von Händler und Ausrüstung (W6, B-126), Verlust-Kaskade selbst (W4.3a), neue Protokoll-Nachrichten.

## Regeln und Einschränkungen

Protokoll-Änderung in eigener Session, die nur das Protokoll und beide Enden anpasst (`docs/arbeitsweise.md` › Domänen).
Additiv: Client und Server werden gemeinsam ausgeliefert. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Händler reist ab → `merchant` fehlt im Zustand → `delta` enthält `"unset": ["merchant"]` → der Client hat kein `merchant` mehr.

## Ausnahme- und Fehlerfälle

Feld wird `null` → steht als Wert im Delta, nicht in `unset`. Kein Feld verschwindet → `unset` fehlt.
Ein Feld verschwindet und kommt im selben Tick nicht wieder → genau einmal in `unset`.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein Feld, das von `prev` zu `cur` fehlt (z. B. `merchant`, `drops`), steht in `unset`, und `apply`
  ergibt genau `cur`; `null` und `events` stehen nie in `unset`.
- **AC-02** Vitest: `applyDelta` entfernt die Felder aus `unset` und lässt alles andere wie bisher.
- **AC-03** `TestDeltaErgibtJedenVollenZustand` ist grün und prüft die Änderung von `castle`, auch mit dem Stand von
  `wip/w4.3a-krieger` (lokal gemergt, nicht eingecheckt).
- **AC-04** `docs/protocol.md` beschreibt `unset`; `task check` und `task check:go` grün.

## Offene Fragen

keine

## Notizen

Messung W4.3a: `go test ./engine/net -run TestDelta` meldet „Tick 3057: Zustand weicht ab“; ohne `drops` im JSON
„Feld castle hat sich im Lauf nie geändert“.
