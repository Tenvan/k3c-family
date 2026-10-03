# F1 · REG · Zielkorridore und Bedienungsregeln

- **Status:** aktiv
- **Domäne:** REG
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-134, B-135, B-136, B-144, B-145
- **Start-Commit:** 54c1657
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-134, B-135, B-136, B-144, B-145 und B-152/AC-01

## Ausgangslage

Die Regelwerke R1 bis R4 sind beschlossen (`docs/rules/*.md`), enthalten aber Zielkorridore nur verstreut. Für Pause, Schriftgröße am TV, Verbindungsverlust und Sprache gibt es keine Regel; die Aktion `pause` existiert ohne Funktion. Die Entscheidungen Q01 bis Q05 und Q23 hat 🧑 am 2026-10-03 getroffen (`docs/fragenkatalog.md` › Beschlüsse); der Sprint schreibt sie als Regeln nieder und lässt 🧑 die Zahlen bestätigen.

## Ziel

Jede Kennzahl hat einen Zielkorridor als Zahl, und Pause, Schriftgröße, Verbindungsverlust und Sprache sind als Regel mit Zahl beschlossen. Am Ende sichtbar: `docs/rules/zielkorridore.md` und `docs/rules/bedienung.md` mit Datum der Bestätigung durch 🧑.

## Beteiligte und Zielgruppen

🧑 hat entschieden und bestätigt die Zahlen im Workshop F1.4; ein Agent schreibt die Beschlüsse nieder und bereitet Vorschläge aus `docs/rules/*.md` vor. Danach arbeiten BAL2, S4, S5 und SP-Sprints mit den Zahlen.

## Anforderungen

B-134, B-135, B-136, B-144 und B-145 › Anforderungen. Sprint-eigen: Jede Entscheidung steht als Regel in `docs/rules/` oder `docs/game-design.md` mit Zahl, nicht nur in einem Ticket oder im Chat.

## Nicht-Ziele

Umsetzung in Code (S5 Pause-Szene, S4 Layouts, B-157 Prüfung), Änderung von Werten in `data/*.json` (B-155, B-156), Feedback-Ereignisliste (Q08, F3).

## Regeln und Einschränkungen

Domäne REG (`docs/rules/`, `docs/game-design.md`, `docs/playtests/`). Beschlüsse gehören 🧑: Ein Agent schreibt keine Zahl als beschlossen, die 🧑 nicht bestätigt hat; Vorschläge sind als „Vorschlag“ markiert. B bleibt unbelegt, View + Menu reserviert (`CLAUDE.md`).

## Beispiele

Workshop F1.1: Agent legt „Überleben Nacht 3 ≥ 80 %“ vor, 🧑 ändert auf ≥ 75 % → die Zahl 75 steht mit Datum in `zielkorridore.md`.

## Ausnahme- und Fehlerfälle

🧑 entscheidet eine Frage nicht → das Ticket bleibt offen, die Regel steht als „offen“ mit Vorschlag, die Sessions danach laufen weiter; der Sprint-Abschluss vermerkt die offene Frage.

## Akzeptanzkriterien

- **AC-01** `docs/rules/zielkorridore.md` existiert mit Kennzahlen, Szenario, Unter- und Obergrenze und Quelle (B-134/AC-01, B-134/AC-02).
- **AC-02** 🧑 hat die Zielkorridore bestätigt, Datum im Kopf der Datei; `docs/game-design.md` verweist auf die Datei (B-134/AC-03, B-134/AC-04).
- **AC-03** Abschnitt „Pause“ in `docs/rules/bedienung.md` beantwortet Q01 mit Zahl, `docs/game-design.md` verweist darauf (B-135/AC-01, B-135/AC-02).
- **AC-04** Abschnitt „Schriftgröße“ nennt je Layout 1 bis 4 Spieler eine Mindestgröße in px und das Prüfverfahren (B-136/AC-01, B-136/AC-02).
- **AC-05** Abschnitt „Verbindung“ nennt Reservierungszeit, Verhalten des Monarchen und Latenz-Ziel in ms (B-144/AC-01, B-144/AC-02).
- **AC-06** Abschnitt „Sprache“ hält die Entscheidung mit Datum fest (B-145/AC-01).
- **AC-07** Der Abschnitt „Reittier“ in `docs/rules/monarch.md` nennt Standard-Reittier und Faktoren (B-152/AC-01).
- **AC-08** `task check` ist grün, und die Tickets B-134, B-135, B-136, B-144, B-145 sind nach `docs/backlog/archiv/` verschoben (Status `erledigt`), soweit ihre Kriterien erfüllt sind.
- **AC-09** 🧑 hat die Vorschläge in `bedienung.md` und im Abschnitt Reittier bestätigt, Datum im Kopf der Dateien, oder offene Punkte sind als Ticket geführt (B-135/AC-01, B-136/AC-01, B-144/AC-01, B-152/AC-01).

## Offene Fragen

Entschieden am 2026-10-03: Q01 Pause, Q03 Schriftgröße (≥ 28 px Vollbild, ≥ 24 px Viertel), Q04 Verbindungsverlust (unverwundbar und ausgeblendet bis 60 s, Latenz ≤ 100 ms), Q05 Deutsch und Englisch (B-172), Q23 Standard-Reittier von Anfang an. Offen für F1.4: Zahlen aus Q02 (Zielkorridore), Höchstdauer der Pause, Schrift für 2 Spieler und Nebeninfo, p95 der Latenz, Tierart und Faktoren des Reittiers. Blockiert die Freigabe der Spec nicht.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F1.1 | `F1.1-bedienung-regeln.md` | Umsetzung | autonom | fertig |
| F1.2 | `F1.2-zielkorridore-vorschlag.md` | Umsetzung | autonom | fertig |
| F1.3 | `F1.3-reittier-regel.md` | Umsetzung | autonom | offen |
| F1.4 | `F1.4-workshop-bestaetigung.md` | Workshop | Mensch | offen |
| F1.5 | `F1.5-abschluss.md` | Umsetzung | autonom | offen |

## Abnahme

–
