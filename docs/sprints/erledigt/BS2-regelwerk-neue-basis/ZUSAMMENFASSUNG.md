# BS2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Regelwerk der neuen Basis: Münzskala, Baumschule, Hafen und Schiff
- **Projekt:** BAS

## Ziel

Die neue Basis steht verbindlich im Regelwerk und im Glossar, bevor Werte und Code folgen: Münzskala 2–30 mit Ausnahmen, Baumschule auf gerodeten Waldbauplätzen, Hafen-Chunk auf Inselstufe 1 und Schiff mit Bauabschnitten und Ressourcen. **Am Ende sichtbar:** `docs/rules/wirtschaft.md`, `materialien-gebaeude.md`, `stufen.md`, `geplante-chunks.md`, `zielkorridore.md` und `docs/glossar.md` tragen die …

## Ergebnis

2026-10-26, Claude (Doku-Sprint, letzte Session BS2.2). AC-01 siehe BS2.1, AC-02 bis AC-04 siehe BS2.2. Befund: BS2.1 änderte den Gold-Korridor nur in der Regel, `balance-targets.json` zog BS1.1 nach (Go-Test `TestGrenzenWieTabelle` lief bei Docs-Diffs nicht mit). Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `wirtschaft.md` beschreibt die Münzskala 2–30 mit Stufen und Ausnahmen; `zielkorridore.md` … |
| AC-02 | – | Glossar und `materialien-gebaeude.md` nennen Baumschule und Baumschul-Platz im Wald, freig … |
| AC-03 | – | `stufen.md`, `geplante-chunks.md`, `welten.md` und Glossar beschreiben Hafen-Chunk, Schiff … |
| AC-04 | – | Schiffskosten je Abschnitt (Münzen und Material) und der Zielkorridor „Schiff fertig“ steh … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-635 | eingeplant | AC-01 – | Münzpreise liegen auf einer kleinen Skala von 2 bis 30 Gold |
| B-636 | eingeplant | AC-02 – | Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen |
| B-637 | eingeplant | AC-03 – | Ein Hafen-Chunk auf Inselstufe 1 baut das Boot zum Verlassen der Insel |
| B-638 | verworfen | AC-04 – | Das Balancing misst die Ressourcen für das Boot |
