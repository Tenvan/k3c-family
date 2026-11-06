# MG2.2 · Fabelwesen und Hirsch: einhorn, pegasus, hirsch

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg2/2-fabelwesen-hirsch
- **Abhängig von:** MG2.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`einhorn`, `pegasus` und `hirsch` sind als `vierbeiner`-Specs mit `schritt`, `trab`, `galopp` über die Asset-MCP-Tools angelegt und gerendert.

## Kontext

- Quelle: `data/sprites.json` › `mounts` (`unicorn`, `pegasus`, `stag`).
- Grundform wie `pferd`; Zusatzteile als eigene Teile mit `eltern` (Horn am `hals`, Flügel am `rumpf-vorn`, Geweih am `hals`); der Vierbeiner-Renderer zeichnet zusätzliche Teile wie `sattel` mit.
- Farben laut D-MG2-01 (einhorn: `hermelin`, Mähne `purpur`, Horn `gold`; pegasus: `hermelin`, Mähne und Flügel-Schatten `stoff`; hirsch: `fell-fuchs`, Geweih `holz`, schlankere Beine über `bein`).
- Werkzeuge mit `checkout`; nach Änderungen `task gen:paper` (B-707).

## Erlaubte Dateien

- `public/grafik/reittiere/**`
- `docs/sprints/*/MG2-papier-reittiere/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (Zeilen D-MG2-xx)

## Nicht-Ziele

Kein Code, kein `assets_style_set`, keine Flug-Animation (nur Gangarten des Rigs).

## Schritte

1. Je Asset `assets_create`, `assets_set` mit Teilen vom Pferd plus Zusatzteil, `assets_preview` je Animation Frame 0 und 4.
2. Zusatzteile mit `assets_preview` (Teil-Boxen) auf Lage prüfen, nachbessern.
3. `task gen:paper`, `assets_review`, `assets_list {gruppe: reittiere}`.
4. `task check:diff`, Commit, Push nach `sprint/mg2`.

## Fertig, wenn

- [ ] AC-01: einhorn, pegasus, hirsch gültig.
- [ ] AC-02: `task gen:paper` grün, Golden eingetragen.
- [ ] AC-03: jede Animation per `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne `ungueltig`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
