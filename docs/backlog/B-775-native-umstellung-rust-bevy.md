# B-775 · K3C läuft nativ in Rust und Bevy auf dem Mini-PC, lokal für 1–2 Spieler ohne Server

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K3C läuft als Phaser-4-Browser-Client (`src/`) mit Go-Server (`engine/`, Docker auf dem Pi). Auf der Xbox (Edge) verliert WebGL nach 2–4 s den Kontext (214–347 MiB Texturen), die Leistung ist dort unbrauchbar. Neue Zielplattform ist ein Mini-PC (Windows, Vulkan-GPU). Gespielt wird lokal zu zweit; ein Server ist dafür nicht nötig. Die Daten zur Lage stehen im Projekt NAT › Notizen.

## Ziel

Ein natives Windows-Spiel in Rust + Bevy mit den heutigen Regeln, der Papier-Grafik und 1–2 lokalen Spielern; Browser, Netz und Server fallen für das Spielen weg.

## Beteiligte und Zielgruppen

🧑 Ralf (Entscheidung, Abnahme am Mini-PC), Familie (spielt zu zweit am TV), Claude (Umsetzung).

## Anforderungen

- Gleiche Regeln wie heute: Die Rust-Simulation liefert für Seed und Eingaben dieselben Ergebnisse wie die Go-Simulation (Goldens).
- Deterministisch, eigener RNG wie `engine/rng`, kein Zufall aus der Plattform.
- 1–2 Spieler lokal, je ein Gamepad, Split-Screen.
- Grafik aus den bestehenden Papier-Sheets (`public/grafik`, `data/paper.json`), Texturen je Level geladen statt alle.
- Flüssig auf dem Mini-PC (Messwerte aus NV1 als Maßstab).

## Nicht-Ziele

Online-Spiel, Browser, Xbox; Regeländerungen während der Portierung; Rust-Server (Entscheidung 007).

## Regeln und Einschränkungen

Entscheidung 005 (Papier-Grafik aus Specs) gilt weiter. Entscheidung 001 (Engine auf Go-Server) und 006/007 (Rust-Kern/-Server) müssten bei Freigabe angepasst werden – neue Entscheidung 008. Bevy-Version gepinnt (`=x.y.z`, `Cargo.lock` eingecheckt).

## Beispiele

Zwei Spieler starten am Mini-PC ein Spiel im Wald → beide steuern je einen Monarchen mit ihrem Gamepad, Split-Screen, die Welt verhält sich wie im heutigen Spiel mit demselben Seed.

## Ausnahme- und Fehlerfälle

Ein Gamepad fehlt → Einzelspieler oder Tastatur. Grafik fehlt (kein `gen:paper`) → klare Meldung beim Start statt leerem Bild.

## Akzeptanzkriterien

- **AC-01** NV1 (B-776) ist erledigt und 🧑 hat auf Grundlage der Messwerte über den Weg entschieden (Entscheidung 008).
- **AC-02** Die Rust-Simulation besteht alle Goldens aus `testdata/golden/` gleich wie Go.
- **AC-03** Das native Spiel läuft auf dem Mini-PC mit 2 Gamepads auf dem heutigen Funktionsstand (Abnahme 🧑).

## Offene Fragen

- Weg endgültig Rust + Bevy oder Go + Ebitengine? Entscheidet 🧑 nach NV1.
- Ruht RST (Rust-Server) damit? Entscheidet 🧑.

## Notizen

Daten und Schätzung: Projekt NAT › Notizen. Vergleich der Kandidaten (2026-11-10): Go + Ebitengine (Go-Sim 1:1, ca. 2–4 Wochen), Bevy (Neubau Sim, ca. 4–6 Wochen), Godot + gdext, MonoGame, Raylib.
