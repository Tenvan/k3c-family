# NG15 · CLI · Natives Spiel: Ton-Gerät ohne Absturz

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-805
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

`native/k3c-game/audio` nutzt `ebiten/audio`; dessen Hook gibt jeden Treiberfehler vor dem Update zurück und beendet `RunGame` (Ebitengine v2.10.6, `audio/audio.go:131`). Einzelheiten: B-805.

## Ziel

Ein defektes oder verschwindendes Ton-Gerät macht das native Spiel stumm, beendet es aber nicht. Am Ende sichtbar: Test mit nachgestelltem Treiberfehler grün, Log-Zeile, Abnahme 🧑 am PC (NG15.3).

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC; Claude (Umsetzung, Review); 🧑 Ralf (Abnahme am PC: Gerät während des Spiels entfernen).

## Anforderungen

`B-805 › Anforderungen` (Revision 2: oto direkt, stumm bis zum Neustart).

## Nicht-Ziele

`B-805 › Nicht-Ziele`; Wiederverbinden eines zurückkehrenden Geräts.

## Regeln und Einschränkungen

Entscheidung 008. oto (`github.com/ebitengine/oto/v3`, heute indirekt v3.5.0) wird direkte Abhängigkeit, ohne Versionswechsel (B-805 › Offene Fragen). Importregel aus `native/k3c-game/view/doc.go` gilt weiter; `game` bleibt unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/audio/**`, `native/k3c-game/go.mod`, `native/k3c-game/go.sum`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** deps

## Beispiele

`B-805 › Beispiele` (Kopfhörer abziehen → Ton aus, Spiel läuft weiter).

## Ausnahme- und Fehlerfälle

`B-805 › Ausnahme- und Fehlerfälle`. Abnahme am PC nicht möglich → NG15.3 bleibt offen, der Sprint bleibt aktiv (Hardware entkoppelt).

## Akzeptanzkriterien

- **AC-01** Nachgestellter Treiberfehler: Spiel läuft weiter, Log-Zeile vorhanden (`B-805/AC-01`).
- **AC-02** Am PC Gerät während des Spiels entfernt, Spiel läuft weiter (Abnahme 🧑, `B-805/AC-02`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: oto direkt, siehe `B-805 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG15.1 | `NG15.1-ton-oto.md` | Umsetzung | autonom | fertig |
| NG15.2 | `NG15.2-review.md` | Review | autonom | fertig |
| NG15.3 | `NG15.3-abnahme-pc.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-12 Review (Claude, `k3c-reviewer`, NG15.2): AC-01 belegt im Ergebnis von NG15.1 (Commit 403454629); AC-02 angenommen, Validierung offen in NG15.3 (🧑 Ralf, Gerät entfernen).
Behobene Befunde: keine schweren. Vermutet: Ton läuft beim Minimieren weiter, Prüfung in NG15.3. Neue Tickets: keine.
Abnahme-Ticket für Sim, Balance und Gen: keins nötig (kein Gen-Feature; Sim gesperrt laut Beschluss 2026-11-11).
Sprint bleibt aktiv bis NG15.3.
