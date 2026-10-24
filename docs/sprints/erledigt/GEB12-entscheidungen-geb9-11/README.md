# GEB12 · REG, CLI · Entscheidungen zu GEB9 bis GEB11 (Server-Stufenfolge, Markierer im Snapshot, Wald-Dichte)

- **Status:** erledigt
- **Projekt:** GEB
- **Domäne:** REG, CLI
- **Reife:** bereit
- **Tickets:** B-458, B-531, B-532, B-538, B-539, B-540, B-541, B-542, B-543, B-544, B-545
- **Start-Commit:** df21c2c2
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

GEB9 (Ausbaustufen der Hub-Gebäude, PR 433), GEB10 (Natur-Nachträge, `sprint/geb10`) und GEB11 (Hub wächst, PR 435) wurden am 2026-10-22 komplett autonom umgesetzt. Alles, was Regeln, Server, Protokoll, Spielstand oder Geschmack berührt und dabei nicht von Ralf vorgegeben war, hat Claude mit einem Default umgesetzt oder bewusst offen gelassen. Dieser Sprint legt diese Entscheidungen Ralf vor. Details stehen in den Sprint-READMEs und Session-Ergebnissen von GEB9, GEB10 und GEB11 sowie in B-531 und B-532 (Branch `sprint/geb10`).

## Ziel

Alle Entscheidungen aus GEB9 bis GEB11 stehen mit Default fest und liegen Ralf als Frage-Tickets im KRT-Sprint KR2 vor; die Folgearbeit ist geplant, der kleine CLI-Default D8 ist umgesetzt. Am Ende sichtbar: je Entscheidung D1–D8 ein Frage-Ticket (B-538 bis B-545), Vermerke in den Specs von GEB9, GEB10, GEB11, das Baugerüst erscheint nachts nicht mehr.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet; Claude bereitet vor (Optionen, Empfehlung) und plant die Folgearbeit. Spielende am TV sind indirekt betroffen (Aussehen und Regeln des Hubs, Wald, Wimpel).

## Anforderungen

Je Entscheidung ein Frage-Ticket mit Frage, Default, Alternative und Folgen (Abschnitt Notizen): D1 B-538, D2 B-539, D3 B-540, D4 B-541, D5 B-542, D6 B-543, D7 B-544, D8 B-545. Die Defaults werden in die Offenen Fragen von GEB9, GEB10 und GEB11 mit Verweis auf das Ticket eingetragen. Einträge in `docs/decisions/` entstehen erst mit Ralfs Bestätigung (GEB12.1), weil Entscheidungsdokumente nur bestätigte Entscheidungen festhalten.

## Nicht-Ziele

Umsetzung von Folgearbeit mit Protokoll-, Spielstand- oder Balancing-Wirkung (D1 Option C, D2 B-532, D3 B-531): eigene Sprints nach Ralfs Entscheidung. Neue Grafik. Geräte-Abnahmen GEB9.6, GEB10.5, GEB11.6 (bleiben in ihren Sprints).

## Regeln und Einschränkungen

Planung und Dokumentation (`docs/`) über `plan_*`; Code nur in GEB12.3 (Client, `src/scenes/render/hubGrowth.ts` und Test). Protokoll und Spielstand ändern nur mit Entscheidung von Ralf. Keine Rücksicht auf alte Spielstände. Autonom freigegeben (Auftrag „autonom abarbeiten“); Defaults gelten, bis Ralf widerspricht.

## Beispiele

D8 bestätigt → B-545 erledigt, nichts weiter. D1 = Option C → Folge-Sprint SIM/REG „Hub-Ausbauschritte“ mit Protokollfeld `hubStep`. D2 = Ja → B-532 wird umgesetzt (Feld `markedBy`).

## Ausnahme- und Fehlerfälle

Ralf lehnt eine autonom umgesetzte Variante ab → Anpassung als Ticket im Projekt GEB; die betroffene Abnahme (GEB9.6, GEB10.5, GEB11.6) bleibt offen. D8 abgelehnt → eine Zeile in `hubLookOf` zurück (B-545 › Notizen).

## Akzeptanzkriterien

- **AC-01** Jede Entscheidung D1–D8 ist von Ralf bestätigt, geändert oder verworfen und im Frage-Ticket (B-538 bis B-545) vermerkt; bestätigte Entscheidungen mit Regelwirkung stehen in `docs/decisions/` bzw. der Regeldatei.
- **AC-02** Je Entscheidung D1–D8 existiert ein Frage-Ticket im Sprint KR2 mit Default und Folgen; die Offenen Fragen von GEB9, GEB10 und GEB11 verweisen darauf.
- **AC-03** Folgearbeit ist geplant: B-531 (D3) und B-532 (D2) im Projekt GEB mit Prio und Verweis auf ihre Frage-Tickets; für D1 Option C und D4/D5/D7-Änderungen nennt das Frage-Ticket den Folge-Umfang.
- **AC-04** Das Baugerüst vor der Burg erscheint nur bei `hubUpgrade.state = waitingWorker` ohne `danger` (D8-Default); Vitest in `src/scenes/render/hubGrowth.test.ts`.

## Offene Fragen

Alle Entscheidungen sind autonom mit Default getroffen (Claude, 2026-10-22) und liegen als Frage-Tickets in KR2; Ralf bestätigt in GEB12.1 bzw. KR2.1. Defaults: D1 A (nur Grafik), D2 Annäherung bis GEB10.5, D3 einplanen ohne Werteänderung, D4 11,25 Kacheln, D5 hinter der Farm, D6 ohne Serverzustand, D7 GEB9-Defaults, D8 Gerüst nur ohne Gefahr (umgesetzt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB12.1 | `GEB12.1-workshop-entscheidungen.md` | Workshop | Mensch | fertig |
| GEB12.2 | `GEB12.2-folgearbeit-planen.md` | Workshop | autonom | fertig |
| GEB12.3 | `GEB12.3-geruest-ohne-gefahr.md` | Umsetzung | autonom | fertig |

GEB12.1 (AC-01) entscheidet Ralf zusammen mit KR2.1. GEB12.2 (AC-02, AC-03) und GEB12.3 (AC-04) laufen autonom.

## Abnahme

2026-10-24: AC-01 (GEB12.1/KR2.1: D1–D8 entschieden, D2 `markedBy` bleibt), AC-02 und AC-03 (GEB12.2), AC-04 (GEB12.3) erfüllt. Neue Tickets: B-531 (Werte D3), B-568 bis B-572.
