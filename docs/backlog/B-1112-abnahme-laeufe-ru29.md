# B-1112 · Abnahme-Läufe RU29 (Race, Gen) und Prüfliste für Ralf, Lane zweit

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU29
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU29 (devtools-Port nach Rust) lief ohne Sim-, Balance- und Gen-Läufe und ohne `check:race` (Sperre Hauptrechner). **Lane: zweit** (Second Worker, B-742). Die Dev-Werkzeuge berühren weder Sim noch Balance noch Grafik; Sim und Balance sind daher voraussichtlich n/a.

## Ziel

Nachweis der Läufe und Prüfliste für Ralf. Zusätzlich offen aus dem Review: Chronik und Dev-Anzeige allokieren je Bild nur bei offenem Menü (`format!`, Klone in `drawcheats`, `drawdebug`, `values`, identisch zu Go; erst nach Messung ändern, RU43).

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

- **AC-01** `check:race` (Lane zweit) grün oder als n/a begründet.
- **AC-02** `gen:paper` samt Grafik-Golden unverändert (n/a, wenn keine Grafik berührt; danach `git clean -fdX public/grafik`, B-1108).
- **AC-03** Ralf prüft im Spiel, sobald B-1111 und RU30.3 geliefert sind: (1) Cheat-Menü öffnet mit F9 (Tastatur) und am Pad (View + Y), je Spieler ein eigenes Menü, das andere Spiel läuft weiter, B schließt, im Spiel ohne Menü ist B unbelegt; Cheats wirken erst mit B-1111; (2) Hilfe F1 zeigt alle Entwickler-Tasten; (3) Debug-Overlay F10 zeigt Formen und Werte in beiden Bildhälften; (4) bei `--shots` erscheint nichts davon; (5) im Log stehen die Chronik-Zeilen, beim Beenden liegt ein `native-session-….json` unter `reports/`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Review RU29: der Hinweistext im Cheat-Menü (`drawcheats.rs:19`) ist vermutlich breiter als die Box in der halben Split-Screen-Zelle (ungeprüft, von Go geerbt); im Spiel ansehen.
