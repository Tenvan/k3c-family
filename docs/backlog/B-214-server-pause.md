# B-214 · Der Server pausiert den Raum im Couch-Raum und schützt den stehenden Monarchen online

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RM1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Optionen-Szene (S5.2, B-146) hält nur den Client an: Solange sie offen ist, sendet das Gerät „Stillstand“ für seine Monarchen, der Raum rechnet weiter. Im Protokoll und im Server gibt es keine Pause (`engine/room`, `engine/net`, `docs/protocol.md`). Die Regel steht in `docs/rules/bedienung.md` › Pause (Q01).

## Ziel

Der Raum hält im Couch-Raum (alle Spieler an einem Gerät) wirklich an; online ist der Monarch eines pausierten Geräts geschützt (unverwundbar), höchstens 60 s, danach verwundbar.

## Beteiligte und Zielgruppen

Spieler an einem Gerät und online; 🧑 entscheidet Protokollform und Abnahme.

## Anforderungen

- Protokoll: Pause und Fortsetzen als Nachricht des Geräts (neue Protokollversion, Spiegel in `docs/protocol.md`).
- Couch-Raum: Simulation des Raums hält an, unbegrenzt; online: nur Schutz der Monarchen dieses Geräts, höchstens 60 s (wie `WaitFor` in `engine/room/room.go`).
- Deterministisch, mit 2 und mehr Spielern je Gerät; Client öffnet und schließt die Szene wie in S5.2.

## Nicht-Ziele

Optionen-Szene selbst (S5.2), Verbindungsverlust (`docs/rules/bedienung.md` § 3).

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 1, `docs/decisions/001`, `engine/rng` statt Zufall, Datei ≤ 400 Zeilen.

## Beispiele

Ein Gerät mit 2 Spielern drückt Menu kurz → Raum hält an, Nacht und Gegner stehen; Fortsetzen → weiter. Zwei Geräte: Gerät A pausiert, B spielt weiter, der Monarch von A ist 60 s unverwundbar.

## Ausnahme- und Fehlerfälle

Gerät pausiert und verliert die Verbindung → Frist laut § 3; die Pause endet.

## Akzeptanzkriterien

- **AC-01** Go-Test: Im Couch-Raum ändert sich der Zustand während der Pause nicht, nach dem Fortsetzen läuft er weiter.
- **AC-02** Go-Test: Online ist der pausierte Monarch 60 s unverwundbar, danach verwundbar; andere Geräte laufen unverändert.

## Offene Fragen

Protokollform (eigene Nachricht oder Feld in `input`): 🧑.

## Notizen

Angelegt in S5.2.
