# B-632 · Balancing-Berichte bringen Ampeln, Kurzauswertung, Tuning-Hinweise und einen interaktiven HTML-Bericht

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** TST
- **Erstellt:** 2026-10-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-25, Ralf im Chat

## Ausgangslage

Ralf (2026-10-25): Die Balancing-Berichte (`reports/simtest-run-N/balance-*.json|.md`) nennen je Ziel nur Wert, Grenzen und Status. Es fehlen eine schnelle Einordnung, Hinweise zum Tunen und eine Grundlage für den nächsten Testlauf. Gewünscht: Ampeln (gut, schlecht, wo tunen), Kurzauswertung, alles, was offline ohne KI auswertbar ist, dazu ein ausführlicher interaktiver HTML-Bericht im Look des Spiels (Spielgrafiken).

Bestand (Erkundung 2026-10-25): `Verdict.status` (`ok`, `knapp`, `verletzt`, `ungültig`, `internal/balance/evaluate.go`) ist schon eine Ampel je Ziel; `Figures`/`NightFigures` (`figures.go`) liefern Überleben, Gold- und Rohstoff-Bilanz und Leerlauf je Nacht, aber ohne Korridore (`docs/rules/zielkorridore.md` § 5); `failSeeds` je Ziel; Baseline `testdata/balance/baseline.json` mit `Compare`; Tuning (`tune.go`, `Tuning.Reason`, `proposalDiff`). Anzeige: `tools/k3c-workbench/frontend/src/balancing/ReportView.tsx`; MCP: `sim_test status` (`simrun/reports.go` `lines()`).

## Ziel

Jeder Balancing-Lauf liefert ohne KI eine verständliche Auswertung: Ampel je Ziel und je Kennzahl, eine Kurzauswertung in wenigen Sätzen, konkrete Tuning-Hinweise mit den Datenwerten, die man drehen kann, Vorschläge für den nächsten Lauf und einen eigenständigen, im Spiel-Look gestalteten interaktiven HTML-Bericht.

## Beteiligte und Zielgruppen

Ralf (liest Berichte, steuert Balancing), Agent DEV (sim_test-Läufe), spätere Workshops (BAL, RG).

## Anforderungen

1. **Auswertung (Go, `internal/balance/insights*.go`):** aus `Summary` (+ optional Baseline/voriger Lauf) eine `Insights`-Struktur: Ampel je Ziel (grün ok, gelb knapp, rot verletzt, grau ungültig) mit Abstand zur Grenze in % der Korridorbreite und Trend zur Baseline; Ampel je Kennzahl aus Richtwerten; Stärken (was gut lief), Schwächen (was schlecht lief), Tuning-Hinweise, Kurzauswertung (3 bis 6 Sätze, regelbasiert), Vorschlag nächster Lauf (Seeds zum Nachspielen aus `failSeeds`, Szenario, sinnvolle `--tune`-Parameter mit Schrittweite).
2. **Richtwerte für Kennzahlen und Hebel als Daten** (`data/balance-hints.json`, nicht im Code): je Kennzahl grün/gelb/rot-Schwellen (z. B. Leerlauf, Überlebensabfall von Nacht zu Nacht, Gold-Bilanz Tag/Nacht, Rohstoff-Überschuss/Mangel, erste Mauer/Werkstatt) und je Ziel die Hebel (Datenpfad, Richtung, typische Schrittweite, z. B. `castleHeld` zu niedrig → Mauer-HP, Gegnerzahl je Welle, Startgold). Begründung in `docs/rules/zielkorridore.md` § 5 als Startwerte, ohne Workshop-Beschluss.
3. **Markdown-Bericht** (`Summary.Markdown`) bekommt oben Kurzauswertung, Ampel-Übersicht, Stärken/Schwächen, Tuning-Hinweise und „Nächster Lauf“; die Kennzahlen je Nacht als Tabelle.
4. **HTML-Bericht** `balance-<Zeit>.html` neben JSON/MD: eine Datei, offline (kein CDN, Grafiken als eingebettete data-URIs), interaktiv (Diagramme Überleben/Gold/Rohstoffe/Leerlauf je Nacht als SVG mit Tooltips, Filter nach Ampel/Szenario/Grad, sortierbare Tabellen, aufklappbare Details je Ziel mit Seeds und Hebeln, Vergleich mit Baseline/vorigem Lauf, Kopierknöpfe für den nächsten `sim_test`-Aufruf), im Spiel-Look: Spielgrafiken aus `public/` (Burg, Mauer, Turm, Gegner, Münzen, Rohstoff-Symbole) und Palette/Schrift des Spiels, hell/dunkel.
5. **Workbench:** `ReportView` zeigt Kurzauswertung, Ampeln, Hinweise und „Nächster Lauf“; Knopf „HTML-Bericht öffnen“. MCP `sim_test status` nennt Kurzauswertung und den Pfad des HTML-Berichts.
6. Alte Berichte ohne neue Felder bleiben lesbar; Auswertung lässt sich aus einem vorhandenen `balance-*.json` nachträglich erzeugen (CLI-Flag oder Workbench-Aktion).

