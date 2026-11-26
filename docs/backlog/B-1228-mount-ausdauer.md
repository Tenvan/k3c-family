# B-1228 · Reittiere haben Ausdauer: Sprint verbraucht sie, Pausen laden sie auf

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** AU1
- **Projekt:** NAC
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute sprintet jedes Reittier ohne Dauer (`docs/rules/monarch.md` § 7: „das Kamel sprintet nicht länger, es sprintet schneller; eine Sprint-Ausdauer für alle Reittiere wäre eine eigene Regel, Entscheidung 🧑 offen“). Der Galopp (Pad RT) kostet nichts. Auftrag Ralf 2026-11-22: Die Ausdauer-Funktion für das Mount fehlt noch und wird eingeplant.

## Ziel

Jedes Reittier hat eine Ausdauer: Galopp (Sprint) verbraucht sie, ohne Galopp lädt sie wieder auf, leer endet der Galopp bis zu einem Mindestwert. Die Stärken der Tiere (Sengoku-Pferd, Kamel) können darüber wirken, zum Beispiel längerer Galopp.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Entwurf, offen: Ausdauer-Leiste je Spieler, Verbrauch pro Sekunde Galopp, Aufladung pro Sekunde ohne Galopp, Mindestwert zum erneuten Galopp.
- Werte je Reittier in `data/mounts.json` (vorläufig, Balancing später), Faktoren pro Tier.
- Logik und Test in `rust/crates/k3c-sim` (Entscheidung 013), das Spiel zeichnet nur eine kleine Anzeige am Monarchen.
- Gilt mit 2 Spielern gleichzeitig, bleibt bei Stufen- und Inselwechsel, Speichern und Laden (Spielstand-Format prüfen, Sperre `spielstand`).
- Regeltext `docs/rules/monarch.md` § 7 nachziehen.

## Nicht-Ziele

Ausdauer für andere Aktionen (Schlag, Skills), Nachteile je Reittier, Balancing der Werte (Abnahme-Ticket).

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

- **AC-01** Galopp verbraucht Ausdauer, ohne Galopp lädt sie auf, leer endet der Galopp (Test in `k3c-sim`).
- **AC-02** Werte je Reittier aus `data/mounts.json`, Faktoren ≥ 0.
- **AC-03** Anzeige am Monarchen, 2 Spieler getrennt; Speichern und Laden erhalten den Wert.
- **AC-04** `task check:diff` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
