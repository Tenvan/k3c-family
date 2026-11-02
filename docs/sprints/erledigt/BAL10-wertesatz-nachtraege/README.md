# BAL10 · SIM · Welt-Wertesatz: Nachträge aus dem Review BAL8.2

- **Status:** erledigt
- **Projekt:** BAL
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-692
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Review BAL8.2 hat sechs Punkte mittlerer Schwere offen gelassen: Skills, Spielstand-Prüfung, Level-Generator, `UseData`, `-check-data`/Balance-Werkzeug und Allokation in `World.values()` nutzen noch die Basis (B-692).

## Ziel

Overrides wirken überall, wo die Regel es verspricht, oder sind dort ausdrücklich gesperrt. Am Ende sichtbar: Go-Tests je Punkt grün, `docs/rules/welten.md` nennt die Ausnahmen.

## Beteiligte und Zielgruppen

Wer Welten und Balancing plant; Agent SIM.

## Anforderungen

`B-692 › Ausgangslage` (sechs Punkte).

## Nicht-Ziele

Editor und Chat (WZ39, B-693); neue Balancing-Werte.

## Regeln und Einschränkungen

Deterministisch; Golden nur mit Begründung; keine Rücksicht auf alte Spielstände; `engine/` importiert nichts aus `tools/`.

## Beispiele

Welt übersteuert `monarch.skills.*` → Skill-Katalog des Raums nutzt den Override.

## Ausnahme- und Fehlerfälle

Override auf einen nicht übersteuerbaren Pfad → Welt wird beim Laden mit Pfad abgelehnt.

## Akzeptanzkriterien

- **AC-01** Je Punkt Test oder Eintrag in `docs/rules/welten.md` (`B-692/AC-01`).
- **AC-02** `task check:diff` grün (`B-692/AC-02`).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Entwurf, abgearbeitet am 2026-11-02 in einem Lauf auf `develop` (Auftrag 🧑: „offene Tickets abarbeiten“).

- BAL10.1 (SIM) Sechs Punkte (AC-01, AC-02).
- BAL10.2 Review, alle Kriterien.

## Abnahme

2026-11-02 (Claude, ein Lauf auf `develop`): AC-01 je Punkt `TestB692*` (Skills, Spielstand, Rüstung, Ruinen-HP, gesperrte Felder, UseData, ResolveWorldFiles, Cache) plus `TestPruefungWeltenKette`, `TestWeltMitOverridesImLauf`, Sperrliste in `docs/rules/welten.md` § 1.3; AC-02 `task check:diff` grün, Golden unverändert. Review-Befund mittel (`hub.sitePlacement` nicht gesperrt) behoben; niedrige Befunde in B-694.
