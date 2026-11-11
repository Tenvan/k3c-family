# NG3.1 · Bein-Bänder der Vierbeiner falten nicht mehr

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng3/1-bein-baender
- **Abhängig von:** NG1
- **Tickets:** B-791
- **Kriterien:** AC-01

## Ziel

Die Bein-Bänder von Elefant, Gegner-Wolf und Alphawolf falten in keiner Phase; `knownFolds` ist leer.

## Kontext

Spec: B-791. `native/k3c-rig/rig/band_test.go` (`TestLegStripNeverFolds`, Liste `knownFolds`), Rig-Format `native/k3c-rig/RIG.md`, Sollwerte aus `tools/paper` (Toleranz 0,01, Test aus NV3.2). Elefant: Fußspitze 1,2 Einheiten, Kurve schießt darüber; Wolf und Alphawolf: Galopp an der Hüfte. Dazu zwei Auffälligkeiten klären: Gegner-Wolf ohne `proportionen.fuss` mit Huf-Spitze; `findeReittier` in `tools/paper` findet für `*-wolf` vermutlich den Gegner-Wolf (ungeprüft). Specs ändern nur über `assets_*` und nur, wenn 🧑 das in der Freigabe so entschieden hat (Default: Kurve begrenzen, Spec bleibt). Rig-Mathematik: Opus.

## Erlaubte Dateien

- `native/k3c-rig/rig/**`
- Planung NG3, B-791 (über MCP)

## Nicht-Ziele

Neue Animationen; Sollwerte ändern; `tools/paper` ändern (Befund dort → Ticket PAP).

## Schritte

1. `knownFolds` leeren → Test rot.
2. Band-Geometrie korrigieren, bis der Test für alle Figuren grün ist; Posen-Test gegen Sollwerte bleibt grün.
3. Die zwei Auffälligkeiten prüfen und in B-791 › Notizen eintragen.

## Fertig, wenn

- [ ] AC-01: `go test ./rig/ -run TestLegStripNeverFolds` in `native/k3c-rig` grün ohne Ausnahmen; Posen-Test grün; Notizen in B-791.
- [ ] `task check:diff` grün (bzw. `go test ./...` in `native/k3c-rig`, solange check:diff das Modul nicht kennt).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

B-791/AC-01 umgesetzt: `knownFolds` ist leer. Elefant: Tangente der Band-Kurve je Stützpunkt begrenzt (höchstens 3× der kürzere Abschnitt). Wolf und Alphawolf: die Beugung im Galopp ist enger als die Bandbreite, `unfold` hält die innere Kante an (Dreieck mit Fläche 0 statt negativ). Specs unverändert. Geprüft mit `go test ./rig/...` (`TestLegStripNeverFolds` ohne Ausnahmen, Posen gegen Sollwerte, `TestHermiteLimitsOvershoot`, `TestUnfoldStopsBackwardEdge`).
B-791/AC-02: beide Spec-Auffälligkeiten belegt, siehe B-791 › Notizen.
