# B-772 · Auf der Xbox läuft WebGL 1 mit einem Texturspeicher von höchstens 130 MiB

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LG1
- **Projekt:** LST
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Nutzer: „LST ganz abarbeiten“, 2026-11-10)

## Ausgangslage

Auf der Xbox (Edge) verliert das Spiel den WebGL-Kontext nach 2,4 bis 4,4 s bei 214 bis 347 MiB (B-769, RX1). Die erste WebGL-Analyse (Nutzer, 2026-11-10) ergab: **WebGL 1.0 funktioniert wohl auf der Xbox**, und der **Speicher sollte auf 130 MiB beschränkt werden**. Die Analyseberichte habe ich noch nicht gelesen, die Zahlen stammen vom Nutzer.

## Ziel

Der Client läuft auf der Xbox stabil mit WebGL 1 und bleibt unter einem Speicherbudget von 130 MiB (Texturen und Puffer), ohne Kontextverlust.

## Beteiligte und Zielgruppen

Spieler auf der Xbox; Entwickler (🧑) nimmt auf dem Gerät ab.

## Anforderungen

- Der Client wählt auf der Xbox WebGL 1 (Phaser `WEBGL` mit Kontext-Versuch 1), sonst bleibt die bisherige Wahl; die Erkennung läuft über das vorhandene Render-Log (`renderInfo.ts`, B-762).
- Es gibt ein festes Speicherbudget von 130 MiB für Texturen auf der Xbox (Datenwert in `data/`, nicht im Code); der Texturspeicher aus `renderInfo.ts` wird dagegen gemessen.
- Wird das Budget überschritten, laden Papierschnitt-Gruppen und Hintergründe in kleinerer Auflösung oder gar nicht (Texturprofil, RX1.6) statt den Kontext zu verlieren.
- Das Client-Log meldet je Start Renderer, WebGL-Version, Budget und Verbrauch (eine Zeile, Rate-Limit beachten).

## Nicht-Ziele

Kein WebGL 2 auf der Xbox, keine Änderung an anderen Geräten, keine neue Grafik.

## Regeln und Einschränkungen

Werte aus `data/`, keine Spiel-Logik im Client, Emoji-Präfix im Log, Datei ≤ 400 Zeilen. Verträge der Papierschnitt-Grafik (Entscheidung 005) bleiben. Abstimmung mit RX1.6 (Texturprofil) und B-764 (Texturbudget), kein Doppelbau.

## Beispiele

- Xbox-Start: Log `🚀 WebGL 1, Texturen 96 von 130 MiB` → kein Kontextverlust.
- PC-Start: WebGL 2, Budget nicht aktiv, Verhalten wie bisher.

## Ausnahme- und Fehlerfälle

- WebGL 1 nicht verfügbar → bisherige Fallback-Kette, Hinweis im Log.
- Budget schon beim Start überschritten → kleinste Auflösung, Warnzeile im Log.

## Akzeptanzkriterien

- **AC-01** Test: Die Wahl der WebGL-Version nach Gerät und die Budgetprüfung (Vitest, reine Logik).
- **AC-02** Test: Das Budget kommt aus `data/` und wird von `tests/projectRules.test.ts`-konformem Code gelesen, kein fester Wert im Code.
- **AC-03** Abnahme (🧑): Xbox-Lauf von Menü bis Spiel ohne Kontextverlust; Client-Log (`/api/dev/log?file=client`, B-770) zeigt WebGL 1 und Verbrauch unter 130 MiB.

## Offene Fragen

keine. Entschieden von 🧑 am 2026-11-10: 130 **MiB**, nur **Texturspeicher**. Beides stammt aus der Erinnerung von 🧑 und wird später an den Logs geprüft (B-770/B-771); weicht das Log ab, wird das Budget angepasst.

## Notizen

Quelle: 🧑 am 2026-11-10 aus der Erinnerung; der Analysebericht ist unbekannt („gestern ab ca. 22:00“). Unter `/api/dev/reports` liegt am 2026-11-10 aber kein Bericht nach 2026-11-09 16:51 UTC; ein Bericht dieser Analyse ist dort also nicht zu finden. Zusammenhang: B-761, B-762, B-764, B-769, RX1.
