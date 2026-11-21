# B-1196 · In den unteren Ebenen stehen nur Verteidigungsanlagen, produzierende Gebäude nur ganz oben

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute darf auf jeder Stufe gebaut werden, was die Hub-Stufe freigibt (`data/hub.json`, `buildings.json`). 🧑-Idee 2026-11-21: In den unteren Minen nur Verteidigungsanlagen und alles, was zum Kampf gehört; Rüstungsschmiede und andere produzierende Gebäude nur ganz oben.

## Ziel

Je Gebäudeart ist festgelegt, auf welchen Ebenen sie stehen darf: Verteidigung und Kampf (Mauer, Turm, Kaserne, Bogner …) auch unten; Produktion und Wirtschaft (Rüstkammer, Schmiede, Werkstatt, Lager, Farm …) nur ganz oben.

## Beteiligte und Zielgruppen

🧑 entscheidet über die Zuordnung je Art; Spieler sehen unten nur erlaubte Bauplätze.

## Anforderungen

- Feld je Art in `data/buildings.json` (z. B. `depths: top | all`); Bauplätze unerlaubter Arten entstehen unten gar nicht (Level-Generator Rust, deterministisch).
- „Ganz oben“ = Tiefe 0 des Hubs der Insel (Stufe 1).
- Hub-Plan und Regeltexte (`materialien-gebaeude.md`, `stufen.md`) ziehen mit; Prüfung `validate_hub_data` kennt die Regel.
- Spielstand: vorhandene Plätze unerlaubter Arten unten (alter Stand) verschwinden oder bleiben; 🧑 2026-11-21: alte Stände egal.

## Nicht-Ziele

Ausrüstung beschaffen (B-1197), Erzadern (B-1198), Balancing.

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

- **AC-01** Unten (Tiefe ≥ 1) entstehen nur Plätze der erlaubten Arten, oben alle (Test über alle festen Welten).
- **AC-02** Die Zuordnung steht in den Daten, nicht im Code.
- **AC-03** Regeltexte und Prüfung angepasst.

## Offene Fragen

Defaults (autonom entscheidbar): Kampf = Mauer, Turm, Tor, Kaserne, Bogner, Heilplatz; Produktion = Werkstatt, Schmiede, Rüstkammer, Lager, Farm, Taverne, Baumschule. 🧑 bestätigt die Zuordnung bei der Freigabe.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
