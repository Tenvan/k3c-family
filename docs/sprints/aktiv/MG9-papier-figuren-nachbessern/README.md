# MG9 · PAP · Papierschnitt nachbessern: Monarch, Reittiere, Truppen, Gegner

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-716, B-717, B-718, B-719
- **Start-Commit:** 5305c03d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-07 (auf Ralfs Zustimmung im Chat)

## Ausgangslage

PS1.3 und PS1.4: Spielerfarben, Reit-Idle, Reittier-Silhouetten, Truppen- und Gegner-Befunde.

## Ziel

Figuren, Reittiere, Truppen und Gegner bestehen die Sichtprüfung.

## Beteiligte und Zielgruppen

Ralf (🧑) prüft die Figuren per Sichtprüfung in Workbench › Assets und nimmt ab; Claude setzt um; Nutzer sind die Spieler (US3 baut die Assets ins Spiel ein).

## Anforderungen

B-716, B-717, B-718, B-719.

## Nicht-Ziele

Bosse (MG10), Spiel-Einbau (US3).

## Regeln und Einschränkungen

Nach RD2. Lanes `reittiere`, `monarch`, `truppen`, `gegner` parallel (max. 4); `monarch` nach `reittiere`.

## Beispiele

Spieler 4 auf dem Wolf: Rock und Satteldecke in einer kontrastreichen Farbe, nicht Braun. Krieger im Angriff: Klinge vor dem Körper, Gesicht frei.

## Ausnahme- und Fehlerfälle

Passt eine Vorgabe nicht in den Rahmen eines Assets, wächst der Rahmen (kein Skalieren der Figur); bleibt ein Befund unlösbar, wird er als Ticket im Projekt PAP festgehalten und im Ergebnis der Session genannt.

## Akzeptanzkriterien

- **AC-01** B-716/AC-01 bis AC-03
- **AC-02** B-717/AC-01 bis AC-03
- **AC-03** B-718/AC-01, AC-02
- **AC-04** B-719/AC-01 bis AC-03

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG9.1 | `MG9.1-reittiere.md` | Umsetzung | autonom | fertig |
| MG9.2 | `MG9.2-monarch.md` | Umsetzung | autonom | fertig |
| MG9.3 | `MG9.3-truppen.md` | Umsetzung | autonom | fertig |
| MG9.4 | `MG9.4-gegner.md` | Umsetzung | autonom | offen |
| MG9.5 | `MG9.5-review.md` | Review | autonom | offen |

Reife Entwurf:

- MG9.1 Reittiere (B-716)
- MG9.2 Monarch (B-717)
- MG9.3 Truppen (B-718)
- MG9.4 Gegner (B-719)
- MG9.5 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
