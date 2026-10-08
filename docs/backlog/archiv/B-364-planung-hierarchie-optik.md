# B-364 · Die Planungsseite zeigt Projekt, Sprint und Session als klar unterscheidbare Ebenen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-08, Screenshot der Planungsseite nach PJ3.1): Titel und Panelfarben der Projekt-Hierarchie besser ausarbeiten. Beobachtet im Reiter Planung (`tools/k3c-dev/frontend/src/planning/ProjectsView.tsx`, `ProjectCard.tsx`, `SprintCard.tsx`):
- Die Spaltenüberschrift heißt weiter „Sprints · 51 von 136 · 85 erledigt“, obwohl die Spalte jetzt Projekte zeigt.
- Projekt-Karte, darin Sprint-Karten und darin Session-Zeilen haben dieselbe Panelfarbe und fast denselben Rahmen; die Ebenen sind nur an der Einrückung zu erkennen.
- Die Sprint-Chips im Projektkopf (`PM1 · geplant · 0/4`) und die Status-Badges der Sprints (`geplant`, `Spec: Entwurf`) sehen gleich aus wie die Session-Badges (`entwurf`, `offen`).

## Ziel

Auf einen Blick ist erkennbar, was Projekt, Sprint und Session ist: passende Überschriften und je Ebene eine eigene, ruhige Panel-Abstufung (hell/dunkel) aus dem Design-System der Workbench.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Überschrift der Spalte nennt Projekte (z. B. „Projekte · 10 aktiv · Sprints 51 von 136“), nicht „Sprints“.
- **AC-02** Projekt-, Sprint- und Session-Ebene haben je eine eigene Panel-Farbe bzw. Rahmen/Akzent aus Theme-Tokens, in Dark und Light lesbar; die Projekt-Karte trägt Rang und Kürzel als Kopf.
- **AC-03** Sprint-Chips im Projektkopf und Session-Badges unterscheiden sich sichtbar (Form oder Farbe).
- **AC-04** Komponententest bzw. Mock-Daten-Prüfung im Browser-Pane (Screenshot im Ergebnis), `dev:test` grün.

## Offene Fragen

- Konkrete Farben je Ebene: Vorschlag in der Session als Screenshot, Auswahl durch 🧑.

## Notizen

2026-10-08 auf Wunsch von 🧑 direkt in PJ3 umgesetzt (eigener Commit auf `sprint/pj3`, Domäne SRV außerhalb des INF-Sprints): Überschrift „Projekte · n nach Rang · Sprints …“, Projekt-Karte mit Akzentkante, getönter Fläche und Rang-Kachel, Sprint-Karten als ruhige Panel-Fläche darin, Sessions ohne Fläche, Sprint-Chips eckig und anklickbar (springen zum Sprint), ABN amber, ruhend/erledigt grau. Zusätzlich (Wunsch 🧑): Projekte, Sprints, ABN und „Ohne Projekt“ einklappbar, gemerkt in den Prefs, „alle einklappen/aufklappen“ an der Überschrift. Geprüft: `dev:test` grün, Browser-Pane mit Mock-Daten (Agent).
