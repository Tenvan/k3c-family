# B-1194 · Ein Bürger mit Werkzeuggürtel ist Handwerker und macht alle Bauarbeiten: roden, Felsen hauen, bauen, reparieren

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute baut und repariert ein Bauer nur mit Hammer (Handwerker, B-950); Roden und Felsen hauen übernehmen Berufe wie Bergmann und Holzfäller. 🧑-Idee 2026-11-21: Ein Bürger mit Werkzeuggürtel wird zum Handwerker, und Handwerker machen alle Bauarbeiten: roden, Felsen hauen, bauen, reparieren und so weiter.

## Ziel

Der Werkzeuggürtel (B-1193) macht einen Bauern zum Handwerker; Handwerker erledigen alle Arbeiten am Gelände und an den Gebäuden: Bäume roden (Axt), Felsen und Steine hauen (Spitzhacke), bauen, ausbauen und reparieren (Hammer).

## Beteiligte und Zielgruppen

🧑 entscheidet über Arbeitsverteilung; Spieler sehen Handwerker mit Werkzeug je Arbeit.

## Anforderungen

- Ein Bauer mit Gürtel nimmt Aufträge an: Baustelle, Reparatur, Rodung, Fels; Reihenfolge und Zuteilung deterministisch (kein Zufall außer `w.rng`).
- Ein Bauer ohne Gürtel arbeitet nicht an Gebäuden und Gelände (heute: ohne Hammer baut er nie).
- Berufe Bergmann und Baumeister (Ausbildung 20 Gold) sowie die Holzfäller-Arbeit gehen im Handwerker auf oder werden zu Tempo-Boni des Handwerkers; Entscheidung in den Offenen Fragen.
- Fällt der Handwerker, bleibt der Gürtel liegen; ein Bauer ohne Gürtel holt ihn (nur ohne Gegner in 8 Units, B-929).
- 2+ Spieler, Spielstand-Version erhöhen, Regeltexte und Glossar.

## Nicht-Ziele

Den Gürtel herstellen (B-1193), Reparaturkosten (B-1195), Erzadern freiräumen (B-1198).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein Bauer mit Gürtel wird Handwerker und erledigt Bau, Reparatur, Rodung und Fels (je ein Test in `k3c-sim`).
- **AC-02** Ohne Gürtel keine dieser Arbeiten.
- **AC-03** Fällt ein Handwerker, wird der Gürtel von einem Bauern wieder aufgehoben (Test).
- **AC-04** Anzeige: Handwerker trägt sichtbar das Werkzeug der Arbeit (Prüfung 🧑 am PC).

## Offene Fragen

Defaults (autonom entscheidbar): Bergmann und Baumeister entfallen als Berufe, ihre Boni werden Ausbaustufen der Werkstatt; ein Gürtel gilt für alle drei Werkzeuge gleichzeitig; Holzfällen bleibt beim Bauern ohne Gürtel (Gold aus Holz, WI1), nur Roden (Baum entfernen für Bauplatz) braucht den Gürtel. 🧑 bestätigt bei der Freigabe.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
