# LB1 · CLI · Lobby zeigt Räume und startet Spiele

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-037
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ohne Lobby muss man Raumcodes kennen (B-037).

## Ziel

Die Lobby zeigt Räume und startet oder betritt ein Spiel per Controller. Am Ende sichtbar: Lobby listet offene Räume, Beitritt ohne Raumcode.

## Beteiligte und Zielgruppen

Familie am TV und Handy; Agent baut in `src/scenes/`.

## Anforderungen

B-037 › Anforderungen.

## Nicht-Ziele

Anlegen-Dialog mit Optionen (K5).

## Regeln und Einschränkungen

CLI; B nicht belegen.

## Beispiele

Lobby öffnen → Raum „ABCD“ mit 2 Spielern, A tritt bei.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Hinweis, Wiederholen-Knopf.

## Akzeptanzkriterien

- **AC-01** Lobby zeigt Räume und startet Spiele (B-037/AC-01, B-037/AC-02, B-037/AC-03, B-037/AC-04).

## Offene Fragen

- Stand: B-037/AC-01 bis AC-03 sind seit SP08 umgesetzt (Raumliste und Beitritt in `LobbyScene`), offen ist AC-04 (Spielstand wählen). LB1.1 setzt das um und belegt alle vier.
- Spielstand-Einträge (Vorschlag, gilt mit der Freigabe der Spec): Die Lobby liest `GET /api/saves`, zeigt die neuesten sechs Spielstände nach `savedAt` unter den Räumen, ohne unlesbare und ohne solche, die schon als Raum offen sind; Auswahl sendet `create` mit `fresh: false`. 🧑
- „Spielen (Name)“ bleibt oben als Start mit `?save` bzw. `familie` (Vorschlag, gilt mit der Freigabe). 🧑

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LB1.1 | `LB1.1-raumliste-spielstand.md` | Umsetzung | autonom | offen |
| LB1.2 | `LB1.2-review.md` | Review | autonom | offen |

## Abnahme

–
