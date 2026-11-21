# B-1179 · Cheat-Fenster braucht ein Panel, das die Umgebungseffekte einzeln ein- und ausschaltet

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LA2
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

LA1.5 hat die Umgebungseffekte hinzugefügt. `world/fx.rs` (`fx::on`, `fx::set`, `EFFECTS`) schaltet sie schon einzeln (Start per `K3C_FX_OFF=haze,halos,…`), aber das Cheat-Fenster (`k3c-devwin`, Protokoll `k3c-devproto`) hat kein Panel dafür. 🧑 2026-11-21: Beim Abdunkeln wirkt das Bild blass; welcher Effekt (Dunst, Tönung, Aufhellung, Höfe) daran schuld ist, soll man live am PC sehen.

## Ziel

Ein Panel im Cheat-Fenster mit je einem Schalter je Umgebungseffekt (Dunst, Lichthöfe, Sterne, Himmelsverlauf, Nacht-Aufhellung, Tönung, Spiegelung), live wirksam im Spiel.

## Beteiligte und Zielgruppen

🧑 am PC.

## Anforderungen

- Protokoll: Katalog liefert `fx::EFFECTS`, der Zustand die ausgeschalteten Effekte, ein Befehl schaltet einen Effekt (Versionszähler prüfen).
- Das Spiel ruft `fx::set`; ein Effekt steht nach Neustart des Fensters wieder auf dem Stand des Spiels.
- Ohne Fenster keine Mehrarbeit je Takt.

## Nicht-Ziele

Neue Effekte, Änderung der Effekt-Werte (`data/look.json`).

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, `k3c-devwin` ohne `unsafe` außerhalb `win32/`, Tests für das Protokoll (`tests/pipe.rs`).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-devproto/**`, `rust/crates/k3c-devwin/**`, `rust/crates/k3c-game/src/devwin/**`, `rust/crates/k3c-game/src/game/devwinhost.rs`
- **Abhängig von:** LA1.5
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Panel „Umgebung“: Haken bei Dunst aus, Spiegelung an → sofort sichtbar im Spiel.

## Ausnahme- und Fehlerfälle

Unbekannte Kennung vom Fenster: Ack mit Fehler, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Das Cheat-Fenster zeigt je Effekt aus `fx::EFFECTS` einen Schalter; Umschalten wirkt im nächsten Bild (Test mit Mock-Host).
- **AC-02** Abnahme am PC durch 🧑.

## Offene Fragen

keine

## Notizen

Herkunft: LA1.5, Nachprüfung 🧑 2026-11-21.
