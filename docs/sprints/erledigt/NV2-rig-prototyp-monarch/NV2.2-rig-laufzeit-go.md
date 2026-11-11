# NV2.2 · Rig-Laufzeit in Go: Pose, Umhang, Zeichnen, Reiten, Teile tauschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** nv2/2-rig-laufzeit-go
- **Abhängig von:** NV2.1
- **Tickets:** B-778
- **Kriterien:** AC-02, AC-03

## Ziel

`task native:rig` öffnet ein Ebitengine-Fenster mit Monarch und Pferd aus Rig-Teilen neben den Sheets: `idle`, `lauf`, Keyframe-Animation, Reiten in Schritt, Trab und Galopp, Umhang, Waffe und Rüstung als optionale Elemente.

## Kontext

- Eingang: `native/k3c-rig/assets/` und `RIG.md` aus NV2.1.
- Zu portieren (Formeln und Konstanten unverändert): `tools/paper/render/matrix.ts`, `baum.ts`, `pose.ts`; `rig/figur.ts`, `proportionen.ts`, `umhang.ts`, `kurve.ts`; für das Pferd `rig/gangarten.ts`, `vierbeiner.ts`, `beinpfad.ts` (`beinPunkte`), aus `reiten.ts` Sattel, Ebenen und Zügel.
- Der Rig-Code ist ein eigenes Paket ohne Ebitengine (`native/k3c-rig/rig`), damit das Spiel ihn später für Gegner und Bosse nutzt; das Zeichnen liegt getrennt.
- Kombination: Reiter am Platz `sattel`, Zeichenreihenfolge nach Ebenen (Reittier hinten, Umhang, Reiter, Reittier vorn, Zügel).
- Vorlage für Fenster, Sheet-Laden, Eingabe: `native/k3c-ebiten/`. Zeichnen: `DrawImage` mit `GeoM` je Teil, Bänder mit `DrawTriangles`.
- Winkel in Grad, positiv im Uhrzeigersinn, y nach unten (wie SVG).

## Erlaubte Dateien

- `native/k3c-rig/**` (eigenes Go-Modul, Ebitengine-Version wie `native/k3c-ebiten/go.mod`)
- `native/Taskfile.yml`
- Planung B-778, NV2 (über MCP)

## Nicht-Ziele

Simulation, HUD, Netz; andere Figuren; alle Reittiere und Gangarten; Änderungen an `native/k3c-ebiten`, `tools/paper`, `engine/`, `src/`.

## Schritte

1. Go-Modul, Rig laden, Matrix-Kette, Plätze; Tests gegen die Soll-Werte (Toleranz 0,01).
2. Figur: Posen `idle`, `lauf`, Keyframes; Pose je Bild aus der Zeit.
3. Vierbeiner: Gangarten, Bein-Bänder.
4. Zeichnen mit Ebenen, vier Farben, daneben die Sheets.
5. Umhang: Verlet mit festem Zeitschritt, Fahrtwind aus der Geschwindigkeit.
6. Reiten: Reiter am Sattel, drei Gangarten, Zügel.
7. Elemente: Waffe und Rüstung an ihren Plätzen tauschen und abnehmen; Tasten und Gamepad.

## Fertig, wenn

- [x] AC-02: `go test` vergleicht Teil-Matrizen und Stützpunkte für Monarch (`idle`, `lauf`) und Pferd (`trab`) mit den Sollwerten.
- [x] AC-03: Screenshots je Zustand (idle, lauf, Keyframe-Animation, Schritt, Trab, Galopp, getauschte und abgenommene Waffe und Rüstung) liegen im Ergebnis; `go vet` und `gofmt` sauber.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-02 geprüft:** `task native:rig:test` grün (`rig/expected_test.go`): Winkel, Teil-Matrizen, Umhang-Kette, Bein-Stützpunkte und Zügel-Enden je Frame gegen `assets/soll/` (Toleranz 0,01) für Monarch `idle`, `lauf` und Reiter mit Pferd `trab`. Gegenprobe: eine geänderte Konstante (Schenkelwinkel, Hufhub) macht den Test rot.
- **AC-03 umgesetzt, geprüft von Claude an Screenshots:** `task native:rig:shots` schreibt 14 Bilder nach `reports/native-rig-shots/` (idle, lauf, give, hit, death, respawn, reiten-idle, schritt, trab, galopp, dazu Axt, Brustpanzer, leerer Waffenplatz, gespiegelt). Fenster: `task native:rig`, zwei Monarchen, je Zeile links Sheet und rechts Rig. `gofmt` und `go vet` sauber. Die Sichtprüfung im laufenden Fenster (Umhang in Bewegung, Gamepads) bleibt bei 🧑 (NV2.4).
- **Befunde:** (1) Band faltete sich am stark gebeugten Knie → Kanten je Stützpunkt an der Papierkante (`breite`), Test `TestLegStripNeverFolds`. (2) Zügel zu dunkel (Farbe doppelt vormultipliziert) → behoben. (3) Offen für 🧑: Der Umhang wird bei stark geknickter Kette kantig (death am Boden).
- **Abweichungen:** Tempo und Blickrichtung statt freier Bewegung im Fenster (der Umhang folgt Fahrtwind × Tempo); Ebitengine 2.10.6 wie `native/k3c-ebiten`, keine neue Abhängigkeit. Neue Tickets: keine.
