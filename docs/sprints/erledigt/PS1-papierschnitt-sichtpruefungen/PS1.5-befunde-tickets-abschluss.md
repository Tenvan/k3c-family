# PS1.5 · Befunde als Tickets, Sprint schließen

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ps1
- **Abhängig von:** PS1.4
- **Tickets:** B-699, B-701
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Alle Befunde aus PS1.1–PS1.4 sind Tickets im Projekt PAP, B-701 hat keine offene Zeile, der Sprint ist abgeschlossen.

## Kontext

Liest die Ergebnisse von PS1.1–PS1.4.

## Erlaubte Dateien

`docs/backlog/` (nur über `plan_*`), `docs/sprints/…/PS1-*/`.

## Nicht-Ziele

Befunde beheben.

## Schritte

1. Je ✏️ aus PS1.1–PS1.4 ein Ticket per `plan_create` (Projekt PAP, Lane im Text, Verweis auf D-Zeile).
2. B-701: Status jeder Zeile auf `bestätigt` oder `geändert (B-…)`.
3. Abnahme der Sprint-README füllen (höchstens fünf Zeilen).
4. B-701 und B-699 schließen, wenn AC erfüllt.

## Fertig, wenn

B-701 ohne `offen`, jeder Befund hat ein Ticket.

## Prüfen

`task test -- planning` grün; `grep -c '| offen' B-701` = 0.

## Ergebnis

2026-11-07, Claude (autonom):

- 13 Folge-Tickets im Projekt PAP, gebündelt: B-712 Stil (Schatten wie Prototyp, neue Materialien), B-713 renderKey mit Reittier, B-714 Keyframe `sichtbar`, B-715 Rig-Proportionen je Tier und Figur, B-716 Reittiere je Tier und Maßstab, B-717 Monarch (Reit-Idle, Gangart je Reittier, give/hit/death/respawn, Spieler 4 und Satteldecke, löst B-711), B-718 Truppen, B-719 Gegner, B-720 Bosse neu, B-721 Gebäude, B-722 Hintergründe neu ohne Bäume, B-723 Natur, Icons, Münze, B-724 Spielbefunde PS1.4 (mit Bildern aus `docs/playtests/ps1/`).
- Vorschlag Welle 4 in `geplant/` (Entwurf): RD2 Renderer/Stil (B-712–B-715) → danach parallel MG9 Figuren (B-716–B-719), MG10 Bosse (B-720) und MG11 Welt (B-721–B-723) → US3 Spiel (B-724).
- B-701: 88 Zeilen `entschieden` (3 mit Rest in CU1), 27 Zeilen verweisen auf ihr Ticket. Die D-MG8-Zeilen haben die fehlenden Spalten nachgetragen bekommen. B-701 ist erledigt.
- PS1 und MG1–MG8 erledigt. B-699 bleibt eingeplant (AC-10 erst mit CU1).
