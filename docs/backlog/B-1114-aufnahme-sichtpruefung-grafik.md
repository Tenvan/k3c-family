# B-1114 · Aufnahme-Modus im Rust-Spiel mit echter Grafik gegen Go prüfen und Speicher von ShotCanvas messen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU30.2 portiert den Aufnahme-Modus (`--shots`) nach Rust. Die Plattform kann keine Pixel auslesen, darum zeichnet `ShotCanvas` (game/shots_canvas.rs) gegen den Software-Rasterer aus still/. Der Rasterer implementiert jede Methode von `Canvas`; die Tests laufen ohne gerenderte Grafik (gen:paper war nicht erlaubt). `ShotCanvas` behält alle Uploads und lädt sie je Bild neu hoch (Speicher doppelt, Kopie je Bild). `--load` ist nur über `shot_saves` getestet, nicht als Lauf.

## Ziel

Der Aufnahme-Modus ist mit echter Grafik gegen Go geprüft, Speicher und Zeit von `ShotCanvas` sind gemessen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

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

- **AC-01** Mit gerenderter Grafik (task gen:paper) laufen `--shots` mit `--seed`, `--menu` (alle sieben Seiten), `--flow`, `--at` und `--stall` in Rust; Sichtvergleich mit den Go-Bildern, Abweichungen als Tickets.
- **AC-02** Ein Lauf `--shots` mit `--load <Spielstand>` schreibt Bilder (Station fehlt als Test).
- **AC-03** Spitzenspeicher und Zeit je Bild von `ShotCanvas::next_image` sind gemessen; bei Bedarf Upload nur der genutzten Texturen (PERF).
- **AC-04** Eine Messung der Allokationen je Takt der Schiffsfahrt (`sail`, `arrive`) steht in tests/alloc_je_bild.rs (RU30.2 prüft nur die Puffer-Kapazität).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
