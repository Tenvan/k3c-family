# B-056 · Die Ratsche zieht gesunkene Werte automatisch nach

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/arbeitsweise.md` › Komplexitäts-Budget: Ausnahmen stehen mit ihrem heutigen Wert in einer Liste, „der Wert darf
nur sinken“. Die Prüfungen aus SP01 verhindern nur das Steigen: Sinkt eine Datei in `tests/complexity-baseline.json`
von 319 auf 310 Zeilen, bleibt der Eintrag 319 gültig, bis die Datei ≤ 300 Zeilen hat (SP01.3 › Schritt 2: „Wert ≥
aktueller Länge“). Genauso bleibt eine Komplexitäts-Ausnahme in `.oxlintrc.json` stehen, wenn die Datei besser wird.
Neuer Code in derselben Datei darf danach wieder bis zum alten Wert wachsen. Im Review SP01.4 stimmten alle Werte
noch genau mit den Messungen überein.

## Ziel

Die Ratsche zieht gesunkene Werte automatisch nach. Nutzen: Eine Verbesserung bleibt erhalten, statt als Spielraum
für neuen Code zu dienen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Bestandsdateien verkleinern (B-018, B-034); Review-Session.

## Anforderungen

- `npm test` scheitert, wenn ein Baseline-Eintrag größer ist als die aktuelle Zeilenzahl, mit Hinweis auf den neuen Wert.
- Für die Oxlint-Ausnahmen gibt es eine gleichwertige Prüfung oder eine Regel im Review, die sie nachzieht.

## Nicht-Ziele

Neue Werkzeuge; Bestandsdateien verkleinern (B-018, B-034).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

`src/landing/landing.ts` sinkt auf 310 Zeilen, Baseline sagt 319 → `npm test` scheitert mit „Wert auf 310 senken“.

## Ausnahme- und Fehlerfälle

Zeilenenden (CRLF/LF) ändern die Zählung nicht (heute wie `wc -l`).

## Akzeptanzkriterien

- **AC-01** Ein Baseline-Wert über der aktuellen Zeilenzahl lässt `npm test` scheitern (Probe, zurücknehmen).
- **AC-02** Eine Oxlint-Ausnahme über dem gemessenen Wert fällt spätestens im Review auf (Prüfung oder Regel in `docs/arbeitsweise.md`).

## Offene Fragen

Automatisch für Oxlint (Skript misst und vergleicht) oder nur als Review-Regel? (🧑)

## Notizen

Gefunden im Review SP01.4. Verworfen 2026-09-30 (🧑, Chat): Das Review wurde entschärft, es gelten nur noch die harten Grenzen aus `docs/arbeitsweise.md` › Komplexitäts-Budget; der Zielwert 300 Zeilen und die Baseline-Ratsche entfallen.
