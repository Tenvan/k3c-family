# GS1.1 · Regeltext: Geschichte in der Welt-Datei und Wendepunkte (B-965)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs1
- **Abhängig von:** –
- **Tickets:** B-965
- **Kriterien:** AC-01

## Ziel

`docs/rules/welten.md` beschreibt das Feld `story` der Welt und der Insel, seine Prüfung und die Wendepunkte; das Glossar führt die neuen Begriffe.

## Kontext

Spec: `docs/backlog/B-965-geschichte-kunstsprache.md` (Anforderungen R1–R8, Beispiele, Offene Fragen mit Defaults). Regelwerk: `docs/rules/welten.md` § 2 (Format, Tabellen Welt und Insel), § 3 (Prüfung beim Laden, Meldungen), § 6.1 (Etappen, B-961). Begriffe: `docs/glossar.md` (Etappe, Endboss, Inselwechsel stehen schon). Die Regel-Tabellen nennen je Regel Begründung und Daten; Quelle der Entscheidung ist 🧑 2026-11-15 (Ideenrunde), Defaults sind als „vorläufig“ zu kennzeichnen. Vorhandene Welten: `standard`, `sengoku`, `nil`, `britannien`; „Polar“ gibt es nicht. Kein Code in dieser Session.

## Erlaubte Dateien

- `docs/rules/welten.md`

## Nicht-Ziele

Code, Welt-Dateien, Texte der Geschichten (B-976), Flaschenpost-Regel (GS4), `docs/rules/stufen.md`.

## Schritte

1. § 2: Feld `story` in die Tabellen Welt (`lexicon`, `start`, `etappen`, `end`, `bottles`) und Insel (`arrival`, `boss`, `done`) eintragen, Zeilenregel (1–2 Zeilen, echte Wörter groß, 2–3 je Zeile), Beispiel aus B-965.
2. § 3: Prüfungen und Meldungen (B-965/R4) ergänzen, Obergrenzen mit Verweis auf `data/story.json`.
3. Neuer Abschnitt § 6.2 „Geschichte an Wendepunkten“: Wendepunkte, Reihenfolge `done` → Etappe bzw. `end`, kein Auslösen beim Laden, Beispiel Etappen 1, 2, 2.
4. § 9 Offen: Defaults aus B-965 als vorläufig vermerken.
5. Glossar-Einträge Geschichte, Kunstsprache, Wortschatz, Wendepunkt, Abschlusszeile im letzten Commit des Sprints (Sammeldatei, `docs/arbeitsweise.md` › Sammeldateien); hier nur als Text im Ergebnis vorbereiten.

## Fertig, wenn

- [ ] AC-01: `docs/rules/welten.md` nennt Felder, Prüfungen mit Meldungen und Wendepunkte mit Beispiel (B-965/AC-01); Glossar-Texte stehen im Ergebnis bereit.
- [ ] `check_run task:check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: `docs/rules/welten.md` § 2, § 2.2, § 3, § 6.2, § 9 (Feld `story`, Prüfungen mit Meldungen, Wendepunkte). `task check:diff` grün. Commit f99dc3a00 auf `sprint/gs1`. Glossar (Abschlusszeile, Geschichte, Kunstsprache, Wendepunkt, Wortschatz) folgt im letzten Commit des Sprints. Festlegungen (vorläufig): Meldung mit Datei-Präfix, Länge in Zeichen, leere Liste abgelehnt, Etappen-Abschluss zählt ab Endboss-Sieg.
