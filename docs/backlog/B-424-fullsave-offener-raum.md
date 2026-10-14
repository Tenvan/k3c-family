# B-424 · Ein neuer Voll-Ausbau-Stand wird nicht vom noch offenen alten Raum überschrieben

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`Manager.FullSave` (`engine/room/fullsave.go`) schreibt `voll-<seed>` neu, auch wenn ein Raum mit diesem Namen noch offen ist. Ein `voll-`-Raum ist kein Test-Raum und bleibt nach dem Verlassen `EmptyFor` (10 min) offen. Der Level-Betrachter startet danach mit `fresh=false`, `create` tritt dem offenen Raum mit dem alten Zustand bei, und dessen nächstes `save()` überschreibt den frischen Stand. Gefunden im unabhängigen Review von PR #293 (Sprint SV1).

## Ziel

Ein Klick auf „Voll ausgebaut starten“ startet immer den frisch erzeugten Stand, nie einen alten Raum.

## Beteiligte und Zielgruppen

🧑 bei Abnahmen am PC; Agenten bei Messläufen.

## Anforderungen

- Ist ein Raum mit dem Namen `voll-<seed>` offen, wird der neue Stand nicht still überschrieben: entweder Ablehnung mit klarer Meldung (409) oder der Raum wird vorher gespeichert und geschlossen (Entscheidung beim Einplanen).
- Ohne offenen Raum bleibt das Verhalten wie bisher.

## Nicht-Ziele

Regeln des Ausbaus (B-425); andere Spielstände als `voll-*`.

## Regeln und Einschränkungen

SRV: `engine/room/`, `engine/net/`. Kein Datenverlust für Räume anderer Namen.

## Beispiele

`voll-abc` gespielt, Landingpage, sofort wieder V → das Spiel zeigt den neuen voll ausgebauten Stand (oder einen Hinweis, warum nicht).

## Ausnahme- und Fehlerfälle

Raum mit verbundenen Geräten → nie schließen, nur ablehnen.

## Akzeptanzkriterien

- **AC-01** Go-Test: `FullSave` bei offenem Raum gleichen Namens überschreibt den Stand nicht still (Ablehnung oder vorher geschlossener Raum, je nach Entscheidung).
- **AC-02** Go-Test: Der danach gestartete Raum hat den frisch erzeugten Zustand.
- **AC-03** `task check:go` grün.

## Offene Fragen

- Ablehnen (409) oder offenen leeren Raum schließen und neu starten? Vorschlag: leeren Raum schließen, mit verbundenen Geräten ablehnen.

## Notizen

Aus dem unabhängigen Review von PR #293 (2026-10-14); hängt an B-315.
