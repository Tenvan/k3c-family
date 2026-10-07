# BR1.1 · Vorbereitung: Korridore prüfen, Kennzahlen messen, Wertänderungen vorschlagen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** br1/1-vorbereitung-messung
- **Abhängig von:** –
- **Tickets:** B-155, B-015
- **Kriterien:** AC-01, AC-02

## Ziel

Die Zielkorridore der Wirtschaft stehen als Zahlen in `docs/rules/`, die Messung vor der Runde liegt vor, und Wertänderungen sind begründet **vorgeschlagen** (noch nicht beschlossen, noch nicht in `data/`).

## Kontext

- **Voraussetzung (Spec):** W1 bis W6 sind umgesetzt (Wirtschaftswerte sind Startwerte); Zielkorridore aus F1 liegen vor. Fehlt eines davon, Session `blockiert` und Ticket, nicht raten.
- **Zielkorridore:** `docs/rules/zielkorridore.md` § 1–3, von 🧑 am 2026-10-03 bestätigt (Q02). Für die Wirtschaft relevant sind die Zeilen zu Gold am Morgen je Spieler, erste Mauer vor Ende der hellen Phase von Tag 1, erster Turm vor Ende von Tag 2, Holz am Tagesbeginn, Hub-Stufe 2 und 3, Kämpfer je Hub, Wellen unter Tage und „Burg hält Nacht 1–5“ je Grad. Welche Zeilen genau zur Wirtschaft zählen, legt die Session anhand der Quellenspalte (`wirtschaft.md`, `materialien-gebaeude.md`, `buerger.md`) fest und nennt sie im Ergebnis. Zahlen werden **nicht** geändert; ein Korridor, der nach Messung ungeeignet scheint, wird Ticket oder Frage an 🧑 (Revision der Datei erhöht nur 🧑).
- **Werte, um die es geht:** `data/hub.json`, `data/buildings.json`, `data/economy.json`, `data/troops.json`; Begründungen gehören nach `docs/rules/wirtschaft.md` und `docs/rules/materialien-gebaeude.md` (B-155 › Anforderungen; `buerger.md` für Truppen). B-015: HP und Kosten jedes Gebäudes in `buildings.json` sind zu begründen (Startwerte in `materialien-gebaeude.md` § 3, Mauer und Turm Stufen 1 bis 5, Hub-Ausbau, Gebäude-Kosten).
- **Messung:** mit dem Balancing-Tester, soweit verfügbar (`task balance`, entsteht in BAL2, Bericht `reports/balance-*.md`; Bot-Profile aus BAL1 und BAL3, 100 feste Seeds), sonst Läufe über die Go-Tests oder `sim_run` in k3c-dev (siehe Spalte „Messbar mit“ in `zielkorridore.md`). Ergebnisse je Kennzahl **vor** jeder Änderung festhalten (Medianwert und Anteil; Befehl, Datum, Commit). Nicht messbare Kennzahlen als Liste mit Grund führen.
- **Aktueller Stand:** Das Paket und `task balance` entstehen erst in BAL1 und BAL2 (`docs/sprints/geplant/BAL1-*`, `BAL2-*`); vor dem Start im Code prüfen, ob sie liegen.
- **Regel Wertänderung:** Jede Änderung in `data/` nennt im späteren Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung (B-155/AC-02) und löst `task golden:update` nach dem Ablauf in `docs/arbeitsweise.md` › „Golden aktualisieren“ aus. Diese Session ändert **keine** Werte, sie schlägt nur vor.
- **Ablage der Messung und Vorschläge:** Abschnitt „Balancing-Runde Wirtschaft (BR1)“ am Ende von `docs/rules/zielkorridore.md` (Tabelle je Kennzahl: Korridor, Messwert, Pass/Fail vorläufig, Vorschlag). Alternativ eigene Datei im selben Ordner; Wahl im Ergebnis nennen.

## Erlaubte Dateien

- `docs/rules/zielkorridore.md` (nur der Abschnitt dieser Runde, Zahlen der Korridore bleiben unverändert)
- `docs/rules/wirtschaft.md`, `docs/rules/materialien-gebaeude.md`, `docs/rules/buerger.md` (nur Begründung und „Offen und Annahmen“)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Wertänderungen in `data/*.json` (BR1.3), Spieleabend (BR1.2), Kampf und Bosse (BR2), neue Mechaniken, Ausbau des Testers (BAL1 bis BAL3), Änderung der Korridor-Zahlen.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Voraussetzungen prüfen (W1 bis W6 erledigt, Tester verfügbar).
2. Wirtschafts-Korridore aus `zielkorridore.md` heraussuchen und im Abschnitt dieser Runde als Zahlen auflisten (AC-01).
3. Kennzahlen messen (Tester oder Go-Tests), Ergebnisse mit Befehl und Datum festhalten.
4. Je Kennzahl außerhalb des Korridors eine Wertänderung vorschlagen (Datei, Pfad, alter und neuer Wert, betroffene Kennzahlen, Begründung) und je Gebäude die Begründung für HP und Kosten aus B-015 vorbereiten.
5. `task check`. Ergebnis mit Messtabelle, Vorschlagsliste, Liste „nicht messbar“, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: `docs/rules/` nennt die Zielkorridore der Wirtschaft als Zahlen je Kennzahl (Datei und Abschnitt im Ergebnis).
- [ ] AC-02: Eine Vorschlagsliste nennt je Wertänderung die betroffenen Kennzahlen (vor und nach) und die Begründung; `data/` ist unverändert (`git status` zeigt keine Änderung in `data/`).
- [ ] Die Messung vor der Runde ist mit Befehl, Datum und Commit festgehalten.

## Prüfen

```bash
task check
```

## Ergebnis

–
