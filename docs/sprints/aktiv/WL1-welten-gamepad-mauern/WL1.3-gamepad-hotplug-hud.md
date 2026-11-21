# WL1.3 · Gamepad-Hotplug und Beitreten-Meldung im HUD entfernen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/3-gamepad-hotplug-hud
- **Abhängig von:** –
- **Tickets:** B-1212
- **Kriterien:** AC-03

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.
Ein Pfad oder Glob je Zeile in Backticks: daraus entsteht die Kollisionsprüfung paralleler Sessions. Sammeldateien (`docs/backlog/README.md`,
`docs/sprints/README.md`, `docs/projekte/README.md`, `docs/glossar.md`) stehen hier nicht; sie ändert `plan_*` oder der letzte Commit des Sprints.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

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

AC-02 geprüft: Die Beitreten-Meldung ist entfernt (`hud/state.rs` setzt `join_s` leer, `m_join` entfernt); Tests `listen_englisch_und_beitritt`, Profil- und Overlay-Test prüfen jetzt die leere Meldung; `still_hud` ignoriert das Go-Element 12 (Beitreten) in Plätzen, Boxfarbe und Textumriss. Beitreten selbst und Lobby unverändert. `cargo test -p k3c-game --lib` grün.

AC-01 teilweise: In `k3c-platform/src/backend/pads.rs` meldet jetzt jedes Ereignis eines unbekannten, nicht getrennten Pads das Gerät an (vorher nur das Connected-Ereignis; ein später eingeschaltetes Pad, das Tasten vor oder ohne Connected sendet, blieb unbekannt). Plattform-Tests grün. Nicht am Gerät geprüft: Ralf prüft das Pad-Einschalten nach dem Start (Abnahme, Gerät); bleibt es ein Problem, ist die Ursache in `game/devices.rs` (Lobby/Sitze) zu suchen. Rot schon vorher: `still_hud/bilder_stimmen_mit_go_ueberein` (pad-tastatur 3,9 %).
