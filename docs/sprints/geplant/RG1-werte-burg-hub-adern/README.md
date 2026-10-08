# RG1 · REG · Werte-Runde: Burg hält Nacht, Hub-Stufe 4–5, Adern-Takt

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-230, B-287, B-289, B-346
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Die Burg hält Nacht 1–5 nur in 47 % der Seeds (B-230), Hub-Stufe 4 und 5 sind mit dem Beutel-Maximum nicht bezahlbar (B-287), der Aggressionspool steigt mit Adern zu schnell (B-289).

## Ziel

Burg, Hub-Ausbau und Wellen-Takt liegen im Zielkorridor, mit Beschluss von 🧑. Am Ende sichtbar: Beschlüsse in `docs/rules/`, geänderte Werte in `data/`, `task balance` grün.

## Beteiligte und Zielgruppen

🧑 beschließt; Agent misst mit dem Balancing-Tester.

## Anforderungen

B-230 › Anforderungen; B-287 › Anforderungen; B-289 › Anforderungen; B-346 › Anforderungen (aus RG3, PJ3).

## Nicht-Ziele

Neue Mechanik, Bot-Umbau über ein Profil hinaus.

## Regeln und Einschränkungen

REG ändert nur Werte; Beschluss je Wert in `docs/rules/`.

## Beispiele

Burg hält 47 % → Ursache Bot oder Werte, Beschluss, neue Messung im Korridor 75–90 %.

## Ausnahme- und Fehlerfälle

Korridor nicht erreichbar → Korridor per Beschluss anpassen, nie still.

## Akzeptanzkriterien

- **AC-01** Burg hält Nacht 1–5 nur in 47 % der Seeds, Ziel 75–90 % (B-230/AC-01).
- **AC-02** Hub-Stufe 4 und 5 sind mit dem Beutel-Maximum bezahlbar (B-287/AC-01).
- **AC-03** Der Aggressionspool steigt mit Adern nicht zu schnell (B-289/AC-01).
- **AC-04** `task balance` zeigt „Burg hält Nacht 1–5“ im Korridor 75–90 % (Normal), die Grad-Kurven sind monoton (B-346/AC-01; aus RG3 AC-01, PJ3).

## Offene Fragen

- Werte und Korridore entscheidet 🧑 im Workshop.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- RG1.1 Messung und Ursachen für Burg, Hub-Kosten, Adern-Takt; Messung und Ursache Nacht 5 (Miniboss Wald), Erklärung der Grad-Kurve, Wertvorschlag (AC-01, AC-02, AC-03, AC-04).
- RG1.2 Workshop (🧑): Beschlüsse und Werte in `data/` (auch `data/bosses.json`), letzte Session schließt ab (AC-01, AC-02, AC-03, AC-04).

## Abnahme

–
