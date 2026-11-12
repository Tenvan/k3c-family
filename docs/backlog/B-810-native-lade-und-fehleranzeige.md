# B-810 · Das native Spiel meldet Startfehler und fehlende Grafik im Fenster und zeigt beim Stufenwechsel eine Ladeanzeige

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NG14
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Das native Spiel beendet sich bei einem Fehler beim Start oder bei fehlender Grafik still; B-775 verlangt eine Meldung im Fenster. Beim Stufenwechsel lädt es die Grafik synchron (`native/k3c-game/game/levels.go:47-61`), das Bild friert ohne Anzeige ein. Ungeprüft: ob die EXE als GUI gebaut ist und stderr damit unsichtbar bleibt.

## Ziel

Wer das Spiel startet, sieht bei einem Fehler eine Meldung im Fenster und beim Stufenwechsel eine Ladeanzeige.

## Beteiligte und Zielgruppen

Spieler am Mini-PC ohne Konsole, Claude (Umsetzung).

## Anforderungen

- Startfehler und fehlende Grafik erscheinen als Text im Fenster mit Hinweis, was fehlt.
- Ladeanzeige während des Stufenwechsels.
- Prüfen, ob stderr in der gebauten EXE sichtbar ist; Ergebnis in den Notizen.
- Log-Zeilen mit Emoji nach Projektregel.

## Nicht-Ziele

Fehler des Ton-Geräts (B-805), asynchrones Laden der Grafik, falls der Umbau die Ladeanzeige überflüssig macht (dann eigenes Ticket).

## Regeln und Einschränkungen

Entscheidung 008, Logging mit Emojis, Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/main.go`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Ordner `public/grafik` fehlt → Fenster zeigt „Grafik fehlt: <Pfad>“. Stufenwechsel → Ladeanzeige, dann Spiel.

## Ausnahme- und Fehlerfälle

Fenster lässt sich nicht öffnen → Fehler auf stderr und Exit-Code ungleich 0. Fehler während des Spiels bleibt Sache von B-805 (Ton).

## Akzeptanzkriterien

- **AC-01** Test: fehlende Grafik liefert einen Fehlerzustand mit Text, nicht Exit ohne Meldung.
- **AC-02** Test: Stufenwechsel setzt vor dem Laden den Zustand „lädt“ und zeigt die Anzeige.
- **AC-03** Notiz: stderr der gebauten EXE geprüft, Ergebnis festgehalten.
- **AC-04** Am PC: Grafikordner umbenennen, starten, Meldung sehen (Abnahme 🧑).

## Offene Fragen

keine

## Notizen

Gefunden in NG9 (2026-11-11). Verwandt, nicht blockierend: B-805.