## Nicht-Ziele

KI-Auswertung, Workshop-Beschlüsse über Korridore der Kennzahlen (bleiben Startwerte bis RG/BAL), neue Messgrößen in der Simulation.

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-workbench`); Werte in `data/`, Workbench-Oberfläche nur über `@webkit` (Atomic Design); HTML-Bericht ohne externe Ressourcen; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen (HTML-Vorlage per `embed` aufteilen); Testläufe nur über `sim_test`.

## Beispiele

Lauf mit `castleHeld` normal 62 % (Korridor 75–90): rote Ampel, Abstand −87 % der Korridorbreite, Trend −10 Punkte zur Baseline; Hinweis: „Burg hält zu selten: Mauer-HP Stufe 1 (`buildings.json:wall.levels.0.hp`) +20 % oder Gegner je Welle (`waves.json:table.*`) −10 % probieren“; Nächster Lauf: Seeds 17, 42, 88 nachspielen, `--tune buildings.wall.levels.0.hp=300..420 step 30`.

## Ausnahme- und Fehlerfälle

Bericht ohne `kennzahlen` → Kennzahl-Ampeln grau, Hinweis im Bericht; keine Baseline → kein Trend; Grafik fehlt → Platzhalter-Symbol; ungültige Seeds → graue Ampel mit Anzahl.

## Akzeptanzkriterien

- **AC-01** `Insights` aus einem Summary: Ampel je Ziel und Kennzahl, Stärken, Schwächen, Tuning-Hinweise, Kurzauswertung, nächster Lauf (Go-Tests mit festen Berichten).
- **AC-02** Richtwerte und Hebel stehen in `data/balance-hints.json`, geprüft beim Laden, dokumentiert (`docs/data/`, `zielkorridore.md` § 5).
- **AC-03** Markdown-Bericht enthält Kurzauswertung, Ampeln, Hinweise und Kennzahlen (Test).
- **AC-04** HTML-Bericht entsteht je Lauf, ist eigenständig (keine externen URLs, Test), interaktiv und im Spiel-Look mit Spielgrafiken; im Browser geprüft (Screenshot).
- **AC-05** Workbench zeigt Auswertung und öffnet den HTML-Bericht; `sim_test status` nennt Kurzauswertung und HTML-Pfad (Tests).
- **AC-06** Alte Berichte bleiben lesbar; Auswertung für vorhandene Berichte nachträglich erzeugbar.

## Offene Fragen

keine; Richtwerte der Kennzahlen sind Startwerte, Ralf justiert sie später.

## Notizen

Vorgabe Ralf im Chat 2026-10-25 („Gib alles, was du rausholen kannst“, HTML mit Spielgrafiken aufpeppen); Erkundung der Bestandsstruktur siehe Ausgangslage. Ralf wollte zuerst einen Mock sehen (HTML-Bericht mit Daten aus `simtest-run-7` im Browser-Pane) und fragte, wohin die Kopierbefehle gehören: Jeder Befehl trägt jetzt ein Ziel (Chat an Claude, Terminal, Workbench › Balancing-Seite), in der Workbench gibt es zu Chat-Befehlen „In Claude öffnen“.

### Ergebnis (2026-10-25, Claude)

- **AC-01** `internal/balance/insights*.go`: `BuildInsights` liefert Ampel je Ziel (Abstand in % der Korridorbreite, Trend zur Baseline), Ampel je Kennzahl, Stärken, Schwächen, Tuning-Hinweise, Kurzauswertung (3–6 Sätze), nächsten Lauf (Seeds aus `failSeeds`, `--tune search`-Bereich ab dem Wert in `data/`, Befehle mit Ziel). Tests `TestInsightsAmpeln`, `TestInsightsTexte`, `TestInsightsKennzahlenUndNaechsterLauf`, `TestInsightsTrend`, `TestInsightsAllesGruen`, `TestHebelBereich` auf dem festen Bericht `testdata/insights-run7.json`.
- **AC-02** `data/balance-hints.json` (Richtwerte als Bänder, Hebel `low`/`high` je Ziel), geladen mit `DisallowUnknownFields` und Prüfung (`TestRichtwerteFehler`), jeder Hebel zeigt auf eine Zahl in `data/` (`TestRichtwerteLaden`). Doku `docs/data/balance-hints.md`, Startwerte mit Begründung in `docs/rules/zielkorridore.md` § 5.
- **AC-03** `balance-*.md` beginnt mit Kurzauswertung, Ampeln, Stärken/Schwächen, Tuning-Hinweisen, Nächstem Lauf und Kennzahlen je Nacht (`TestMarkdownMitAuswertung`).
- **AC-04** `balance-*.html` entsteht je Lauf neben JSON/MD: eine Datei, Vorlage `internal/balance/report/*` per embed, Spielgrafiken aus `public/` (Burg, Mauer, Turm, Lagerfeuer, Axt, Spitzhacke, Erz, Münze, Skelett, Goblin) als data-URIs, Palette der Landingpage, Nacht/Tag-Thema. Interaktiv: Filter nach Ampel/Grad/Szenario/Suche, sortierbare Tabellen, aufklappbare Ziele mit Seeds und Hebeln, vier SVG-Diagramme mit Tooltips, Baseline-Vergleich, Kopierknöpfe. `TestHTMLEigenstaendig` (keine externen URLs), `TestHTMLOhneGrafiken` (Platzhalter). Im Browser-Pane geprüft (Screenshots, keine Konsolenfehler, Filter/Sortierung/Details/Tooltips per Skript).
- **AC-05** Workbench: `ReportView` zeigt `InsightsView` (Ampeln, Kurzauswertung, Stärken/Schwächen, Hinweise, Nächster Lauf mit Kopieren und „In Claude öffnen“) und den Knopf „HTML-Bericht öffnen“ (RPCs `BalanceInsights`, `BalanceReportHTML`). `sim_test status` nennt die ersten drei Sätze der Kurzauswertung und den Pfad des HTML-Berichts (`TestStatusMitKurzauswertung`, `TestBalanceAuswertungUndHTML`, `InsightsView.test.tsx`).
- **AC-06** Alte Berichte bleiben lesbar (ohne `kennzahlen` graue Kennzahl-Ampeln mit Hinweis, `TestInsightsAlterBericht`); nachträglich: CLI `k3c-balance --insights reports/…/balance-<Zeit>.json` und „HTML-Bericht öffnen“ in der Workbench (`TestReevaluate`).
- Läufe aus der Workbench vergleichen jetzt mit `testdata/balance/baseline.json` (Trend). Die Baseline ist veraltet (B-625): alle Ziele erscheinen dort als „neu“.
- `task check:diff` grün, Review (Sonnet) ohne schweren Befund; behoben: `--insights` nur für `balance-*.json`.
