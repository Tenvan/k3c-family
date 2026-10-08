# W8 · CLI · Bauplätze mit Grund und alle Rohstoffe im Client

- **Status:** geplant
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-207, B-209, B-188
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client zeigt nicht, warum ein Platz gesperrt ist (B-207), kennt nicht alle Platz-Arten aus `hub.json` (B-209) und nicht alle fünf Rohstoffe (B-188).

## Ziel

Spieler sehen am Platz, ob er frei ist und was fehlt; neue Platz-Arten und Rohstoffe fallen beim Typecheck auf. Am Ende sichtbar: Gesperrte Plätze zeigen den Grund, Client-Typen passen zu `hub.json` und den fünf Rohstoffen.

## Beteiligte und Zielgruppen

Spielende am TV; Agent baut in `src/scenes/` und `src/model/`.

## Anforderungen

B-207 › Anforderungen; B-209 › Anforderungen; B-188 › Anforderungen.

## Nicht-Ziele

Neue Gebäude, Wirtschaftsregeln.

## Regeln und Einschränkungen

CLI; nach W6 (Anzeigen Wirtschaft).

## Beispiele

Platz „Schmiede“ bei Hub-Stufe 2 → „ab Hub-Stufe 3“.

## Ausnahme- und Fehlerfälle

Unbekannte Platz-Art vom Server → Platzhalter, Warnung 🤒 im Client-Log.

## Akzeptanzkriterien

- **AC-01** Der Client zeigt freie und gesperrte Bauplätze mit Grund (ab Hub-Stufe n, Linie fehlt) (B-207/AC-01, B-207/AC-02, B-207/AC-03).
- **AC-02** `src/model/data.ts` kennt alle Platz-Arten aus `hub.json` (B-209/AC-01, B-209/AC-02).
- **AC-03** Der Client kennt alle fünf Rohstoffe des Servers (B-188/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W8.1 Client-Typen für Platz-Arten und Rohstoffe (AC-02, AC-03).
- W8.2 Freie und gesperrte Plätze mit Grund (AC-01).
- W8.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
