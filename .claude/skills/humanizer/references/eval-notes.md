# Humanizer — Bewertungsnotizen

Manuelle Bewertungskriterien für die Eval-Prompts in `evals/evals.json`. Für Reviewer:innen, die Skill-Outputs prüfen.

## Was ein guter Humanizer-Output enthält

- **Drei Abschnitte:** „Entwurf", „KI-Audit", „Finale Fassung", plus „Änderungen"
- **Audit ist nicht leer:** Selbst beim besten Entwurf bleibt meist mindestens ein Tell — wenn das Audit sagt „nichts mehr", ist das verdächtig
- **Finale Fassung weicht vom Entwurf ab:** Sonst war das Audit Theater
- **Bedeutung erhalten:** Kernaussagen aus dem Original tauchen wieder auf, nicht nur Stil
- **Konkretisierung:** Vage Behauptungen werden mit echten Details, Zahlen, Quellen gefüllt — falls vorhanden. Wenn das Original keine echten Details enthält, schreibt ein guter Humanizer das transparent („das Original macht keine konkreten Angaben").

## Klassische Pseudo-Humanisierungen (rote Flagge)

- **Synonym-Tausch ohne Wirkung:** „entscheidend" → „wichtig" — beides immer noch generisch.
- **Em-Dash zu Halbgeviertstrich, sonst nichts:** Reine Typo-Korrektur ohne Strukturarbeit.
- **„Lassen Sie uns" → „Lass uns":** Tonlage geändert, Floskel bleibt.
- **Nur Liste in Fließtext:** Inline-Header-Liste wird zu drei Sätzen mit denselben Buzzwords.
- **Englische Anführungszeichen bleiben:** Skill behauptet, deutsch zu sein, lässt aber " stehen.

## Deutsch-spezifische Pflichtprüfungen

Vor Abnahme:

1. Anführungszeichen: konsistent „..." (nicht "..." oder gemischt)
2. Striche: – oder Komma, kein —
3. Komposita: KI-Tool, E-Mail, nicht „KI Tool"
4. Konjunktivkette: maximal eins pro Absatz
5. Sie/Du: nicht im selben Text gemischt

## Bewertungsschema (Kurzform)

| Kriterium | Gewicht | Schwelle |
|-----------|---------|----------|
| Bedeutung erhalten | 30 % | Kein Faktenverlust |
| Tells reduziert | 30 % | Mind. 70 % der Muster aus Original adressiert |
| Audit ehrlich | 15 % | Audit nennt verbleibende Schwächen |
| Stimme ergänzt | 15 % | Ich-Form, Meinung, Rhythmus dort wo passend |
| Deutsch-Schnellcheck | 10 % | Alle 5 Punkte erfüllt |

Skill ist „akzeptiert", wenn ≥ 80 % erreicht und kein Pflichtkriterium (Bedeutung, Deutsch-Schnellcheck) durchgefallen ist.
