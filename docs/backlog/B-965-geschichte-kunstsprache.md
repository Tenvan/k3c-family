# B-965 · Jede Welt und Insel erzählt eine kurze Geschichte in Kunstsprache mit eingestreuten echten Wörtern

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-15: Welten und Inseln haben keine Geschichte. Ausformulierte Texte liest niemand, wenn es zu viel wird.

## Ziel

Jede Welt und jede Insel hat eine kleine Geschichte mit den Themen Start, Etappen und Endgegner. Sie steht in einer Kunstsprache aus wenigen Wörtern; kurze echte Wörter im Satz deuten die Handlung nur an.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Geschichte je Welt und je Insel: Start, Etappen, Endgegner.
- Kunstsprache mit kleinem Wortschatz, dazwischen 2 bis 3 echte Wörter je Zeile als Andeutung.
- **Machart (🧑 2026-11-15): je Welt eigener Klang.** Jede Welt hat eigene Laute und einen eigenen kleinen Wortschatz (Sengoku japanisch, Nil ägyptisch, Polar nordisch); die echten Wörter tragen den Sinn.
- **Anzeige (🧑 2026-11-15): Einblendung an Wendepunkten.** Ein bis zwei Zeilen blenden kurz ein und wieder aus: bei Ankunft auf der Insel, beim Abschluss einer Etappe, vor dem Endgegner. Kein Tastendruck, das Spiel läuft weiter.
- **Entstehung (🧑 2026-11-15): Claude schreibt, 🧑 nimmt ab.** Je Welt ein kleiner Wortschatz und feste Zeilen in der Welt-Datei, geschrieben über die Welten-MCP-Tools; „Welt anlegen“ erzeugt die Geschichte gleich mit.
- Kurz: wenige Zeilen, nichts zum Durchlesen.

## Nicht-Ziele

Ausformulierte Texte, Dialoge, Vertonung (vorerst).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Sengoku: „Oru kamae KRONE no sen. Shiro nu BRENNT.“
Nil: „Akh'et ra KRONE sen-hotep. Ma'u FLUT kha, kha SAND.“
Polar: „Hjørn vald KRONE ok ís. Skal EIS brjóta – VOLK ska.“

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Einblendung im Split-Screen: je Spielerhälfte oder über die ganze Breite?

## Notizen

Idee aus der Ideenrunde k3c-07 (2026-11-15), Grundzüge mit 🧑 geklärt; Akzeptanzkriterien, Bereich und Größe fehlen noch, Spec bleibt Entwurf. Berührt das Format der Welt-Datei und die Etappen aus B-961.
