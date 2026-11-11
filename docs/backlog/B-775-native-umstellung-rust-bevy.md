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

Ein natives Windows-Spiel in **Go + Ebitengine** mit den heutigen Regeln, der Papier-Grafik und 1–2 lokalen Spielern; Browser, Netz und Server fallen für das Spielen weg. (Erster Ansatz Rust + Bevy, siehe Titel; 🧑 hat sich am 2026-11-11 für Go ausgesprochen.)

## Beteiligte und Zielgruppen

🧑 Ralf (Entscheidung, Abnahme am Mini-PC), Familie (spielt zu zweit am TV), Claude (Umsetzung).

## Anforderungen

- Gleiche Regeln wie heute: Die Go-Simulation (`engine/sim`, `engine/level`) läuft unverändert im selben Programm, ohne Netz und ohne JSON-Roundtrip; `engine/room` und `engine/store` soweit nötig (Takt, Spielstände).
- Deterministisch wie bisher (`engine/rng`), Goldens bleiben grün.
- 1–2 Spieler lokal, je ein Gamepad, Split-Screen.
- Grafik aus den bestehenden Papier-Sheets (`public/grafik`, `data/paper.json`), Texturen je Level geladen statt alle; Monarch als Gliederpuppe (B-778).
- Effekte des heutigen Clients (Parallaxe, Fluss mit Spiegelung, Lichtkarte, Partikel, Tweens) nachgebaut.
- Flüssig auf dem Mini-PC (Messwerte aus NV1 als Maßstab).

## Nicht-Ziele

Online-Spiel, Browser, Xbox; Regeländerungen während der Umstellung; Rust-Portierung und Rust-Server (Entscheidungen 006/007).

## Regeln und Einschränkungen

Entscheidung 005 (Papier-Grafik aus Specs) gilt weiter. Entscheidung 001 (Engine auf Go-Server) wird angepasst: Engine bleibt Go, der Server entfällt für das Spielen – neue Entscheidung 008 bei Freigabe. Ebitengine-Version in `go.mod` festgelegt, eigenes Go-Modul oder Einbindung per `replace` (bei der Planung klären). Unter Windows zeichnet Ebitengine über DirectX 11, nicht Vulkan.

## Beispiele

Zwei Spieler starten am Mini-PC ein Spiel im Wald → beide steuern je einen Monarchen mit ihrem Gamepad, Split-Screen, die Welt verhält sich wie im heutigen Spiel mit demselben Seed.

## Ausnahme- und Fehlerfälle

Ein Gamepad fehlt → Einzelspieler oder Tastatur. Grafik fehlt (kein `gen:paper`) → klare Meldung beim Start statt leerem Bild.

## Akzeptanzkriterien

- **AC-01** NV1 (B-776, B-777) ist erledigt, die Mini-PC-Werte liegen vor und 🧑 hat den Weg bestätigt (Entscheidung 008).
- **AC-02** Das native Spiel nutzt `engine/sim` unverändert; alle bestehenden Go-Tests und Goldens bleiben grün.
- **AC-03** Das native Spiel läuft auf dem Mini-PC mit 2 Gamepads auf dem heutigen Funktionsstand (Abnahme 🧑).

## Offene Fragen

- Ruht RST (Rust-Engine/-Server) damit? Entscheidet 🧑.
- Bleibt der Go-Server samt Browser-Client als zweiter Weg erhalten oder wird er nach der Umstellung entfernt? Entscheidet 🧑.

## Notizen

Daten, Prototyp-Vergleich und Schätzung: Projekt NAT › Notizen. Schätzung Go + Ebitengine ca. 2–4 Wochen, dazu B-778 ca. 1 Woche.
