# B-805 · Ein Fehler des Ton-Geräts beendet das native Spiel nicht mehr

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG15
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

`native/k3c-game/audio` nutzt `ebiten/audio`. `audio.NewContext` hängt einen Hook ein, der vor jedem Update läuft und jeden Treiberfehler zurückgibt (Ebitengine v2.10.6, `audio/audio.go:131`); `RunGame` endet damit, bevor `game.Update` eingreifen kann. Nur „kein Gerät“ fällt unter Windows still auf ein Null-Gerät (oto `driver_windows.go:66`). Belegt in der Zusammenführung der Welle 2b (2026-11-11).

## Ziel

Ein defektes oder verschwindendes Ton-Gerät macht das Spiel stumm, beendet es aber nicht.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC, 🧑 Ralf (Freigabe der Abhängigkeit), Claude (Umsetzung).

## Anforderungen

- Ton über oto direkt statt `ebiten/audio` (oder ein gleichwertiger Weg, der Treiberfehler fängt).
- Fehler des Geräts: Log-Zeile, Ton aus, Spiel läuft weiter.

## Nicht-Ziele

Neue Klänge, Änderung der Zuordnung Ereignis → Klang.

## Regeln und Einschränkungen

Entscheidung 008. oto ist heute indirekte Abhängigkeit von Ebitengine; direkte Nutzung ändert `go.mod` des Moduls.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/audio/**`, `native/k3c-game/go.mod`, `native/k3c-game/go.sum`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** deps

## Beispiele

Kopfhörer während des Spiels abziehen → Ton aus oder auf anderem Gerät, Spiel läuft weiter.

## Ausnahme- und Fehlerfälle

Gerät kommt zurück → Ton bleibt stumm bis zum Neustart (entschieden, siehe Offene Fragen).

## Akzeptanzkriterien

- **AC-01** Test oder nachgestellter Fehler des Treibers: Spiel läuft weiter, Log-Zeile vorhanden.
- **AC-02** Am PC: Gerät während des Spiels entfernen (Abnahme 🧑).

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: oto (`github.com/ebitengine/oto/v3`) wird direkte Abhängigkeit des Moduls `native/k3c-game`. Begründung: oto ist schon indirekte Abhängigkeit von Ebitengine (v3.5.0 in `go.mod`), es kommt kein neuer Code ins Modul, nur die Markierung `// indirect` entfällt; Sperre `deps` bleibt. Gerät kommt zurück → stumm bis zum Neustart (Vorschlag übernommen, kein Wiederverbinden).

## Notizen

Gefunden in NG8 und der Zusammenführung der Welle 2b (2026-11-11).
