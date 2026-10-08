# BR2.1 · Vorbereitung: Korridore prüfen, Kennzahlen je Grad messen, Wertänderungen vorschlagen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** br2/1-vorbereitung-messung
- **Abhängig von:** –
- **Tickets:** B-156
- **Kriterien:** AC-01, AC-02

## Ziel

Die Zielkorridore für Kampf und Bosse stehen als Zahlen in `docs/rules/`, die Messung je Schwierigkeitsgrad vor der Runde liegt vor, und Wertänderungen sind begründet **vorgeschlagen** (noch nicht beschlossen, noch nicht in `data/`).

## Kontext

- **Voraussetzung (Spec):** K1 bis K5 sind umgesetzt (Gegner-, Wellen- und Boss-Werte sind Startwerte). Fehlt eines davon, Session `blockiert` und Ticket, nicht raten. BR2 läuft nach BR1 (B-155); dessen Wirtschaftswerte gelten als Stand.
- **Zielkorridore:** `docs/rules/zielkorridore.md` § 1–3, von 🧑 am 2026-10-03 bestätigt (Q02). Für Kampf und Bosse relevant: „Burg hält Nacht 1–5“ (Normal und je Grad Dev, Leicht, Hart, Ultra), Zerstörte Gebäude je Welle, Gegner einer Welle besiegt bis Tagesanbruch, Abstand der Wellen unter Tage, Miniboss Wald, Miniboss Höhle und Mine, Endboss, Burg hält Vollmond und Blutmond; dazu die Messgrößen aus § 4 („Fehlende Messgrößen“: Boss-Kampfdauer Median 60–180 s, Verluste je Welle, Anteil des Monarchen am Schaden), soweit sie bis dahin eine Zeile bekommen haben. Welche Zeilen zu Kampf und Bosse zählen, legt die Session anhand der Quellenspalte (`gegner.md`, `bosse.md`, `stufen.md`, `wirtschaft.md` § 4) fest und nennt sie im Ergebnis. Zahlen der Korridore werden **nicht** geändert (Änderung nur durch 🧑).
- **Werte, um die es geht:** `data/enemies.json`, `data/waves.json`, `data/difficulty.json` (Grade `dev`, `easy`, `normal`, `hard`, `ultra` mit Faktoren auf Wellengröße, Gegner-HP und -Schaden). Begründungen gehören nach `docs/rules/gegner.md` oder `docs/rules/bosse.md` (B-156 › Anforderungen). Boss-Werte liegen dort, wo K2 sie anlegt (vor dem Start im Code prüfen); sind sie nicht in den drei Dateien, ergänzt die Session die Liste der erlaubten Daten-Dateien im Ergebnis, nicht stillschweigend.
- **Messung je Grad:** mit dem Balancing-Tester, soweit verfügbar (`task balance`, entsteht in BAL2, Bot-Profile aus BAL1/BAL3, Grad-Kurven aus BAL3, 100 feste Seeds), sonst Läufe über die Go-Tests oder `sim_run` in k3c-dev (Spalte „Messbar mit“ in `zielkorridore.md`). Boss- und Event-Kennzahlen sind erst mit K2 und K3 messbar. Ergebnisse je Kennzahl und Grad **vor** jeder Änderung festhalten (Befehl, Datum, Commit); nicht messbare Kennzahlen als Liste mit Grund.
- **Regel Wertänderung:** Jede Änderung in `data/` nennt später im Commit die betroffenen Kennzahlen mit Wert vor und nach der Änderung (B-156/AC-02) und löst `task golden:update` nach `docs/arbeitsweise.md` › „Golden aktualisieren“ aus. Diese Session ändert **keine** Werte.
- **Ablage der Messung und Vorschläge:** Abschnitt „Balancing-Runde Kampf und Bosse (BR2)“ am Ende von `docs/rules/zielkorridore.md` (Tabelle je Kennzahl und Grad: Korridor, Messwert, Pass/Fail vorläufig, Vorschlag). Alternativ eigene Datei im selben Ordner; Wahl im Ergebnis nennen.

## Erlaubte Dateien

- `docs/rules/zielkorridore.md` (nur der Abschnitt dieser Runde, Zahlen der Korridore bleiben unverändert)
- `docs/rules/gegner.md`, `docs/rules/bosse.md` (nur Begründung und „Offen und Annahmen“)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Wertänderungen in `data/*.json` (BR2.3), Spieleabend (BR2.2), Wirtschaft (BR1), neue Mechaniken, Release (RL1), Änderung der Korridor-Zahlen.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Voraussetzungen prüfen (K1 bis K5 erledigt, Tester verfügbar).
2. Korridore für Kampf und Bosse aus `zielkorridore.md` heraussuchen und im Abschnitt dieser Runde als Zahlen auflisten (AC-01).
3. Kennzahlen je Grad messen, Ergebnisse mit Befehl und Datum festhalten.
4. Je Kennzahl außerhalb des Korridors eine Wertänderung vorschlagen (Datei, Pfad, alter und neuer Wert, betroffene Kennzahlen, Begründung).
5. `task check`. Ergebnis mit Messtabelle je Grad, Vorschlagsliste, Liste „nicht messbar“, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: `docs/rules/` nennt die Zielkorridore für Kampf und Bosse als Zahlen je Kennzahl (Datei und Abschnitt im Ergebnis).
- [ ] AC-02: Eine Vorschlagsliste nennt je Wertänderung die betroffenen Kennzahlen (vor und nach) und die Begründung; `data/` ist unverändert (`git status` zeigt keine Änderung in `data/`).
- [ ] Die Messung vor der Runde liegt je Grad mit Befehl, Datum und Commit vor.

## Prüfen

```bash
task check
```

## Ergebnis

–
