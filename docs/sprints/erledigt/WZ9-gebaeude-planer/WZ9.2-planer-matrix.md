# WZ9.2 · Gebäude-Planer: Matrix, Abweichung, Live-Vorschau

- **Status:** fertig
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

- Voraussetzungen: Asset-Studio (WZ6, erledigt), Live-Vorschau (WZ7.3), Snapshot-Option (WZ9.1), Generator-Paket `tools/ruletable/table` (WZ15.2). Fehlt WZ7.3, WZ9.1 oder WZ15.2, Session `blockiert`.
- Das Studio ist ein eigenes Go-Modul mit Wails-Oberfläche (`tools/k3c-studio/`: `app.go`, `frontend/`). Die Abweichung (Ist-Block der Tabelle gegen soll-Block aus der JSON) rechnet das Go-Backend mit der Funktion aus `tools/ruletable/table`. Geschrieben wird der Block erst in WZ9.3.
- Matrix: Zeilen Bauplatz (`sites`, `islandSites`, Mauerlinien, Tor, Händler), Spalten Hub-Stufen aus `levels` (beliebig viele, keine feste 5). Begriffe laut Glossar („Gebäude-Planer“, „Live-Vorschau“, „Hub-Stufe“).
- Firewall-Regel des Studios (B-459) und Dateiregeln (≤ 400 Zeilen, Funktion ≤ 60) beachten.

## Erlaubte Dateien

- `tools/k3c-studio/` (Backend, `frontend/`, Tests)
- `tools/ruletable/table/` nur lesend importieren (go.mod `replace` im Studio-Modul ist erlaubt)
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

AC-01 geprüft: neue Studio-Seite „Gebäude-Planer“ (Tab `planer`), Go-Backend `App.HubPlan()` (`app_planer.go`) liefert das Matrix-Modell: Zeilen `sites`, `islandSites`, je Linie Mauer/Turm und Tor, Händler Kaufen/Verkaufen; Spalten `len(levels)`; Zelle mit Platz und Tiefe (`offsetUnits`, `fromDepth`, `needsDeeper`, `onlyDepth`). Test gegen die heutige `data/hub.json` (Rüstkammer ab Stufe 4 usw.).
AC-03 geprüft: `App.HubRules(hubJSON)` (`app_planer_rules.go`) berechnet die Abweichung nur über `tabelle.Gegenprobe` (ist/soll, abweichende Zeilen, Task `rules:update`); Test mit absichtlich abweichender Tabelle auf einer Kopie, Regeltabelle des Checkouts bleibt byte-gleich.
AC-04 geprüft: `WorldPreviewArgs.HubCandidate` geht als `room.PreviewOptions.HubCandidate` an die Live-Vorschau (`LivePreview` mit optionaler Prop `planer`, Bühne wiederverwendet, ohne Prop unverändert): ohne Kandidat steht die Rüstkammer in Stufe 3 nicht, mit Kandidat schon, ungültiger Kandidat → Fehler; Hub-Stufe und Kandidat steuert die Seite.
AC-08 geprüft: `hub7.json` ergibt im Go-Test 7 Spalten, die Vorschau läuft mit Stufe 7 (8 nicht); Frontend-Test zeigt „Hub-Stufe 1…7“; im Code keine feste 5 (auch nicht im Mock, `hubLevelChoices().length`).
Prüfung: Studio-Vitest (16 Dateien, 129 Tests), `tsc --noEmit`, `vite build`, Studio `go vet`/`go test`/`golangci-lint`, `task:check`, `task:check:go`, `go:lint` grün; `data/hub.json`, Regeltabelle und `testdata/golden` unverändert.
Review (getrennter Agent, 0 CRITICAL/HIGH): behoben unbehandelte Promise-Ablehnungen (NoticeCard bei Fehler, 2 Tests), Kommentar `num`, Gebäude-Namen aus `site.<kind>` der Spieltexte, Kandidat in `ctx` der Vorschau (Auswahl wird zurückgesetzt), feste 5 und ungeschütztes `JSON.parse` im Mock, Fehlermeldung „nicht lesbar als hub.json“. Offen: Zeilen-Diff der Abweichung vergleicht nach Position (eine eingefügte Zeile markiert alle folgenden; Anzeige korrekt, nur schlecht lesbar) — bei Bedarf in WZ9.3; Matrix-Schlüssel `line@k`/`gate@k` passen nicht zu den Schaltern der Vorschau (`wall@x`); „Planer-Stand“ ist kein Glossar-Begriff (Kandidat genügt).
Abweichungen: Seed und Biom der Planer-Vorschau fest `k3c`/`forest`; Mauerlinie und Tor als eine Zeile je Linie mit ± (hub.json ist symmetrisch); Burg und Ausbaustufen von Mauer/Turm nicht in der Matrix; der Browser-Mock zeigt die 7-Stufen-Fixture und meldet gegen die 5-Zeilen-Regeltabelle immer eine Abweichung.
