# B-1068 · Abnahme RU20: Sichtprüfung HUD und Gen-Läufe

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU20
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU20 hat das HUD (Anzeigen je Spielerhälfte, Tasten-Icons, Banner) nach Rust portiert. Sim-, Balance-, Gen- und Race-Läufe sowie die Sichtprüfung sind in der Sprint-Zeit gesperrt (B-742).

## Ziel

Sprint RU20 ist nach Gen-Läufen und Sichtprüfung am PC mit Pad abgenommen.

## Beteiligte und Zielgruppen

Claude führt die Läufe, Ralf prüft im Spiel.

## Anforderungen

- **Claude:** `task check:race` (lokal ohne gcc übersprungen; als übersprungen nennen) und Bildtests mit `K3C_GRAFIK_DIR=C:\WORKSPACE\TENVAN\k3c\public\grafik` und `K3C_STILL_REQUIRE=1` (`cargo test -p k3c-game still_hud`).
- **Claude:** Befunde für RU23 prüfen: `RefCell`-Doppelausleihe in `draw_cell` gegen `Hud::icon` (B-1067), `unimplemented!` bei offenem Menü oder Seekarte in `draw.rs`, `ignore`-Tests von RU23 (`menu_footer_per_device`, `visible_labels_budget_after_culling`, `skill_menu_geldsack_linie`, `bedienhinweis_menu_und_seekarte`, `glyphen_mit_icons`) und RU10/RU18 (`purse_groesse_und_obergrenze_mit_skills`).
- **Ralf:** Im Spiel prüfen: HUD beider Spielerhälften (Münzen, Krone, Tageszeit), Pad- und Tastatur-Hinweise (nur Xbox-Symbole und Tastenkappen), Banner, Text 2 bis 3 px tiefer als in Go (B-1063, B-1065), Sichtprüfung `glyph_shot_test.go` (nicht portiert).
- **Claude (Klasse A):** Abweichung Monarch-Daten festhalten: Ein Skill-Wert mit falschem Typ in `monarch.json` leert in Rust die ganze Datei, Go überspringt nur den Eintrag; angleichen oder dokumentieren.

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

- **AC-01** Bildtests mit `K3C_STILL_REQUIRE=1` laufen grün (nicht übersprungen).
- **AC-02** Ralf bestätigt die Sichtprüfung der Anzeigen und Tasten-Icons am PC mit Pad.
- **AC-03** Die Monarch-Daten-Abweichung ist behoben oder dokumentiert.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
