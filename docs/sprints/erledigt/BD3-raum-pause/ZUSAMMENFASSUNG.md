# BD3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum-Pause im Couch-Raum, Schutz des pausierten Monarchen online
- **Projekt:** BED

## Ziel

Ein Gerät kann den Raum pausieren: Im Couch-Raum steht die Simulation wirklich still, online ist der Monarch des pausierten Geräts höchstens 60 s unverwundbar. Am Ende sichtbar: `task check:go` grün, neue Protokollversion mit Beispiel unter `testdata/protocol/`.

## Ergebnis

2026-10-22 (BD3.4, autonom): AC-01 und AC-02 umgesetzt und per Go-Test geprüft (BD3.2, BD3.3), AC-03 umgesetzt (BD3.1, Protokoll v8, Beispiele `c2s-pause.json`, `c2s-resume.json`). `task check` und `task check:go` grün; `task check:race` lokal übersprungen (kein C-Compiler), prüft die CI. Behobene Befunde: Eingaben aus der Pause verfallen bei resume, Frist steht in der Dev-Pause. Sprint-Review (Opus): abgenommen. Neue Tickets: B-528, B-529 (Entscheidungen zur Prüfung durch 🧑), B-530 (wartende Monarchen nicht unverwundbar). Version: v0.21.0 gemeinsam mit BD7 als ein Release (Entscheidung 🧑 2026 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Go-Test: Im Couch-Raum ändert sich der Zustand während der Pause nicht, nach dem Fortsetze … |
| AC-02 | geprüft | Go-Test: Online ist der pausierte Monarch 60 s unverwundbar, danach verwundbar; andere Ger … |
| AC-03 | geprüft | `docs/protocol.md` beschreibt `pause`/`resume` mit neuer Version und Beispiel unter `testd … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-214 | erledigt | AC-01 geprüft, AC-02 geprüft | Der Server pausiert den Raum im Couch-Raum und schützt den stehenden Monarchen o … |
