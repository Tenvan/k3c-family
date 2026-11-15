# GS3.5 · Menü: Seekarte als Vorschau in der Spielstand-Auswahl (B-978)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.3, GS3.4
- **Tickets:** B-978
- **Kriterien:** AC-05

## Ziel

Die Spielstand-Auswahl zeigt für den markierten Stand die Seekarte mit Trophäen, Schiff und den Zahlen der zuletzt abgeschlossenen Insel.

## Kontext

Spec: `docs/backlog/B-978-spielstand-auswahl-seekarte.md` (R2–R6, AC-02, AC-03). Menü: `native/k3c-game/menu/saves.go` (`savesScreen`, `refresh`, `fill`, `saveRow`), Tests `saves_test.go`; Daten `view.SaveInfo` (`native/k3c-game/view/session.go`), gefüllt in `native/k3c-game/local/saves.go` aus `engine/store` (erweitert in GS3.4). Zeichnen des Dioramas aus GS2/GS3.3 (`native/k3c-game/hud/drawsea_paper.go`) wiederverwenden: gemeinsame Zeichenfunktion ohne Zeiger und Eingabe; Menü-Layout `menu/layout.go`, Weltbilder `menu/art.go`. B geht zurück, Laden und Löschen bleiben wie heute. Route nur beim Aufbau der Seite lesen, nicht je Frame.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- `native/k3c-game/view/**`
- `native/k3c-game/local/**`
- `native/k3c-game/hud/**`

## Nicht-Ziele

Eigener Menüpunkt „Chronik“, Blättern durch Zeilen, Änderungen an Laden und Löschen.

## Schritte

1. Tests zuerst: markierter Stand bestimmt die Vorschau; Auf/Ab wechselt; nicht ladbar → keine Vorschau; bestehende `saves_test.go` grün; ohne Assets Formen.
2. `view.SaveInfo` und `local/saves.go` um Route, Abgeschlossene, Chronik erweitern.
3. Zeichenfunktion des Dioramas für das Menü nutzbar machen (ohne Zeiger).
4. Vorschau rechts neben der Liste, darunter Abschlusszeile und Zahlen der zuletzt abgeschlossenen Insel.
5. Fachprüfung `k3c-hud`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-05: Vorschau des markierten Stands, Laden, Löschen und B unverändert (B-978/AC-02, AC-03).
- [ ] B-978/AC-04 (TV) bleibt offen bei 🧑.

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
