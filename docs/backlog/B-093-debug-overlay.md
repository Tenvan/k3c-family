# B-093 · Ein Debug-Overlay zeigt Verbindung, Snapshot-Takt und Entitäten im Spiel

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** U4
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1; Revision 2 (Ö statt F3, dev standardmäßig an) auf Zuruf von Ralf am 2026-10-02

## Ausgangslage

Der Browser zeigt immer eine FPS-Zahl (`HudScene.ts`), sonst nichts über den Zustand der Verbindung. Wer ein Ruckeln, Verzögerungen oder einen verlorenen Raum untersucht, muss die Browser-Konsole (`window.game` mit `?dev=1`) oder den Server (`/api/status`, `k3c-tui`) bemühen. Auf der Xbox gibt es keine Konsole.

## Ziel

Ein ein- und ausblendbares Overlay im Spiel zeigt: FPS, Raum-Code und Geräte-Kennung, Verbindungsstatus, Protokollversion, Snapshot-Takt (Hz) und Alter des letzten Snapshots, Zeit/Tag und Entitäten je Art. Nutzen: Fehlersuche am TV ohne Konsole.

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Testen auf Xbox und Handy.

## Anforderungen

- Nur lesend: Das Overlay zeigt, was der Client ohnehin kennt (`clientConnection`, Snapshots, `World`); es sendet nichts.
- Nur sichtbar, wenn eingeschaltet: mit `?dev=1` verfügbar, per Taste umschaltbar (Belegung siehe Offene Fragen); ohne `?dev=1` kein Overlay und kein messbarer Mehraufwand.
- Die Kennzahlen kommen aus einer reinen, getesteten Funktion (Verbindungs- und Weltzustand → Textzeilen).
- Die vorhandene FPS-Anzeige geht im Overlay auf, statt doppelt zu erscheinen.

## Nicht-Ziele

Dev-Aktionen wie Gold geben, Stufe wechseln, Neustart (B-080, Server-Aufrufe), Ping über eine neue Protokoll-Nachricht (nur mit eigener Protokoll-Session), Aufzeichnen oder Exportieren der Werte.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes`, `src/online/client*`); Controller-Taste B und View + Menu bleiben unbelegt; mit 2+ lokalen Spielern gilt das Overlay für das ganze Gerät, nicht je Spieler. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

`game.html?dev=1`, Overlay an: „Raum FAMILIE · Gerät a3f1 · verbunden · v2 · 30 Hz · letzter Snapshot 31 ms · Tag 2 · 14 Gegner · 3 Truppen“. Verbindung bricht ab → Status „getrennt seit 2,4 s“.

## Ausnahme- und Fehlerfälle

Noch keine Verbindung → „verbindet…“, keine Zahlen erfinden. Fehlende Werte → „–“. Ohne `?dev=1` bleibt das Spiel unverändert.

## Akzeptanzkriterien

- **AC-01** Eine reine Funktion liefert aus Verbindungs- und Weltzustand die Overlay-Zeilen; Vitest deckt verbunden, getrennt, verbindet und fehlende Werte ab.
- **AC-02** In der Entwicklungsphase ist das Overlay ohne Parameter verfügbar, mit `?dev=0` wird es nicht erzeugt (Test oder Prüfung der Szene); es lässt sich per Tastatur- und per Controller-Taste umschalten.
- **AC-03** Die alte FPS-Anzeige ist im Overlay aufgegangen; `task check` ist grün, `src/scenes` bleibt frei von Spiel-Logik.
- **AC-04** 🧑 hat das Overlay auf der Xbox oder am TV abgenommen.

## Offene Fragen

Entschieden (🧑, 2026-10-02): Ö (Tastatur; F3 ist im Browser belegt, D ist „laufen“) und Klick auf den linken Stick (Controller) schalten um; die FPS-Anzeige gibt es nur noch im Overlay.

## Notizen

Ping/Round-Trip bräuchte eine Protokoll-Nachricht; bei Bedarf eigenes Ticket. B-080 liefert später die Dev-Aktionen.
