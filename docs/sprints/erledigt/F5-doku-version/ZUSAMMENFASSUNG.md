# F5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Doku-Drift, Version und Landing-Kacheln
- **Projekt:** –

## Ziel

Doku und `CLAUDE.md` zeigen den heutigen Stand, die Landingpage nennt Server- und Client-Version, und jede Kachel führt in die Lobby statt in tote Parameter. Am Ende sichtbar: Fußzeile „Server <Version> · Client <Version>“ auf der Landingpage am TV, keine toten Kacheln.

## Ergebnis

- 2026-10-03, Review F5.4: AC-01, AC-02 (F5.1), AC-03 bis AC-05 (F5.2), AC-06 (F5.3) belegt in den Session-Ergebnissen; AC-04 Optik am TV angenommen, Validierung offen (🧑); `task check` und `task check:go` grün (`-race` lokal ohne C-Compiler, die CI prüft es). - Keine schweren Befunde; `/api/health` behält `ok: true`, keine Seitenregel berührt, Version enthält kein Geheimnis. Beobachtung ohne Befund: Fußzeile zeigt „Client … · Server …“ statt der Spec-Reihenfolge (nur Reihenfolge, beide Versionen und Abweichung belegt). Keine neuen Tickets; B-141 und B-079 archiviert. - Version: v0.6.0 vorgesc …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/rules/archiv/` enthält die vier `ist-*.md`; `docs/rules/` selbst keine; kein Verweis … |
| AC-02 | geprüft | Der Satz zur Spielstruktur in `CLAUDE.md` nennt keine „Umsetzung offen“ mehr (B-141/AC-02) … |
| AC-03 | geprüft | `GET /api/health` liefert `ok: true` und `version` (B-141/AC-03). |
| AC-04 | geprüft | Die Fußzeile der Landingpage zeigt Server- und Client-Version und markiert Abweichung (B-1 … |
| AC-05 | geprüft | `.html` wird mit `no-cache`, JS und Assets mit `immutable` ausgeliefert, per Test belegt ( … |
| AC-06 | – | Jede Kachel mit Ziel `game.html` benutzt nur Parameter, die die Lobby kennt (B-079/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-141 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft | Doku und CLAUDE.md stimmen mit dem Code überein, die Version ist sichtbar |
| B-079 | erledigt | AC-06 – | Die Kacheln der Landingpage passen zum Start über die Lobby |
