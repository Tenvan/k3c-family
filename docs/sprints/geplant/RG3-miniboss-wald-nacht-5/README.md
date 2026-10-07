# RG3 · REG · Miniboss Wald: Burg hält Nacht 5

- **Status:** geplant
- **Domäne:** REG
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-346
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Laut BR1.1 hält die Burg die Nächte 1–4 in 99 %, Nacht 5 aber nur in 4 %. In Welle 5 kommt der Miniboss Wald (`goblinLeader`). Die Grad-Kurve ist nicht monoton: Hart hält besser als Normal (B-346).

## Ziel

„Burg hält Nacht 1–5“ liegt je Grad im Korridor, der Miniboss bleibt eine spürbare Prüfung. Am Ende sichtbar: Beschluss in `docs/rules/bosse.md`, Werte in `data/bosses.json`, `task balance` zeigt Normal im Korridor 75–90 % und monotone Grad-Kurven.

## Beteiligte und Zielgruppen

Familie als Spieler; ein Agent misst mit dem Balancing-Tester; 🧑 beschließt die Werte.

## Anforderungen

B-346 › Anforderungen.

## Nicht-Ziele

Neue Mechaniken; Wirtschaftswerte (BR1); Bot-Umbau (BAL6); die übrigen Werte aus RG1 (Hub-Stufe 4–5, Adern-Takt).

## Regeln und Einschränkungen

- REG ändert nur Werte in `data/`, Begründung in `docs/rules/bosse.md`; Golden nach B-137 (`task golden:update` mit Begründung im Commit).
- Messen erst nach BAL6, denn ohne einen Bot, der Truppen und Türme baut, ist die Ursache für Nacht 5 nicht von fehlender Verteidigung zu trennen.
- RG1 (B-230) misst dieselbe Kennzahl „Burg hält Nacht 1–5“. Die Beschlüsse beider Sprints werden aufeinander abgestimmt, Werte nie doppelt ändern.
- Testläufe nur über `sim_test` (B-348).

## Beispiele

B-346 › Beispiele (Normal, 2 Spieler, Seed 1: Burg fällt in Nacht 5).

## Ausnahme- und Fehlerfälle

Korridor mit vertretbaren Boss-Werten nicht erreichbar → Korridor per Beschluss von 🧑 anpassen, nie still.

## Akzeptanzkriterien

- **AC-01** `task balance` zeigt „Burg hält Nacht 1–5“ im Korridor 75–90 % (Normal), die Grad-Kurven sind monoton (`B-346/AC-01`).

## Offene Fragen

- Gehört die Prüfung zu BR2 (Kampf und Bosse) oder davor? Mit dem eigenen Sprint vorerst davor, vor BR2. 🧑, nicht blockierend.
- Werte und Korridor entscheidet 🧑 im Workshop RG3.2.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- RG3.1 Messung und Ursache Nacht 5, Erklärung der Grad-Kurve, Wertvorschlag (AC-01).
- RG3.2 Workshop (🧑): Beschluss und Werte in `data/bosses.json`, letzte Session schließt ab (AC-01).

## Abnahme

–
