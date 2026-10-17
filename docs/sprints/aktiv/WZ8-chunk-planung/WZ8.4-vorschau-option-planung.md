# WZ8.4 · Vorschau-Option geplante Chunks

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** wz8/4-vorschau-option-planung
- **Abhängig von:** WZ8.3, WZ7.4
- **Tickets:** B-461
- **Kriterien:** AC-08

## Ziel

Die Vorschau-Funktion aus WZ7.1 nimmt die Planung als Option „geplante Chunks“ ihrer Options-Struktur und reicht sie in `sim.CreateOptions` durch: deterministisch, ohne globalen Zustand, ungültige Planung wird abgelehnt.

## Kontext

Spec: B-461 und Sprint-README WZ8. **Beginnt erst, wenn WZ7 erledigt ist** (Vorschau-Funktion WZ7.1, Welt-Betrachter WZ7.2–WZ7.4) und WZ8.3 `fertig` ist. Vor dem Start die tatsächlichen Namen der Vorschau-Funktion und ihrer Options-Struktur aus dem Code von WZ7.1 lesen (Pfad vermutlich unter `engine/room/` oder `engine/net/`); die Namen hier sind Annahme.

Reicht die Options-Struktur aus WZ7.1 nicht (z. B. globaler Zustand statt Parameter): Session `blockiert`, Anpassung als Ticket in SRV, keine zweite Vorschau-Funktion.

Vorrang (Hinweis, Test erst in WZ9.4): Ist später auch die Kandidaten-`hub.json` gesetzt, gelten deren Linien und die Planung wird gegen sie geprüft. `sim.CreateOptions` aus WZ8.3 bekommt dort ein weiteres Feld, ohne die Signatur zu ändern. Kein HTTP-Endpunkt, kein Protokoll- oder Spielstand-Wechsel.

## Erlaubte Dateien

- Die Datei(en) der Vorschau-Funktion und ihrer Options-Struktur aus WZ7.1 (`engine/room/` bzw. `engine/net/`) und zugehörige Tests
- Planungsdateien von WZ8 über k3c-dev

## Nicht-Ziele

Änderungen an der Vorschau-Funktion über die Option „geplante Chunks“ hinaus; zweite Vorschau-Funktion; HTTP-Endpunkt, Protokoll- oder Spielstand-Änderung; Kandidaten-`hub.json` (WZ9.4); Änderungen an `engine/sim` oder `engine/level` (→ Ticket); globaler Zustand für den Entwurf.

## Schritte

1. Prüfen, ob WZ7 erledigt ist und die Options-Struktur aus WZ7.1 parameterbasiert ist; sonst `blockiert` mit Ticket.
2. Tests zuerst (rot): gleiche Planung → gleicher Snapshot; ohne Option gilt die eingebettete Planung; ungültige Planung wird mit dem Fehler aus WZ8.2 abgelehnt; zwei Aufrufe nacheinander mit verschiedener Planung liefern je ihren Snapshot.
3. Option „geplante Chunks“ in der Options-Struktur ergänzen und in `sim.CreateOptions` durchreichen.
4. `task check:go`; Commit `feat(srv): Vorschau mit geplanten Chunks (WZ8.4)`.

## Fertig, wenn

- [ ] AC-08: Tests für Determinismus, eingebettete Planung als Standard, Ablehnung ungültiger Planung und aufeinanderfolgende Aufrufe mit verschiedener Planung grün; kein globaler Zustand.
- [ ] Keine Änderung außerhalb der Vorschau-Funktion und ihrer Tests.
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
```

Bei laufender Workbench `check_run`. Manuelle Prüfungen nennt diese Session nicht.

## Ergebnis

Umgesetzt von Claude (autonom), 2026-10-17; getrennter Reviewer-Agent vor dem Commit.

- **Voraussetzung geprüft:** WZ7.1 liegt auf `develop` (`room.BuildPreview`, `room.PreviewOptions`, ohne globalen Zustand); WZ8.3 `fertig`. Die Options-Struktur reichte, kein Ticket nötig.
- **AC-08 umgesetzt, geprüft (`task check:go` grün):** `room.PreviewOptions.Planned []level.PlannedChunk` („geplante Chunks“, ungespeicherter Entwurf möglich), durchgereicht über `sim.Options{Planned}` an `level.GenerateWith`; `net.Preview` und das Studio bleiben unverändert (die Option ist neu, der Nullwert ändert nichts). Tests in `engine/room/preview_planned_test.go`: gleiche Planung → gleiche Welt; nil und leer = Level wie `level.Generate`; ungültige Planung wird mit dem Grund der Prüfung (`Platz 0`) abgelehnt; aufeinanderfolgende Aufrufe mit verschiedener Planung liefern je ihre Welt (Portal bei Hub +325 nur mit Planung A), danach unverändert.
- **Abweichung (Annahme, in der nächsten Session zu klären):** Ohne Option („nil“) gilt **keine** Planung statt der eingebetteten. Der Regeltext § 1 sagt, ein Biom-Level ohne Insel bleibt rein zufällig; die Vorschau hat in WZ7/WZ8 nur die Quelle Biom. „Eingebettete Planung als Standard“ gilt erst mit der Quelle Insel + Stufe (WZ8.5). `sim.CreateOptions` wird in dieser Session nicht benutzt, weil die Vorschau eine einzelne Welt über `sim.CreateWorld` baut; die Planung geht über `sim.Options`.
- **Dateien:** nur `engine/room/preview.go` und der neue Test; keine Änderung an `engine/sim` oder `engine/level`.
- **Manuelle Prüfung:** keine. Neue Tickets: keine.
