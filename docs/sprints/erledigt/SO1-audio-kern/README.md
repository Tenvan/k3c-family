# SO1 · CLI · Audio-Kern

- **Status:** erledigt
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-011
- **Start-Commit:** 1fa9529
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-011; mit Änderungen aus dem Spec-Review (Format und Autoplay nach Xbox-Messung B-166, Verweis in AC-03 korrigiert)

## Ausgangslage

Das Spiel hat keinen Ton (B-011). Die Gamepad-Testseite kennt kein Audio (geprüft für `gamepad-test.html` und `src/tools/gamepadTest.ts`); Autoplay und Formate auf Edge der Xbox sind unbekannt (B-166). Feedback-Events liefert F3.

## Ziel

Ein Audio-Kern mit Mixer, Entsperren per Geste und Lautstärke je Gerät steht bereit, an dem Effekte (SO2) und Musik (SO4) hängen. Am Ende sichtbar: ein Demo-Ton auf Ereignis am TV, getrennt regelbare Lautstärken, Verhalten im Split-Screen.

## Beteiligte und Zielgruppen

Spieler am TV und Handy; Umsetzung durch Agent; 🧑 hört am TV ab.

## Anforderungen

B-011 › Anforderungen (Lautstärke einstellbar; Soundeffekte und Musik folgen in SO2 und SO4). Sprint-eigen: `src/audio/` mit Mixer (Busse Musik, Effekte, Ambient), Lautstärke je Gerät im `localStorage`, Format gemäß Ergebnis aus B-166 (Xbox-Messung, `docs/game-design.md`: ogg, mp3 und wav ja, m4a nein; also ogg mit mp3-Fallback), Sound-Atlas (ein Sprite-Sheet statt vieler Requests), Positions-Dämpfung im Split-Screen (jeder hört seinen Bereich, Warnungen global).

## Nicht-Ziele

Katalog und Effekte (SO2, B-167), Hörproben (SO3, B-169), Musik (SO4, B-168), Optionen-Szene (S5, B-146).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Audio reagiert nur auf Events und Snapshots; B nicht belegen; 2 Spieler gleichzeitig. Voraussetzung: B-166 (Messergebnis) und F3 (Feedback-Events).

## Beispiele

Erste Taste am Controller → Audio wird entsperrt, Demo-Ton auf ein Ereignis; Effekte-Lautstärke 0 → Effekte stumm, Musik-Bus unverändert.

## Ausnahme- und Fehlerfälle

Auf der Xbox läuft der `AudioContext` schon vor der ersten Geste, eine Controller-Taste zählt als Geste (B-166). Andere Browser (PC, Handy) blockieren Audio bis zur ersten Eingabe → kein Fehler, Ton startet nach der ersten Taste (B-011 › Ausnahme- und Fehlerfälle). `localStorage` nicht verfügbar → Standardlautstärken, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Der Mixer hat getrennte Busse Musik, Effekte und Ambient mit eigener Lautstärke (B-011/AC-03).
- **AC-02** Die Lautstärken werden je Gerät im `localStorage` gespeichert und beim Start gelesen (Test).
- **AC-03** Vor der ersten Eingabe läuft das Spiel ohne Audio-Fehler, nach der ersten Taste startet der Ton (B-011 › Ausnahme- und Fehlerfälle; Test oder Beobachtung).
- **AC-04** Das Audio-Format entspricht dem Ergebnis aus B-166, mit Fallback (Beobachtung am TV).
- **AC-05** Die Sounds liegen in einem Sound-Atlas, die Zahl der Audio-Requests beim Start ist dokumentiert.
- **AC-06** Im Split-Screen mit 2 Spielern hört jeder seinen Bereich, Warnungen sind global (Test auf der Dämpfungsfunktion).
- **AC-07** Mindestens ein Demo-Ereignis löst einen Ton aus (B-011/AC-01, erster Schritt).
- **AC-08** `task check` ist grün.

## Offene Fragen

- Welche Ereignisse sind „wichtig“ (B-011)? Wird in SO2 mit B-167 geklärt (Q15), blockiert diesen Sprint nicht.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO1.1 | `SO1.1-mixer-lautstaerke.md` | Umsetzung | autonom | fertig |
| SO1.2 | `SO1.2-entsperren-format-atlas.md` | Umsetzung | autonom | fertig |
| SO1.3 | `SO1.3-daempfung-demo.md` | Umsetzung | autonom | fertig |
| SO1.4 | `SO1.4-review.md` | Review | autonom | fertig |
| SO1.5 | `SO1.5-hoerprobe-tv.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-04, Agent (Claude Opus 5.5) in SO1.4, leichtes Review. AC-01, AC-02: Ergebnis SO1.1; AC-03, AC-05: Ergebnis SO1.2; AC-06 bis AC-08: Ergebnis SO1.3 (Tests, kein Browser-Pane). AC-04: angenommen, Validierung offen (SO1.5, Hörprobe am TV); ebenso die Browser-Pane-Beobachtung zu AC-03 und AC-07 (SO1.2, SO1.3: keine Freigabe durch 🧑).
Befunde: keine schweren (kein Audio vor der ersten Eingabe, `localStorage` und `AudioContext` mit try/catch, kein `Math.random()`, Töne selbst erzeugt: kein Credit nötig). Neue Tickets: keine. B-011 bleibt eingeplant (SO2, SO4). `task check:go`: `TestSavesDelete` und `TestGleichzeitigesSpeichern` scheitern auf Windows sporadisch (B-187), im Wiederholungslauf grün; Sprint ändert kein Go.
Version: v0.6.0 vorgeschlagen (gemeinsamer Tag nach v0.5.0, Minor); gesetzt erst nach Bestätigung durch 🧑.
