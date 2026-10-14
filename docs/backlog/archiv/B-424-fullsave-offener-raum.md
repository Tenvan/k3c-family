# B-424 · Ein neuer Voll-Ausbau-Stand wird nicht vom noch offenen alten Raum überschrieben

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-14 🧑 Chat (Umsetzungsauftrag mit Entscheidung)

## Ausgangslage

`Manager.FullSave` (`engine/room/fullsave.go`) schreibt `voll-<seed>` neu, auch wenn ein Raum mit diesem Namen noch offen ist. Ein `voll-`-Raum ist kein Test-Raum und bleibt nach dem Verlassen `EmptyFor` (10 min) offen. Der Level-Betrachter startet danach mit `fresh=false`, `create` tritt dem offenen Raum mit dem alten Zustand bei, und dessen nächstes `save()` überschreibt den frischen Stand. Gefunden im unabhängigen Review von PR #293 (Sprint SV1).

## Ziel

Ein Klick auf „Voll ausgebaut starten“ startet immer den frisch erzeugten Stand, nie einen alten Raum.

## Beteiligte und Zielgruppen

🧑 bei Abnahmen am PC; Agenten bei Messläufen.

## Anforderungen

- Ist ein Raum mit dem Namen `voll-<seed>` offen und **kein Gerät verbunden**, schließt `FullSave` ihn zuerst (ausstehende Speicherungen abwarten, Bericht schreiben, Code frei) und schreibt danach den neuen Stand. Wartende Monarchen (getrenntes Gerät) zählen als nicht verbunden.
- Ist **ein Gerät verbunden**, lehnt `FullSave` ab (`ErrSaveInUse`, HTTP 409) und schreibt nichts.
- Ohne offenen Raum bleibt das Verhalten wie bisher.
- Zwischen Schließen und Schreiben kann kein Gerät dem alten Raum beitreten.

## Nicht-Ziele

Regeln des Ausbaus (B-425); andere Spielstände als `voll-*`.

## Regeln und Einschränkungen

SRV: `engine/room/`, `engine/net/`. Kein Datenverlust für Räume anderer Namen.

## Beispiele

`voll-abc` gespielt, Landingpage, sofort wieder V → das Spiel zeigt den neuen voll ausgebauten Stand (oder einen Hinweis, warum nicht).

## Ausnahme- und Fehlerfälle

Raum mit verbundenen Geräten → nie schließen, nur ablehnen.

## Akzeptanzkriterien

- **AC-01** Go-Test: Offener leerer Raum `voll-<seed>` wird von `FullSave` geschlossen; der danach mit `fresh=false` gestartete Raum hat den frisch erzeugten Zustand, nicht den alten.
- **AC-02** Go-Test: Mit verbundenem Gerät liefert `FullSave` `ErrSaveInUse`, der Stand im Store bleibt unverändert, der Raum läuft weiter; `POST /api/dev/fullsave` antwortet 409.
- **AC-03** `task check:go` grün.

## Offene Fragen

– (geklärt 2026-10-14 🧑 im Chat: leeren Raum schließen, bei verbundenen Geräten ablehnen)

## Notizen

Aus dem unabhängigen Review von PR #293 (2026-10-14); hängt an B-315.

Umgesetzt 2026-10-14 ohne Sprint, auf ausdrücklichen Auftrag von 🧑 im Chat (Branch `fix/b-424-fullsave-offener-raum`):
- `Manager.closeEmpty` (`engine/room/fullsave.go`) schließt unter `m.mu` den offenen Raum mit dem Namen: ausstehende Speicherungen abwarten (`flush`), Bericht, `closed`, Code frei, Log 🧹. Mit verbundenem Gerät `ErrSaveInUse` (Log 🚫). `FullSave` hält `m.mu` bis nach dem Schreiben, damit niemand dazwischen beitritt.
- `POST /api/dev/fullsave` antwortet bei `ErrSaveInUse` mit 409 (`engine/net/dev.go`). Das WebSocket-Protokoll bleibt unverändert, der Fehler kommt nur über HTTP.
- **AC-01** geprüft: `TestVollAusgebautSchliesstLeerenRaum`. Alter Raum zu, Store und neuer Raum haben Hub 5, nicht den geänderten alten Zustand.
- **AC-02** geprüft: `TestVollAusgebautLehntBeiVerbundenemGeraetAb` (Store unverändert, Raum läuft) und `engine/net` › `TestDevVollAusgebautRaumBelegt` (409).
- **AC-03** geprüft: `task check:go` grün.
