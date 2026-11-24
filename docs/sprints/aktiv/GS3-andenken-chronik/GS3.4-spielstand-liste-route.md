# GS3.4 · Ablage: Spielstand-Liste liefert Route und Chronik (B-978)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.1
- **Tickets:** B-978
- **Kriterien:** AC-04

## Ziel

Die Liste der Spielstände liefert je ladbarem Stand Route, aktuelle Insel, abgeschlossene Inseln und Chronik.

## Kontext

Spec: `docs/backlog/B-978-spielstand-auswahl-seekarte.md` (R1, R4, AC-01). Code: `engine/store/saves_list.go` (`Saves.List`, `SaveInfo` mit Tag, Tiefen, Welt, `Loadable`, `Problem`), Spielstand-Felder aus `engine/sim/island_save.go` (`world`, `islandId`, `completed`, `chronicle` aus GS3.1), Welt-Dateien über die Welten-Ablage des Raums (`engine/room/worlds.go`). Die Liste liest nur Stand und Welt-Datei; sie baut keine Insel und rechnet nichts. Unbekannte oder ungültige Welt → Stand nicht ladbar wie heute, Felder leer. `engine/` importiert nichts aus `tools/`.

## Erlaubte Dateien

- `engine/store/**`

## Nicht-Ziele

Menü und Zeichnen (GS3.5), Änderung am Spielstand-Format, Laden-Logik.

## Schritte

1. Test zuerst: Stand mit `completed` und `chronicle` → Route, aktuelle Insel, Abgeschlossene, Chronik; nicht ladbarer Stand → leer, `Loadable` false.
2. `SaveInfo` erweitern, in `List` füllen (Welt einmal je Stand lesen).
3. Review-Agent (Domäne SRV), `check_run task:check:go`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-04: Liste liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik; nicht ladbare Stände bleiben leer (B-978/AC-01).

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
