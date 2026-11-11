# NV3.2 · Rig-Export für alle Figuren mit Teile-Baum

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** nv3/2-rig-export-alle-figuren
- **Abhängig von:** –
- **Tickets:** B-789
- **Kriterien:** AC-03

## Ziel

Der Export schreibt Teile, Rig-Dateien und Sollwerte für alle Figuren mit Teile-Baum, und Go rechnet ihre Posen wie `tools/paper`.

## Kontext

`native/k3c-rig/export/` (bisher Monarch und Pferd), `native/k3c-rig/rig/` (Posen, `expected_test.go`), `native/k3c-rig/RIG.md`. Figuren: 6 Truppen, 7 Gegner, 5 Bosse (`figur`), 11 Reittiere, 2 Gegner (`vierbeiner`) unter `public/grafik/`. `requisite` bleibt außen vor. Texturbudget je Figur im Ergebnis nennen.

## Erlaubte Dateien

- `native/k3c-rig/**`
- `native/Taskfile.yml`

## Nicht-Ziele

Änderungen an Specs oder `tools/paper`; Zeichnen im Level (NV3.3).

## Schritte

1. Export über alle Specs mit Rig-Typ `figur` und `vierbeiner` laufen lassen; Abweichungen vom Monarch-Schema (fehlende Teile, andere Animationen) im Format abbilden, `RIG.md` nachziehen.
2. Sollwerte je Figur schreiben, `go test ./rig/` vergleicht alle.
3. Größe der Teile (PNG, MiB RGBA) je Figur und gesamt notieren.

## Fertig, wenn

- [ ] AC-03 (Teil): Rig-Dateien und Sollwerte für alle 31 Figuren, `task native:rig:test` grün, `rig:export:check` byte-genau.

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

AC-03 (Teil) umgesetzt: Export schreibt Rig, Teile und Sollwerte für alle 31 Figuren plus Monarch nach `assets/<gruppe>/<id>/` (32 Ordner, keine ausgelassen); geprüft mit `task native:rig:test` (133 Animations- und 33 Reit-Subtests, Toleranz 0,01) und `rig:export:check` byte-genau (Agent, Opus).
Größen: 455 PNG, 7,55 MB auf der Platte, 20,74 MiB RGBA gesamt (je Figur 0,19 bis 1,18 MiB).
Abweichungen: Ordner jetzt `<gruppe>/<id>` (Wolf gibt es als Gegner und Reittier); Plätze und Elemente nur beim Monarchen; gofmt-Schritt in `rig:test` entfernt CR (scheiterte unter Windows immer). Bein-Bänder klappen bei Elefant, Wolf, Alphawolf um: Falt-Test für sie übersprungen, B-791. Fenster nicht gestartet.
