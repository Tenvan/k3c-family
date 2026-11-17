# SI4.2 · Assets Kamel und Gatter (B-1000)

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si4
- **Abhängig von:** SI4.1
- **Tickets:** B-1000
- **Kriterien:** AC-02

## Ziel

Die Papierschnitt-Assets Kamel und Gatter sind angelegt und geprüft; `data/paper.json` ordnet Kamel, Sengoku-Pferd und Gatter zu (B-1000/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen), nie Specs von Hand. Reittiere liegen in `public/grafik/reittiere/` (unter anderem `pferd`, `rappe`, `schimmel`, `hirsch`, `elefant`); der Monarch ist ein Rig mit Reittier, deshalb braucht das Kamel dieselben Posen und Ankerpunkte wie `pferd` (mit `assets_get` vergleichen). Das Sengoku-Pferd nutzt das vorhandene `rappe`. Zuordnung in `data/paper.json` (Reittiere dort unter dem Sprite-Schlüssel, heute `horse`); neue Schlüssel: `camel`, `sengokuHorse`. PNG und `manifest.json` sind nicht eingecheckt (B-737); Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742). Diese Session läuft vor der Engine (SI4.3); sie braucht nur die Spec `B-1000`.

## Erlaubte Dateien

- `public/grafik/reittiere/**`
- `public/grafik/objekte/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Rentier, Code im Spiel (SI4.4), Änderung am Monarchen-Rig, Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, `pferd` und `rappe` ansehen (Posen, Anker).
2. `kamel` in Gruppe `reittiere` anlegen, Posen und Anker wie `pferd`.
3. `gatter` in Gruppe `objekte` anlegen (Platz für ein stehendes Tier davor).
4. `data/paper.json`: `camel` auf `kamel`, `sengokuHorse` auf `rappe`, Gatter.
5. `assets_preview`, Fachprüfung durch Agent `k3c-grafik` (Rig-Passung), schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-02: Kamel und Gatter existieren, die drei Zuordnungen stehen in `data/paper.json`, `k3c-grafik` ohne schweren Befund.
- [ ] `check_run task:check:diff` grün.

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

–
