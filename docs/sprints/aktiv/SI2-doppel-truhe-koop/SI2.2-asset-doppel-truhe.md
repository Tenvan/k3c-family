# SI2.2 · Assets Doppel-Truhe (B-998)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si2
- **Abhängig von:** SI2.1
- **Tickets:** B-998
- **Kriterien:** AC-02

## Ziel

Die Papierschnitt-Assets der Doppel-Truhe sind angelegt, zugeordnet und geprüft (B-998/AC-01).

## Kontext

Papierschnitt nach Entscheidung 005: Assets nur über die MCP-Tools `assets_*` (`assets_guide` zuerst lesen), nie Specs von Hand. Vorbild: `public/grafik/objekte/truhe` und `truhe-offen` (mit `assets_get` ansehen). Die Doppel-Truhe hat zwei Schlösser; benötigte Zustände: zu, linkes Schloss leuchtet, rechtes Schloss leuchtet, offen (Spec `B-998`). Zuordnung in `data/paper.json`. PNG und `manifest.json` sind nicht eingecheckt (B-737); Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742). Diese Session läuft vor der Engine (SI2.3); sie braucht nur die Spec.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `data/paper.json`
- `data/asset-prompts.json`

## Nicht-Ziele

Code im Spiel (SI2.4), Änderung an der normalen Truhe, Sounds.

## Schritte

1. `assets_guide` und `assets_style_get` lesen, `truhe` und `truhe-offen` ansehen.
2. `doppel-truhe` anlegen (Zustände zu, Schloss links leuchtet, Schloss rechts leuchtet) und `doppel-truhe-offen`, gleiche Größe und Fußpunkt wie die Truhe.
3. `data/paper.json`: Zuordnung.
4. `assets_preview`, Fachprüfung durch Agent `k3c-grafik`, schwere Befunde beheben.

## Fertig, wenn

- [ ] AC-02: Beide Assets existieren, `data/paper.json` ordnet sie zu, `k3c-grafik` ohne schweren Befund.
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
