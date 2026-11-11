# B-775 · K3C läuft nativ in Go und Ebitengine auf dem Mini-PC, lokal für 1–2 Spieler ohne Server

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-10
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

K3C läuft als Phaser-4-Browser-Client (`src/`) mit Go-Server (`engine/`, Docker auf dem Pi). Auf der Xbox (Edge) verliert WebGL nach 2–4 s den Kontext (214–347 MiB Texturen), die Leistung ist dort unbrauchbar. Neue Zielplattform ist ein Mini-PC (Windows, Vulkan-GPU). Gespielt wird lokal zu zweit; ein Server ist dafür nicht nötig. Die Daten zur Lage stehen im Projekt NAT › Notizen.

## Ziel

Ein natives Windows-Spiel in **Go + Ebitengine** (Entscheidung 008) als **lokal laufender Monolith**: ein Prozess, ohne Server und ohne Netz. Simulation, Level, Spielstände, Zeichnen und Eingabe liegen im selben Programm; heutige Regeln, Papier-Grafik, 1–2 lokale Spieler.

## Beteiligte und Zielgruppen

🧑 Ralf (Entscheidung, Abnahme am Mini-PC), Familie (spielt zu zweit am TV), Claude (Umsetzung).

## Anforderungen

- Gleiche Regeln wie heute: Das native Spiel bindet `engine/sim`, `engine/level` und `engine/rng` unverändert ein; die Goldens bleiben gültig.
- Deterministisch, nur `engine/rng`, kein Zufall aus der Plattform.
- 1–2 Spieler lokal, je ein Gamepad, Split-Screen.
- Grafik aus den bestehenden Papier-Sheets (`public/grafik`, `data/paper.json`), Texturen je Level geladen statt alle; der Monarch als Rig aus Teilen, wenn der Prototyp B-778 das bestätigt.
- Flüssig auf dem Mini-PC (Messwerte aus NV1 als Maßstab).
- Kein Rust im Repo (B-779).

## Nicht-Ziele

Online-Spiel, Browser, Xbox; Regeländerungen während der Umstellung; Rust und Bevy (Entscheidung 008).

## Regeln und Einschränkungen

Entscheidung 005 (Papier-Grafik aus Specs) und 008 (Go + Ebitengine, Rust entfällt) gelten; 006 und 007 sind überholt. Entscheidung 001 gilt für das Online-Spiel weiter, bis NAT es ablöst. Ebitengine-Version in `go.mod` festgelegt, kein cgo.

## Beispiele

Zwei Spieler starten am Mini-PC ein Spiel im Wald → beide steuern je einen Monarchen mit ihrem Gamepad, Split-Screen, die Welt verhält sich wie im heutigen Spiel mit demselben Seed.

## Ausnahme- und Fehlerfälle

Ein Gamepad fehlt → Einzelspieler oder Tastatur. Grafik fehlt (kein `gen:paper`) → klare Meldung beim Start statt leerem Bild.

## Akzeptanzkriterien

- **AC-01** NV1 (B-776, B-777) ist erledigt und 🧑 hat über den Weg entschieden (Entscheidung 008: Go + Ebitengine). Entschieden am 2026-11-11; offen ist nur die Messung am Mini-PC (NV1.3).
- **AC-02** Das native Spiel rechnet mit der unveränderten Go-Simulation; alle Goldens aus `testdata/golden/` bleiben grün.
- **AC-03** Das native Spiel läuft auf dem Mini-PC mit 2 Gamepads auf dem heutigen Funktionsstand (Abnahme 🧑).
- **AC-04** Der Weg für den Monarchen (Rig aus Teilen oder Sheets) ist nach B-778 entschieden.

## Offene Fragen

- Wie bindet der Monolith die Simulation ein: `engine/room` ohne Netz oder direkt `engine/sim`? Klärt der erste Umsetzungs-Sprint nach NV2.
- Wann fallen Go-Server (`engine/net`, `cmd/k3c-server`), Browser-Client (`src/`) und die Werkzeuge dafür weg? Default: erst, wenn der Monolith den heutigen Funktionsstand hat; eigenes Aufräum-Ticket dann.

## Notizen

Daten und Schätzung: Projekt NAT › Notizen. Vergleich der Kandidaten (2026-11-10): Go + Ebitengine (Go-Sim 1:1, ca. 2–4 Wochen), Bevy (Neubau Sim, ca. 4–6 Wochen), Godot + gdext, MonoGame, Raylib.

2026-11-11, Revision 2: 🧑 hat Go + Ebitengine gewählt (Leistung im Spiel, Kompilierung), Rust entfällt komplett (Entscheidung 008). Der Dateiname des Tickets nennt noch den alten Weg. Folge-Tickets: B-778 (Rig-Prototyp Monarch, Sprint NV2), B-779 (Rust und Bevy entfernen).
