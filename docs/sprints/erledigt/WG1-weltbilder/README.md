# WG1 · PAP · Weltbilder: großes Titelbild für die Weltauswahl, kleines für Spielstand und Pause

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Welten haben keine eigenen Bilder.

## Ziel

Großes und kleines Weltbild je Welt als Papierschnitt-Asset.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/**`, `data/paper.json`
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Weltbilder für `standard` (und die lokalen Welten) (`B-937`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WG1.1 | `WG1.1-weltbilder.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-21 (Claude, Klasse A, nachträgliche Prüfung): AC-01 geprüft: je Welt (standard, sengoku, nil, britannien) ein großes und ein kleines Weltbild unter `public/grafik/hintergruende/weltbild-*`, Zuordnung in `data/paper.json` › `weltbilder`.
Gen und Golden laufen im Abnahme-Ticket B-1145 (Lane zweit); Sichtprüfung der Bilder bei 🧑. Die Session-Datei WG1.1 enthält keinen Ergebnis-Text.
