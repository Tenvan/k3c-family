# WZ32 · DEV · Balancing-Seite: feste Fläche, eine Liste, Lauf-Editor mit Varianten-Tabelle

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-658, B-659, B-660, B-661, B-662
- **Start-Commit:** ad01a5b6
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat (umfasst B-658 bis B-662 in Revision 1)

## Ausgangslage

Die Balancing-Seite der Workbench (WZ28) scrollt als Ganzes (7.059 px bei 1.000 px Breite), bricht wegen einer Container Query auf sich selbst nie um, führt Läufe und Berichte in zwei Listen, zeigt Berichte nur als `simtest-run-N` und nimmt Varianten als Freitext. Befund und Mock: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Mock-Quellen der Boards im Scratchpad der Planungs-Session, Details in B-658 bis B-662.

## Ziel

Die Balancing-Seite ist in jeder Fensterbreite bedienbar: feste Fläche mit Liste und Detail, Berichte erkennbar und gesammelt verwaltbar, ein Lauf entsteht im Lauf-Editor mit Varianten-Tabelle und Daten-Browser.

Am Ende sichtbar: Workbench im Browser-Pane mit `?mock=1` bei 1.440, 960 und 600 px wie im Mock; Vitest und Go-Tests grün.

## Beteiligte und Zielgruppen

Ralf beim Balancing (entscheidet, nimmt ab 🧑); Claude setzt um.

## Anforderungen

- B-658 › Anforderungen
- B-659 › Anforderungen
- B-660 › Anforderungen
- B-661 › Anforderungen
- B-662 › Anforderungen
- Offene Fragen der Tickets nach den Defaults von Claude entschieden (Freigabe 2026-10-29).

## Nicht-Ziele

- Lauf-IDs über Neustarts (B-616).
- Änderungen an Balancing-Kennzahlen, Zielen oder `data/`.
- Andere Seiten der Workbench, außer sie haben denselben Container-Fehler (B-658).

## Regeln und Einschränkungen

- Arbeit im ausgecheckten Branch, nur Commits (Wunsch 🧑, kein Sprint-Branch, kein Push, kein PR).
- Atomic Design über `@webkit` (`tests/atomicDesign.test.ts`); Spec der Workbench-Seiten (`docs/standards/workbench-seiten.md`).
- Format der Varianten bleibt gleich mit Go (`simrun/spec.go`).
- Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Siehe B-659 bis B-662 › Beispiele.

## Ausnahme- und Fehlerfälle

Siehe Tickets; zusätzlich: ältere Berichte ohne Auftrag bleiben lesbar und löschbar.

## Akzeptanzkriterien

- **AC-01** Bericht liefert Auftrag, Label, Lauf-ID und getroffene Ziele (B-662/AC-01).
- **AC-02** Sammel-Bewerten, Sammel-Löschen und Baseline aus der Oberfläche (B-662/AC-02, AC-03).
- **AC-03** Ereignisse statt Abfrage, Werte aller Dateien, Option „je Variante“ (B-662/AC-04, AC-05).
- **AC-04** Umbruch unter 1.100 px und feste Fläche mit eigenen Scroll-Bereichen (B-658/AC-01, AC-02, B-659/AC-01 bis AC-03).
- **AC-05** Eine Liste mit Suche, Chips und Mehrfachauswahl, Vergleich von 2 bis 4 Berichten (B-660/AC-01 bis AC-03).
- **AC-06** Detail mit fünf Reitern, Erneut starten, Als Variante übernehmen (B-660/AC-04, AC-05).
- **AC-07** Lauf-Editor mit Ausgangspunkt, Abschnitten und Fußleiste (B-661/AC-01, AC-06).
- **AC-08** Varianten-Tabelle mit Prüfung, Bereich als Abstimmung, Textmodus verlustfrei (B-661/AC-02 bis AC-04).
- **AC-09** Daten-Browser mit Suche über alle Dateien (B-661/AC-05).
- **AC-10** `task check` und `task check:go` grün.

## Offene Fragen

keine (Defaults der Tickets gelten, Freigabe 2026-10-29).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ32.1 | `WZ32.1-go-seite-berichte.md` | Umsetzung | autonom | fertig |
| WZ32.2 | `WZ32.2-seitenrahmen.md` | Umsetzung | autonom | fertig |
| WZ32.3 | `WZ32.3-liste-und-detail.md` | Umsetzung | autonom | offen |
| WZ32.4 | `WZ32.4-lauf-editor.md` | Umsetzung | autonom | offen |
| WZ32.5 | `WZ32.5-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
