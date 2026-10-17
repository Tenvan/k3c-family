# WZ9.2 · Gebäude-Planer: Matrix, Abweichung, Live-Vorschau

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz9/2-planer-matrix
- **Abhängig von:** WZ9.1, WZ15.2, WZ6, WZ7.3
- **Tickets:** B-462
- **Kriterien:** AC-01, AC-03, AC-04, AC-08

## Ziel

Im Asset-Studio zeigt die Seite Gebäude-Planer die Matrix Hub-Stufe × Bauplatz mit Platz und Tiefe aus `data/hub.json`, die Abweichung zur erzeugten Regeltabelle und die Live-Vorschau des Planer-Stands (Kandidat) in der gewählten Hub-Stufe.

## Kontext

- Voraussetzungen: Asset-Studio (WZ6, erledigt), Live-Vorschau (WZ7.3), Snapshot-Option (WZ9.1), Generator-Paket `tools/regeltabelle/tabelle` (WZ15.2). Fehlt WZ7.3, WZ9.1 oder WZ15.2, Session `blockiert`.
- Das Studio ist ein eigenes Go-Modul mit Wails-Oberfläche (`tools/k3c-studio/`: `app.go`, `frontend/`). Die Abweichung (Ist-Block der Tabelle gegen soll-Block aus der JSON) rechnet das Go-Backend mit der Funktion aus `tools/regeltabelle/tabelle`. Geschrieben wird der Block erst in WZ9.3.
- Matrix: Zeilen Bauplatz (`sites`, `islandSites`, Mauerlinien, Tor, Händler), Spalten Hub-Stufen aus `levels` (beliebig viele, keine feste 5). Begriffe laut Glossar („Gebäude-Planer“, „Live-Vorschau“, „Hub-Stufe“).
- Firewall-Regel des Studios (B-459) und Dateiregeln (≤ 400 Zeilen, Funktion ≤ 60) beachten.

## Erlaubte Dateien

- `tools/k3c-studio/` (Backend, `frontend/`, Tests)
- `tools/regeltabelle/tabelle/` nur lesend importieren (go.mod `replace` im Studio-Modul ist erlaubt)
- Kein neuer Glossar-Begriff (WZ6.1 trägt „Gebäude-Planer“ ein); fehlt einer, Ticket anlegen

## Nicht-Ziele

Kein Bearbeiten und kein Schreiben von `data/hub.json` oder der Regeltabelle (WZ9.3), keine Änderung an Werten oder Kosten, keine manuelle Browser- oder Studio-Abnahme (WZ9.5), keine neue Abhängigkeit.

## Schritte

1. Stand von WZ6, WZ7.3, WZ15.2 und WZ9.1 prüfen; sonst `blockiert`.
2. Backend-Methode: `hub.json` laden und als Matrix-Modell liefern (Stufen, Plätze, Tiefen, Linien, Tor, Händler).
3. Backend-Methode: Abweichung zur Regeltabelle über `tabelle` berechnen (Ist gegen Soll).
4. Seite Gebäude-Planer im Frontend: Matrix, Abweichungsanzeige, Auswahl der Hub-Stufe.
5. Live-Vorschau einbinden (aus WZ7.3), speist sich aus der Snapshot-Option (Kandidat) und der gewählten Hub-Stufe; Anzeige auch ungespeichert.
6. Go- und Frontend-Tests; Test mit der 7-Stufen-Fixture zeigt 7 Spalten.
7. `task check:dev` bzw. `check_run`.

## Fertig, wenn

- [ ] AC-01: Matrix Hub-Stufe × Bauplatz mit Platz und Tiefe (Test auf das Matrix-Modell).
- [ ] AC-03: Abweichung zur erzeugten Regeltabelle wird im Backend berechnet und angezeigt (Test mit absichtlich abweichender Tabelle).
- [ ] AC-04: Live-Vorschau folgt dem Planer-Stand und der gewählten Hub-Stufe (Test über die Snapshot-Option).
- [ ] AC-08: Fixture mit 7 Stufen zeigt 7 Spalten.
- [ ] Prüfläufe der Studio-Module grün.

## Prüfen

```bash
task check
task check:go
```

Unter k3c-dev: `check_run` mit `checkout` (inklusive Studio-Modul). Keine manuelle Prüfung; die Sichtprüfung gehört 🧑 (WZ9.5).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
