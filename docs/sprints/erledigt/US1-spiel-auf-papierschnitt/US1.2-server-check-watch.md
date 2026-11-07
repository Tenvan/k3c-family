# US1.2 · Server prüft Gruppen-Indizes und ignoriert Specs im Watch

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us1/2-server-check-watch
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-02

## Ziel

`k3c-server -check-data` meldet fehlende Dateien aus einem Gruppen-Index; der Watch ignoriert `*.paper.json`.

## Kontext

Check: `cmd/k3c-server` (`-check-data`), Watch: `engine/watch`. Entscheidung 004, B-701 D-11.

## Erlaubte Dateien

- `cmd/k3c-server/`
- `engine/watch/`
- `engine/` (nur Prüfcode für Grafik)

## Nicht-Ziele

Client-Code.

## Schritte

1. Go-Test (rot) für fehlende Datei aus Index.
2. Prüfung umsetzen.
3. Go-Test (rot) für Watch und `*.paper.json`, umsetzen.
4. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-02: Go-Tests für Check und Watch grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02 geprüft: `TestCheckPaperFehlendeDatei` (fehlendes Sheet, fehlendes Manifest, kaputter Index → Fehler mit Pfad, `checkData` Exit 1 → alter Stand bleibt), `TestCheckPaperRepoStand` (eingecheckter Stand vollständig), `TestPaperSpecsSindKeineAenderung` (Watch ignoriert `*.paper.json`, PNG löst aus) grün.
- `task check:diff`: Go grün; ein Workbench-Frontend-Test (`CatalogPrompts`) rot unter Last, einzeln grün (B-704).
- Abweichung: Gruppen-Index wird auch ohne Dev-Mode geprüft; Gebäude-Stufen `stand.s<n>.png` stehen nicht im Manifest und bleiben ungeprüft.
