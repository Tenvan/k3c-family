# B-163 · Die Spiel-Grafiken kommen aus einem Atlas, der Kaltstart hat ein Zeitbudget

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** GR4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint GR4; mit Änderungen aus dem Spec-Review (Voraussetzungen, AC-06 Ladefehler, Texturgröße in GR4.3)

## Ausgangslage

`src/scenes/sprites.ts` lädt jede Figur-Animation als eigene PNG per `scene.load.spritesheet` (41 Ordner, 110 PNG unter `public/sprites/`); Umgebungs-Grafiken (`public/grafik/`, 119 PNG) kommen nach B-010 hinzu. Es gibt weder Atlas noch Messung der Ladezeit auf der Xbox; die Lade-Szene (B-029) fehlt.

## Ziel

Ein Build-Schritt packt die im Spiel genutzten Grafiken zu wenigen Atlanten, und für den Kaltstart gilt ein gemessenes Zeitbudget. Nutzen: Auf der Xbox und im WLAN starten Spiel und Lade-Szene ohne dutzende Einzel-Requests.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox); Entwickler und Agenten bauen und messen; 🧑 misst am TV.

## Anforderungen

- Aufgabe `task atlas` (Eintrag in `Taskfile.yml`) erzeugt Atlas-Bild und -Beschreibung aus den im Spiel genutzten Grafiken (Quelle: `data/sprites.json`, nach GR1/GR3 zusätzlich die Zuordnung aus B-161); Ergebnis eingebunden in `task build`.
- Das Spiel lädt Atlanten statt Einzeldateien; Anzahl der Requests beim Start gesenkt, Messwert vorher und nachher im Ticket.
- Budget „Kaltstart bis Menü“ in Sekunden auf der Xbox, Messung über die Gamepad-Testseite oder eine Messung im Browser (Wert legt 🧑 fest, Startwert aus der ersten Messung).
- Atlas-Erzeugung deterministisch (gleiche Eingabe, gleiche Ausgabe).

## Nicht-Ziele

Lade-Szene selbst (B-029, im selben Sprint), Nachladen im Hintergrund (B-029 Nicht-Ziel), neue Grafiken (B-162).

## Regeln und Einschränkungen

Aufgaben nur über `task` (`CLAUDE.md`); Lizenzen und Credits bleiben je Quell-Pack erhalten; keine neue schwere Abhängigkeit ohne Begründung (Komplexitäts-Budget, `docs/arbeitsweise.md`); `task check` und `task build` grün.

## Beispiele

`task build` → `dist/` enthält Atlas-PNG und -JSON; der Start lädt einen Atlas statt hunderter Einzel-PNG.

## Ausnahme- und Fehlerfälle

Ein Quell-Bild fehlt → `task atlas` bricht mit Dateinamen ab. Atlas wird zu groß für die Textur-Grenze des Geräts → Aufteilung auf mehrere Atlanten, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `task atlas` erzeugt Atlas-Bild und -Beschreibung; zweimaliger Lauf liefert byte-gleiche Dateien.
- **AC-02** Das Spiel lädt die Figuren und die eingebauten Umgebungs-Grafiken aus Atlanten; die Zahl der Grafik-Requests beim Start ist gegenüber vorher gemessen und dokumentiert.
- **AC-03** Ein Kaltstart-Budget in Sekunden ist festgelegt, am TV (Xbox) gemessen und in `docs/game-design.md` oder diesem Ticket festgehalten; die maximale Texturgröße (`MAX_TEXTURE_SIZE`) der Xbox steht daneben.
- **AC-04** `task check` und `task build` sind grün.

## Offene Fragen

keine

## Notizen

Lücke 11 aus `docs/plan-weiterentwicklung.md` § 4. Hängt an X1 (Messung auf der Xbox) und B-161 (Liste der genutzten Grafiken).
