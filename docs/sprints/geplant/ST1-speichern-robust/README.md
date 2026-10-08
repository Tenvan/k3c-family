# ST1 · SRV · Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik

- **Status:** geplant
- **Projekt:** LST
- **Domäne:** SRV
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-186, B-187, B-272
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Server speichert beim Verlassen, nicht periodisch (B-186); unter Windows scheitert Speichern an kurz gesperrten Dateien (B-187); Spielmetrik-Reports fallen aus der Rotation (B-272).

## Ziel

Ein Absturz kostet höchstens 60 s Spiel, kein Spielstand oder Bericht geht durch eine gesperrte Datei verloren. Am Ende sichtbar: HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt.

## Beteiligte und Zielgruppen

Spielende am TV; 🧑 betreibt den Server am PC oder Pi.

## Anforderungen

B-186 › Anforderungen; B-187 › Anforderungen; B-272 › Anforderungen.

## Nicht-Ziele

Cloud-Speicher, Spielstand-Migration (B-288).

## Regeln und Einschränkungen

SRV; die HUD-Anzeige „gesichert“ läuft als eigene Session (Protokoll-Grenzfall).

## Beispiele

Server speichert um 21:00:00 und 21:01:00 → zwei 💾-Logzeilen, HUD blinkt „gesichert“.

## Ausnahme- und Fehlerfälle

Zieldatei 2 s gesperrt → Wiederholung mit Frist ⏳, danach ❌ im Log, alter Stand bleibt.

## Akzeptanzkriterien

- **AC-01** Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ (B-186/AC-01, B-186/AC-02, B-186/AC-03).
- **AC-02** Speichern übersteht unter Windows eine kurz gesperrte Zieldatei (B-187/AC-01, B-187/AC-02).
- **AC-03** Die Rotation in reports/ erfasst auch die Spielmetrik-Reports (B-272/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- ST1.1 Speichern alle 60 s und bei Tagesanbruch, Anzeige „gesichert“ (AC-01).
- ST1.2 Wiederholung bei gesperrter Datei, Rotation für Spielmetrik (AC-02, AC-03).
- ST1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
