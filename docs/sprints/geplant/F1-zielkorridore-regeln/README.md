# F1 · REG · Zielkorridore und Bedienungsregeln

- **Status:** geplant
- **Domäne:** REG
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-134, B-135, B-136, B-144, B-145
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Regelwerke R1 bis R4 sind beschlossen (`docs/rules/*.md`), enthalten aber Zielkorridore nur verstreut. Für Pause, Schriftgröße am TV, Verbindungsverlust und Sprache gibt es keine Regel; die Aktion `pause` existiert ohne Funktion. Die fünf Entscheidungen Q01 bis Q05 stehen in `docs/fragenkatalog.md`.

## Ziel

Jede Kennzahl hat einen Zielkorridor als Zahl, und Pause, Schriftgröße, Verbindungsverlust und Sprache sind als Regel mit Zahl beschlossen. Am Ende sichtbar: `docs/rules/zielkorridore.md` und `docs/rules/bedienung.md` mit Datum der Bestätigung durch 🧑.

## Beteiligte und Zielgruppen

🧑 entscheidet in drei Workshops (Q01 bis Q05); ein Agent bereitet die Vorschläge aus `docs/rules/*.md` vor und schreibt die Beschlüsse nieder. Danach arbeiten BAL2, S4, S5 und SP-Sprints mit den Zahlen.

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

## Offene Fragen

Q01 bis Q05 aus `docs/fragenkatalog.md`; entscheidet 🧑 in den Workshops. Blockiert die Freigabe der Spec nicht, wohl aber die Abnahme der jeweiligen Kriterien.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- F1.1 🧑 Workshop Zielkorridore: Agent bereitet Vorschlag aus `docs/rules/*.md` vor, 🧑 entscheidet Q02, Ergebnis in `docs/rules/zielkorridore.md` (AC-01, AC-02).
- F1.2 🧑 Workshop Bedienung 1: Pause (Q01) und Mindest-Schriftgröße (Q03) in `docs/rules/bedienung.md` (AC-03, AC-04).
- F1.3 🧑 Workshop Bedienung 2: Verbindungsverlust und Latenz (Q04), Sprache (Q05); schließt den Sprint ab (Doku-Sprint, kein Review), Tickets archivieren (AC-05, AC-06, AC-08).
- F1.4 🧑 Reittier-Regel: Standard-Reittier und Faktoren (Q23) in `docs/rules/monarch.md` (AC-07).

## Abnahme

–
