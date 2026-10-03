# DBG2 · CLI · Debug-Overlay: Gold, Material, Zeitraffer

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-179
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Overlay zeigt nur Informationen (B-093). Voraussetzung: DBG1 (Protokoll `dev`). Vorgezogen für Tests (🧑, 2026-10-03); steht in der CLI-Bahn vor S5.

## Ziel

Das Debug-Overlay bedient Gold droppen, Material geben und den Zeitraffer. Am Ende sichtbar: Aktionen im Overlay am PC, Handy und Controller.

## Beteiligte und Zielgruppen

🧑 und Entwickler beim Testen; Agenten setzen um; 🧑 nimmt am Gerät ab.

## Anforderungen

B-179 › Anforderungen.

## Nicht-Ziele

Serverseitige Aktionen (DBG1), Grad-Wechsel (B-107).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`, `src/online/`); `src/scenes` zeichnet nur, B nicht belegen, View + Menu reserviert.

## Beispiele

Overlay mit Ö öffnen → „Gold 50“ → Münzen am Spieler; „8×“ → Faktor im Overlay.

## Ausnahme- und Fehlerfälle

Raum ohne Dev-Mode → keine Liste; `forbidden` → Hinweis im Overlay.

## Akzeptanzkriterien

- **AC-01** Eine reine Funktion macht aus der Auswahl die `dev`-Nachricht (B-179/AC-01).
- **AC-02** Die Aktionsliste erscheint nur im Dev-Mode des Raums bei aktivem Overlay (B-179/AC-02).
- **AC-03** Das Overlay zeigt den Zeitfaktor (B-179/AC-03).
- **AC-04** `noSim`-Test und `task check` grün (B-179/AC-04).
- **AC-05** 🧑 hat die Aktionen mit allen drei Eingaben ausprobiert (B-179/AC-05).

## Offene Fragen

Siehe Ticket › Offene Fragen.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- DBG2.1 Reine Funktionen: Auswahl → `dev`-Nachricht, Sichtbarkeit der Aktionsliste, Test (AC-01, AC-02).
- DBG2.2 Overlay-Ansicht mit Aktionen und Zeitfaktor, Bedienung Maus, Touch, Controller (AC-03).
- DBG2.3 🧑 Abnahme am PC, Handy und Controller (AC-05).
- DBG2.4 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05).

## Abnahme

–
