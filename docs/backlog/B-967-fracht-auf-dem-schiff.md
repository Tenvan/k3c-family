# B-967 · Vor der Abfahrt wählen die Spieler die Fracht des Schiffs

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** SI1
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Heute beginnt die neue Insel nach dem Inselwechsel mit dem Startvorrat von 100 Holz (`docs/rules/stufen.md` § 1, B-177); eine Wahl, was mitfährt, gibt es nicht.

## Ziel

Vor der Abfahrt wählen die Spieler, was mitfährt: ein paar Truppen, Münzen oder Material. Der Inselwechsel wird zur Entscheidung statt zum Neustart bei null.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Laderaum mit Plätzen (🧑 2026-11-15):** Das Schiff hat wenige Plätze (Startwert offen, z. B. 6). Jeder Platz nimmt eine Truppe, einen Beutel Münzen oder eine Kiste Material; die Spieler entscheiden die Mischung.
- **Beladen in der Welt am Steg (🧑 2026-11-15):** Kein Menü. Am Steg stehen Ladeplätze: Münzen einwerfen lädt einen Beutel, Truppen gehen per Befehl an Bord, Bauern tragen Material aus dem Vorrat. Beide Spieler laden gleichzeitig, die Fracht ist an Deck sichtbar.
- **Mehr Plätze durch besseres Schiff (🧑 2026-11-15):** Der Startvorrat der neuen Insel bleibt. Das Grundschiff (Kiel, Rumpf, Segel) hat wenige Plätze; ein vierter, freiwilliger Bauabschnitt „Laderaum“ vergrößert es.
- Die Fracht kommt auf der neuen Insel zum Startvorrat hinzu; Zurückgelassenes bleibt auf der alten Insel (keine Rückkehr).
- Funktioniert mit 2 Spielern gleichzeitig.

## Nicht-Ziele

Lade-Menü oder Overlay, automatische Beladung nach Profil, Wegfall des Startvorrats.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/stufen.md`, `docs/rules/wirtschaft.md`, `engine/sim/**`, `data/**`, `native/k3c-game/**` (vorläufig)
- **Abhängig von:** B-961
- **Braucht 🧑:** TV-Abnahme
- **Größe:** L (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt Spielregeln, Spielstand und Schiffskosten; Werte (Plätze, Größe von Beutel und Kiste, Kosten des Laderaums) später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925).

## Beispiele

Grundschiff mit 6 Plätzen: 3 Schützen, 2 Beutel Münzen, 1 Kiste Holz → Insel 2 beginnt mit 100 Holz Startvorrat plus dieser Fracht. Mit Laderaum passen mehr Plätze an Bord.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Wie viele Plätze haben Grundschiff und Laderaum, was kostet der Laderaum?
- Wie viel fasst ein Beutel Münzen und eine Kiste Material?
- Fahren die Monarchen-Münzen im eigenen Beutel ohnehin mit (heutiges Verhalten ungeprüft)?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
