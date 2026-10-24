# B-612 · Balancing-Berichte lassen sich einzeln löschen und bewerten, der Vergleich erklärt sich selbst

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24) bei der Abnahme der Balancing-Seite: Berichte sollen einzeln löschbar und bewertbar sein (gut, schlecht usw.). Der Knopf „Als Basis“ ist unklar.

## Ziel

Jeder Bericht in der Liste hat eine Bewertung und lässt sich nach Rückfrage löschen; der Vergleich zweier Berichte ist auf der Seite erklärt.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

- Bewertung je Bericht: keine, gut, schlecht, unklar, Referenz; gespeichert in `reports/ratings.json` (überlebt Neustarts, Pfad als Schlüssel).
- Löschen je Bericht mit Bestätigung: JSON und gleichnamiges Markdown; ein Lauf-Ordner verschwindet nur, wenn er danach leer ist; die Bewertung fällt mit weg.
- Nur Pfade unter `reports/` (Prüfung wie beim Lesen).
- Vergleich: Knopf „Vergleichsbasis“ statt „Als Basis“, Hilfe-Popup und Hinweis, welcher Bericht A und welcher B ist.

## Nicht-Ziele

Mehrfachauswahl, Filter nach Bewertung, Papierkorb.

## Regeln und Einschränkungen

Atomic Design über `@webkit`; Löschen nur nach Rückfrage.

## Beispiele

Bericht „simtest-run-2“ → Bewertung „schlecht“; „Löschen“ → Rückfrage → Bericht weg.

## Ausnahme- und Fehlerfälle

Pfad außerhalb `reports/` oder unbekannte Bewertung → Fehler; Bericht fehlt → Fehler, Liste lädt neu.

## Akzeptanzkriterien

- **AC-01** Go-Test: Bewertung wird gespeichert und in der Liste geliefert; Löschen entfernt JSON, Markdown und Bewertung, lehnt fremde Pfade ab.
- **AC-02** Vitest: Bewertung setzen, Löschen mit Rückfrage, Vergleichsbasis mit Erklärung.

## Offene Fragen

keine. Default (Claude): Bewertungen keine/gut/schlecht/unklar/Referenz, Ablage `reports/ratings.json`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
