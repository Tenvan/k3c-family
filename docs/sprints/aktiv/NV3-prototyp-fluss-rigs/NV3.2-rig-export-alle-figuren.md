# NV3.2 · Rig-Export für alle Figuren mit Teile-Baum

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
