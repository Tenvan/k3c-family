# B-622 · Fackeln gibt es als Spielobjekt mit Licht in der Nacht

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W15
- **Projekt:** WRT
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Umgesetzt in W15.1 (2026-11-02), seit dem Wegfall des Browser-Clients im nativen Spiel: Fackeln gehören zu gebauter Mauer und gebautem Turm, kein Kauf und keine Bauoption (🧑 2026-10-31). Regel `engine/sim/torches.go` › `TorchXs` (nur `built`, Versätze aus `data/light.json` › `torches`: Mauer `[0]`, Turm `[-0.75, 0.75]`, links der Hub-Mitte gespiegelt). Das Spiel zeichnet Halter und Flamme und hebt nachts die Helligkeit um die Fackel an (`native/k3c-game/world/light.go`, `lightdraw.go`; Licht `torch` Radius 6, Stärke 0,8). Tests: `engine/sim/torches_test.go` (`TestFackelPlaetze`, `TestFackelnDerWelt`), `native/k3c-game/world/light_test.go` › `TestTorchesLikeSim`, gemeinsamer Fall `testdata/torches.json`. Offen ist nur die Sichtprüfung durch 🧑 (W15.3).

## Ziel

Fackeln stehen an gebauten Mauern und Türmen und leuchten nachts; 🧑 hat das am PC gesehen.

## Beteiligte und Zielgruppen

Spielende (jede Zelle im Split-Screen zeigt die Fackeln ihrer Stufe); Agent SIM und CLI (fertig); 🧑 nimmt am PC ab.

## Anforderungen

- Fackeln entstehen aus dem Zustand der Bau-Plätze (kein eigener Zustand, kein Spielstand-Feld): gebaute Mauer eine Fackel, gebauter Turm zwei.
- Nachts und in der Dämmerung leuchten sie mit dem Licht `torch`; am Tag ohne Schein (Halter bleibt sichtbar).
- An zerstörter Mauer entfällt die Fackel.
- Deterministisch, gleich für 1 bis 4 Spieler; Werte nur in `data/light.json`.

## Nicht-Ziele

Neue Lichtberechnung; Fackeln als Kauf oder Bauoption; Fackeln an Burg, Hafen oder frei in der Welt (neues Ticket, falls gewünscht); Fackel als Papierschnitt-Asset (heute gezeichnete Form).

## Regeln und Einschränkungen

Das Spiel zeichnet nur, die Regel steht in `engine/sim`. Werte in `data/light.json` (nur im Spiel sichtbar, keine Wirkung auf die Simulation).

- **Bereich:** `data/light.json`
- **Abhängig von:** W15.2
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Gebauter Turm → zwei Fackeln (0,75 Units links und rechts der Mitte), nachts heller Kreis mit Radius 6. Gebaute Mauer → eine Fackel in der Mitte.

## Ausnahme- und Fehlerfälle

Mauer zerstört (wieder unbezahlt) → keine Fackel. `data/light.json` ohne `torches` oder ohne Licht `torch` → keine Fackeln, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Fackeln stehen an gebauter Mauer und gebautem Turm und leuchten nachts: Tests `TestFackelPlaetze`, `TestFackelnDerWelt`, `TestTorchesLikeSim` grün (erfüllt, W15.1/W15.2) und 🧑 bestätigt es am PC (W15.3, offen).

## Offene Fragen

- Wo stehen Fackeln? → entschieden (🧑, 2026-10-31, Sprint W15): an gebauter Mauer und gebautem Turm, sonst nirgends.
- Reichen Radius 6 und Stärke 0,8? → Default: ja, Werte bleiben (vorläufig, 🧑 bestätigt bei der Abnahme W15.3; Änderung nur in `data/light.json`).

## Notizen

Gefunden in W13.1. Stand geprüft 2026-11-15: Umsetzung im nativen Spiel vorhanden (`native/k3c-game/world/light.go`:112–131, `lightdraw.go`:103). Der Sprint W15 ist seit 2026-10-31 von Ralf freigegeben (Revision 1) und umfasst diese Spec; das Ticket selbst steht noch auf `Entwurf`. Vorschlag: Freigabe des Sprints auf das Ticket übertragen und nach W15.3 Status erledigt.
