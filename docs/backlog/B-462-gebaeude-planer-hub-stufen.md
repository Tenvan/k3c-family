# B-462 · Ein Gebäude-Planer bearbeitet, welches Gebäude ab welcher Hub-Stufe verfügbar ist

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ9
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Welches Gebäude ab welcher Hub-Stufe verfügbar ist, steht in `data/hub.json`: `sites` (je Platz `kind`, `offsetUnits`, `hubLevel`, `fromDepth`, `needsDeeper`), `islandSites`, `wallLines` (Linie k ab Hub-Stufe k, Tor ab `gateHubLevel`) und `merchant` (`onlyDepth`); die Regel dazu in `docs/rules/materialien-gebaeude.md` § 3 (Tabelle Hub-Stufe → Gebäude). Bearbeitet wird von Hand; Regel und Daten können auseinanderlaufen. B-458 (Hub wächst kleinteiliger) und GEB9 (Ausbaustufen) brauchen genau diese Zuordnung. Die Werte sind Domäne REG; jede Änderung kann Sim-Goldens verschieben.

## Ziel

Ein Gebäude-Planer im Asset-Werkzeug zeigt und bearbeitet die Freischaltung: je Hub-Stufe die verfügbaren Gebäude mit Platz (`offsetUnits`), Tiefe (`fromDepth`) und Bedingungen; er schreibt `data/hub.json` gültig zurück und zeigt das Ergebnis in der Live-Vorschau (B-460).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Kosten, Bauzeiten und Werte der Gebäude (`data/buildings.json`, REG); neue Gebäude-Arten; mehr Hub-Stufen (B-458) – der Planer soll sie aber ohne Umbau aufnehmen können.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Planer zeigt eine Matrix Hub-Stufe × Gebäude aus `data/hub.json` (`sites`, `islandSites`, Mauerlinien und Tor aus `wallLines`, Händler), inklusive Platz und Tiefe.
- **AC-02** Änderungen (Stufe, Platz, Tiefe) werden geprüft (Platz frei, Abstand zu Zahlzielen wie in `engine/sim` geprüft, Stufe im Bereich) und erst dann in `data/hub.json` geschrieben; Format und Reihenfolge der Datei bleiben stabil.
- **AC-03** Abweichungen zwischen `data/hub.json` und der Tabelle in `materialien-gebaeude.md` § 3 werden angezeigt.
- **AC-04** Die Live-Vorschau (B-460) zeigt die geänderte Freischaltung je Hub-Stufe.
- **AC-05** Der Planer weist darauf hin, dass eine gespeicherte Änderung eine REG-Änderung ist: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung.

## Offene Fragen

1. Die Platz-Prüfung lebt in Go (`engine/sim`, `data`-Validierung): ruft der Planer sie über das Go-Backend auf (Annahme) oder prüft er selbst?
2. Ist ein Schreiben von `data/hub.json` aus dem Werkzeug erlaubt (Werte = Domäne REG)? Annahme: ja, aber nur Freischaltung und Platz, Werte bleiben REG.
3. Soll die Regeltabelle in `materialien-gebaeude.md` künftig aus der JSON erzeugt werden?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
