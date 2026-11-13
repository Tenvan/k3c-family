# B-826 · Fahrplan-Abschnitt „Offen am Gerät“ und verwaiste Zeilen pflegt das Werkzeug

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Fahrplan `docs/sprints/README.md` hat Teile, die kein `plan_*`-Tool schreibt, und der Hook `tools/hooks/planGuard.ts` lehnt Handänderungen ab, solange die Workbench läuft (WZ44.4, 2026-11-13):

- Abschnitt „Offen am Gerät“ ist Freitext und verweist noch auf Projekt ABN/Sprint HW1 (beide entfallen mit Entscheidung 009).
- Zeile LG1 steht unter „Geplant“ mit `geplant/LG1-…/`, obwohl LG1 in `erledigt/` liegt; `plan_set` entfernt nur die Zeile mit dem aktuellen Ordner (`takeRoadmapRow`, `tools/k3c-workbench/internal/planning/tables.go`).
- Zeile KB6 unter „Erledigt“ hatte sieben statt drei Spalten; `plan_set KB6` hat sie auf drei gekürzt, übernimmt aber die alte zweite Zelle als Thema (`SIM, REG` statt des Sprint-Titels, `tables.go` Zeile mit `firstOf(known[h], old[h], …)`).

## Ziel

Der Fahrplan entsteht vollständig aus den Dateien: `plan_set` (oder ein eigener Aufruf) schreibt „Offen am Gerät“ aus den offenen Mensch-Sessions aktiver Sprints nach Gerät (Zeile `Gerät:` im Kontext, wie `plan_queue`) und heilt Zeilen eines Sprints in falschen Tabellen.

## Beteiligte und Zielgruppen

🧑 (Gerätetermine), planende Agenten.

## Anforderungen

- „Offen am Gerät“ wird bei jedem schreibenden `plan_*`-Aufruf aus den Session-Dateien neu geschrieben (je Gerät eine Liste; Reihenfolge wie `plan_queue`).
- Zeilen eines Sprints, deren Ordner nicht mehr existiert, werden entfernt (LG1 unter „Geplant“).
- Eine Zeile mit falscher Spaltenzahl nimmt das Thema aus dem Sprint-Titel, nicht aus einer verschobenen Zelle (KB6).

## Nicht-Ziele

Neue Planungsregeln; Oberfläche der Planungsseite.

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Test in `tools/k3c-workbench/internal/planning`.

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `docs/sprints/README.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Nach `plan_set NG15.3 Status fertig` verschwindet NG15.3 aus „Offen am Gerät › PC“.

## Ausnahme- und Fehlerfälle

Session ohne Zeile `Gerät:` → Gerät „unbekannt“.

## Akzeptanzkriterien

- **AC-01** „Offen am Gerät“ nennt nach einem schreibenden `plan_*`-Aufruf jede offene Mensch-Session aktiver Sprints genau einmal unter ihrem Gerät.
- **AC-02** LG1 steht nur unter „Erledigt“, KB6 trägt seinen Titel als Thema; Test für beide Fälle.

## Offene Fragen

keine

## Notizen

Befund aus WZ44.4 (B-785/AC-02, AC-03 teilweise verschoben). Die Sessions tragen seit WZ44.4 eine Zeile `Gerät:` im Kontext (NG9.5 Mini-PC; NG11.4, NG14.5, NG15.3, KB4.4, KB5.6, W15.3 PC; PB1.2 Veröffentlichung).
