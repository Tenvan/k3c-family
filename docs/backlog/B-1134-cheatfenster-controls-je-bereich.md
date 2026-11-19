# B-1134 · Cheat-Fenster: Controls je Bereich (Welt und Zeit, Wirtschaft, Spieler und Skills, Gegner und Welle, Anzeige)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU56
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Cheat- und Debug-Funktionen des Rust-Spiels stehen in `rust/crates/k3c-game/src/devtools/cheatlist.rs` (Gruppen Vorrat, Ausbau, Zeit, Grad, Kampf, Bosse aus `data/bosses.json`) und in den F-Tasten (`view/devkeys.rs`: F1 Hilfe, F2 Leistung, F3 Fluss, F5 Speichern, F6 Laden, F7 Pause, F8 Zeitraffer, F9 Cheats, F10 Debug, F11 Vollbild, F12 Screenshot). Das Cheat-Fenster (RU56) bekommt diese Funktionen als Controls.

## Ziel

Alle Funktionen der bestehenden Dev-Menüs sind im Cheat-Fenster als Controls erreichbar, nach Bereichen auf fünf Reitern, und lösen im Spiel dieselben Cheat-Pfade aus; dazu die Auswahl von Welt, Insel und Stufe.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI). Nutzer Ralf und Tester, die Szenen gezielt aufbauen. Fachprüfung `k3c-hud`.

## Anforderungen

Jeder Bereich besteht aus einer Datei im Fenster (`k3c-devwin/src/pages/<bereich>.rs`) und einer im Spiel (`k3c-game/src/devwin/<bereich>.rs`); die Registry steht seit RU56.3 und RU56.5.
- **Welt und Zeit:** Auswahl Welt, Insel, Stufe (Listen aus dem `catalog`) mit "Wechseln" (`goto`); Tageszeit Tag, Abend, Nacht, Morgen; "Nächste Tageszeit"; Zeitraffer 1, 2, 4, 8×; Pause und Einzelschritt; Grad Leicht, Normal, Hart, Ultra; Speichern und Neu laden.
- **Wirtschaft:** Gold +10, +100, +500 und freier Betrag; Ressourcen Holz, Stein, Kupfer, Eisen, Kristall je +50 und freier Betrag; Lager-Maximum +100; alle Ressourcen füllen; alles ausbauen auf Stufe 1 bis 5; Burgfall.
- **Spieler und Skills:** Zielspieler wählen (nach Spielerzahl aus dem `state`); Skills lernen und zurücksetzen, Skillpunkte +n (soweit es dazu Dev-Aktionen in `k3c-sim/src/dev/` gibt; fehlt eine, `ok:false` und Ticket); Spielerzahl wird nur angezeigt (Wechsel während des Spiels ist Folgeticket; Default).
- **Gegner und Welle:** Welle jetzt; Nacht sofort; Boss auslösen (Liste aus `data/bosses.json`) und nächste Boss-Phase; Ereignisse Vollmond, Blutmond, Händler-Überfall; Gegner und Truppen erzeugen (`spawn`, Art und Anzahl).
- **Anzeige:** Debug-Overlay aus, Geometrie, Werte; Leistungsanzeige; Fluss-Fassung; Tastenübersicht; Vollbild; Screenshot.
- Controls zeigen den Zustand (Umschalter gedrückt, Listen vorbelegt, Zeitraffer als Auswahl) aus dem `state`; nach jedem Befehl eine kurze Rückmeldung (`ack`, bei Fehler rot in der Statuszeile).

## Nicht-Ziele

Neue Dev-Aktionen in der Sim, Entfernen der Menüs im Spiel (B-1136), Umschalter für das Look-Profil (nach RU55 als Folgeticket).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-devwin/src/pages/**`, `rust/crates/k3c-game/src/devwin/world.rs`, `rust/crates/k3c-game/src/devwin/eco.rs`, `rust/crates/k3c-game/src/devwin/players.rs`, `rust/crates/k3c-game/src/devwin/foes.rs`, `rust/crates/k3c-game/src/devwin/view.rs`
- **Abhängig von:** B-1132, B-1133
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Fenster-Dateien und Spiel-Dateien je Bereich sind getrennt, damit die fünf Sessions RU56.6 bis RU56.10 parallel laufen können. Befehle nur über die Cheat-Pfade (`DevSession::dev`, `DevCmd`), nie an der Sim vorbei.

## Beispiele

Reiter Welt und Zeit: Welt "Wald", Insel 2, Stufe 1 wählen, "Wechseln"; dann "Nacht" und "4×" → Spiel springt dorthin, ist Nacht und läuft mit vierfacher Geschwindigkeit. Reiter Gegner: "Boss: Drache" → Boss erscheint.

## Ausnahme- und Fehlerfälle

Befehl ohne Verbindung: Control ausgegraut. `ok:false` vom Spiel: Fehlertext in der Statuszeile, Spiel unverändert. Liste leer (Katalog fehlt): Control bleibt deaktiviert.

## Akzeptanzkriterien

- **AC-01** Welt und Zeit: Test je Control, dass der richtige `dev`-, `ui`- oder `goto`-Befehl entsteht und im Spiel die gleiche DevAction auslöst wie der F9-Eintrag (soweit vorhanden).
- **AC-02** Wirtschaft: dito für alle Einträge der Gruppen Vorrat und Ausbau.
- **AC-03** Spieler und Skills: dito; fehlende Dev-Aktionen als Tickets vermerkt.
- **AC-04** Gegner und Welle: dito für Welle, Nacht, Bosse, Ereignisse, Spawn.
- **AC-05** Anzeige: dito für alle F-Tasten-Funktionen mit Zustand (Debug-Seite, Leistung, Fluss).
- **AC-06** Am PC (🧑): Szene "Nacht, Boss, 4×" in unter einer Minute aufgebaut; Controls verständlich.

## Offene Fragen

keine

## Notizen

Entschieden mit Default (autonom, Claude, 2026-11-19): Spielerzahl nur Anzeige (Wechsel erfordert einen neuen Raum; Folgeticket bei Bedarf), kein Look-Umschalter vor RU55.
