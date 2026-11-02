# SO8.4 · Hörprobe der vier Klassen-Töne

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so8/4-hoerprobe-klassen-toene
- **Abhängig von:** SO8.3
- **Tickets:** B-576
- **Kriterien:** AC-04

## Ziel

🧑 hat die vier Klassen-Töne gehört und bestätigt oder geändert (AC-04).

## Kontext

Agent: Mensch (Geschmack). Hören im Asset-Studio (Sound-Seite, Reiter Hörprobe, nach EXE-Neubau) oder im Spiel: Skill einer Klasse lernen und wirken, zwei Spieler. Änderung = anderen Kandidaten an Platz 1 ziehen, danach `task gen:soundatlas`.

## Erlaubte Dateien

- `data/assets.json` (über das Asset-Studio), `docs/assets/sounds.md`, `docs/backlog/`

## Nicht-Ziele

Töne für passive Skills (später mit dem AI-Chat).

## Schritte

1. 🧑 hört je Klasse (Tank, Zauberer, Heiler, Dieb) den Skill-Ton und bestätigt oder ändert.
2. Claude zieht Katalog und Atlas nach, setzt Status.

## Fertig, wenn

- [x] AC-04: Vier Töne bestätigt oder geändert (Ergebnis mit Datum und Gerät).

## Prüfen

Manuell durch 🧑.

## Ergebnis

Abnahme durch Ralf (🧑), bestätigt im Chat am 2026-10-31; Claude hat die Prüfung nicht selbst gesehen.

- AC-04 abgenommen (Ralf, Hörprobe): die vier Klassen-Töne (Tank, Zauberer, Heiler, Dieb) gehört und bestätigt. Eine Änderung eines Tons wurde nicht gemeldet, daher keine Änderung an Katalog und Atlas durch diese Session.
- Befunde: Claude wurden keine genannt, es entstehen keine neuen Tickets.
