# B-791 · Bein-Bänder von Elefant, Wolf und Alphawolf klappen im Rig nicht mehr um

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG3
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit NV3.2 exportiert `native/k3c-rig/export` alle Figuren mit Teile-Baum. Bei drei Vierbeinern klappt das Dreiecksband der Beine um (`native/k3c-rig/rig/band_test.go`, Liste `knownFolds`): beim Elefanten an der Fußspitze in jeder Pose (Spitze 1,2 Einheiten, die Kurve schießt darüber hinaus), bei Gegner-Wolf und Alphawolf im Galopp an der Hüfte. Die Posen selbst stimmen mit `tools/paper` überein. Dazu zwei Auffälligkeiten aus dem Export: Der Gegner-Wolf hat ohne `proportionen.fuss` eine Huf-Spitze; `findeReittier` in `tools/paper` findet für die Monarch-Animationen `*-wolf` vermutlich den Gegner-Wolf statt des Reittiers (ungeprüft).

## Ziel

Alle Vierbeiner zeichnen ihre Beine im Rig ohne umgeklappte Flächen; der Falt-Test gilt wieder für alle Figuren.

## Beteiligte und Zielgruppen

Claude (Umsetzung), 🧑 Ralf (Bild).

## Anforderungen

- Bein-Band von Elefant, Wolf und Alphawolf faltet in keiner Phase.
- `knownFolds` ist leer.
- Die zwei Spec-Auffälligkeiten sind geklärt (Fehler oder gewollt).

## Nicht-Ziele

Neue Animationen; Änderung der Sollwerte ohne Grund.

## Regeln und Einschränkungen

Rig-Format `native/k3c-rig/RIG.md`; Specs nur über `assets_*`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-rig/rig/**`
- **Abhängig von:** NV3
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Elefant im Schritt: Fußspitze ohne umgeklapptes Dreieck.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Zeichen-Geometrie.

## Akzeptanzkriterien

- **AC-01** `go test ./rig/ -run TestLegStripNeverFolds` grün ohne Ausnahmen.
- **AC-02** Die zwei Spec-Auffälligkeiten stehen mit Ergebnis in den Notizen.

## Offene Fragen

Kurve am Band-Ende begrenzen oder Spec des Elefanten ändern (längere Spitze)? Entscheidet die Umsetzung mit 🧑.

## Notizen

Gefunden in NV3.2 (2026-11-11).
