# LB1 · CLI · Lobby zeigt Räume und startet Spiele

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** Entwurf
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

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- LB1.1 Raumliste und Beitritt (AC-01).
- LB1.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
