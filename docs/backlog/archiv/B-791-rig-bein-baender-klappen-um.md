# B-791 · Bein-Bänder von Elefant, Wolf und Alphawolf klappen im Rig nicht mehr um

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** NG3
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

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

keine.

Entschieden von 🧑 am 2026-11-11 (Chat): Die Kurve am Band-Ende wird begrenzt; die Specs bleiben unverändert.

## Notizen

Gefunden in NV3.2 (2026-11-11). Umgesetzt in NG3.1 (2026-11-11): Kurve am Band-Ende begrenzt (Elefant) und innere Kante angehalten (Wolf, Alphawolf im Galopp), `knownFolds` leer.

Ergebnis der zwei Spec-Auffälligkeiten (AC-02, beide belegt):
1. `gegner/wolf` und `gegner/alphawolf` haben kein `proportionen.fuss`; `fussVon` fällt auf `huf` zurück (Spitze 3,4), die Reittier-Wölfe haben `pfote` (2,2). Vermutlich ein Spec-Fehler; Änderung nur über `assets_*` mit Freigabe 🧑.
2. `findeReittier` (`tools/paper/rig/reiten.ts`) durchsucht die Nachbar-Gruppen alphabetisch; „gegner“ kommt vor „reittiere“, also findet die Monarch-Spec für `"reittier": "wolf"` den Gegner-Wolf. Fehler in `tools/paper`, braucht ein PAP-Ticket.
