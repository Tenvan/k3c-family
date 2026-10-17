# B-451 · GEB6: Abnahme von Vorschau und Spiel durch Ralf

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB6 ist umgesetzt und geprüft (GEB6.1–GEB6.5, Review GEB6.6 ohne schwere Befunde) und mit PR #319 und #323 auf `develop`. AC-03 verlangt die Abnahme von Natur und Objekten (Bäume, Busch, Fels, Erzbrocken, Adern, Truhe, Portal, Ausgang, Lager) in Vorschau und Spiel durch 🧑; Agenten nehmen nicht ab. Die Abnahmeliste steht in `GEB6.6-review-abnahme.md` › Ergebnis (Punkte 1–15; Punkt 13 ist bestätigt).

## Ziel

🧑 hat die Objekte gesehen und bestätigt oder Nachbesserungen benannt; GEB6.6 kann `fertig` werden und der Sprint nach `erledigt/`.

## Beteiligte und Zielgruppen

🧑 (Ralf) nimmt ab; Claude setzt danach Status und Ablage nach.

## Anforderungen

- Vorschau `assetvorschau.html?szene=gebaeude` und das Spiel (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle, zwei Spieler) nach der Abnahmeliste in GEB6.6.

## Nicht-Ziele

Animationen von Portal und Feuer (Nicht-Ziel GEB6), GEB7 (Hintergründe), TV-Test.

## Regeln und Einschränkungen

Arbeitsweise › SDD: Abnahme gehört 🧑; Browser-Prüfung durch einen Agenten nur mit Freigabe von 🧑 für diesen Lauf.

## Beispiele

🧑 schreibt „abgenommen“ → GEB6.6 `fertig`, Sprint nach `erledigt/`. 🧑 nennt Nachbesserungen (z. B. Mine-Fels dunkler) → eine Session im Sprint oder ein Ticket je Punkt.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Entscheidungsfrage, kein Code.

## Akzeptanzkriterien

- **AC-01** 🧑 bestätigt GEB6 AC-03 (Abnahmeliste Punkte 1–12, 14 und 15 in `GEB6.6-review-abnahme.md`) oder nennt Nachbesserungen.

## Offene Fragen

Sind Maße, Looks und Geschmacksfragen (Mine-Fels, Feuerstelle, Ausgang, Kristall-Ader) so abgenommen? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
