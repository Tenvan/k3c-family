# RX1 · CLI, PAP · Xbox hält den WebGL-Kontext: Render-Log, Kontextverlust abfangen, Texturen auf 2048 px

- **Status:** geplant
- **Projekt:** LST
- **Domäne:** CLI, PAP
- **Reife:** Entwurf
- **Tickets:** B-761, B-762, B-763, B-764
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Auf der Xbox-Edge (192.168.2.209) meldet der Client nach 3–4 s dreimal „WebGL Context lost“, danach weiße Seite bei ca. 12 FPS. Die Technik-Anzeige (B-761, PR #530, in `develop`) zeigt Renderer und Texturwerte nur am TV; ins Server-Log gelangen sie nicht. Verdacht: Sheets bis 3072×1920 px (`public/grafik`) sprengen den Texturspeicher. Details: B-761 bis B-764.

## Ziel

Die Xbox hält den WebGL-Kontext, und das Render-Verhalten jedes Clients ist im Server-Log lesbar. Am Ende sichtbar: Xbox-Start ohne „WebGL Context lost“ im Server-Log (`logs_query`), Startzeile mit Renderer, GPU und Texturspeicher, Leistungsverlauf alle 30 s, FPS-Wert notiert.

## Beteiligte und Zielgruppen

🧑 testet und nimmt auf der Xbox ab (RX1.3, RX1.6); Claude setzt um und wertet das Log über `logs_query` aus.

## Anforderungen

B-762 › Anforderungen (Render-Log); B-763 › Anforderungen (Kontextverlust); B-764 › Anforderungen (Texturprofil ≤ 2048 px, Laden bei Bedarf); B-761 nur noch AC-05 (Abnahme am TV). Sprint-eigen: Die Technik-Anzeige (B-761, `?gfx=1`) steht ab RX1.1 standardmäßig **an** (Wunsch von 🧑, ändert B-761/AC-04; abschaltbar im Menü und mit `?gfx=0`). Die Reihenfolge ist fest, B-764 beginnt erst nach dem Xbox-Lauf (RX1.3), weil die Log-Werte die Ursache bestätigen müssen.

## Nicht-Ziele

Leistungsoptimierung des Canvas-Renderers; zweites Texturprofil oder Low-Profil; Wechsel der Grafikbibliothek; neue Grafik; Split-Screen-Leistung (PF1).

## Regeln und Einschränkungen

- Domäne CLI (`src/`); RX1.4 berührt zusätzlich `tools/paper` und `public/grafik`-Specs (Entscheidung 005) und ist deshalb Domäne PAP.
- Kein Spiel-Zustand im Client; B nicht belegen; Seitenwechsel nur über `openPage()`.
- Log nur über `/api/clientlog` (Rate-Limit beachten), Emoji-Präfix laut CLAUDE.md.
- PNG und Manifeste sind nicht eingecheckt (B-737); Grafik-Golden pflegen.
- Keine Sim-, Balance- und Gen-Läufe in den Sessions (B-742); sie laufen im Abnahme-Ticket.
- Review vor dem Commit bei RX1.2 (Neuaufbau/Neuladen = Shell- und Pfad-Code).

## Beispiele

Xbox startet → Server-Log zeigt „🚀 Renderer: WebGL2, GPU …, maxTex …, N Texturen, ~X MiB“, danach alle 30 s „💓 FPS Ø …“. Kontext geht verloren → genau ein Neuaufbau oder Canvas-Neustart, Hinweis im Spiel, Spiel bleibt bedienbar.

## Ausnahme- und Fehlerfälle

Bestätigt der Xbox-Lauf (RX1.3) die Texturvermutung nicht, wird B-764 verworfen und die Ursache neu gesucht (Sprint wird angepasst). Verlust wiederholt sich sofort: keine Schleife, Canvas bleibt mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Startzeile mit allen Feldern für WebGL und Canvas (`B-762/AC-01`, Test).
- **AC-02** Kontextverlust und -wiederherstellung erzeugen je eine Zustandszeile (`B-762/AC-02`, Test).
- **AC-03** Leistungsverlauf höchstens alle 30 s, FPS-Warnung höchstens je Minute (`B-762/AC-03`, Test).
- **AC-04** Fehlgeschlagener Textur-Ladevorgang erzeugt Fehlerzeile mit Datei und Status (`B-762/AC-04`, Test).
- **AC-05** Ein simulierter Kontextverlust löst genau einen Neuaufbau oder Canvas-Neustart aus, ein zweiter keinen weiteren (`B-763/AC-01`, `B-763/AC-02`, Test).
- **AC-06** Kein generiertes Sheet ist größer als 2048 px je Kante (`B-764/AC-01`, Test).
- **AC-07** Der Client lädt beim Start nur die Texturen der ersten Stufe (`B-764/AC-02`, Test).
- **AC-08** Xbox-Lauf (🧑): Technik-Anzeige sichtbar, Startwerte, Leistungsverlauf und Ladedaten im Server-Log des Pi (`B-761/AC-05`, `B-762/AC-05`).
- **AC-09** Xbox bleibt nach Kontextverlust bedienbar, keine weiße Seite (`B-763/AC-03`).
- **AC-10** Xbox-Start ohne „WebGL Context lost“ im Server-Log, FPS-Wert notiert (`B-764/AC-03`).

## Offene Fragen

- Wiederherstellen des Kontexts oder Neuladen mit `?renderer=canvas`? Default: Neuaufbau versuchen, sonst Canvas-Neuladen; entscheidet RX1.2 beim Planen (🧑 informieren).
- B-764 verlangt Laden „bei Bedarf“ je Stufe: Ob die Gruppen-Indizes (`index.paper.json`) dafür schon je Stufe ladbar sind, klärt RX1.4 beim Planen; ist der Umbau groß, wird er ein eigenes Ticket.
- Welcher Browser-Parameter legt auf der Xbox die Texturgrenze fest (maxTex aus B-762)? Klärt RX1.3.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RX1.1 | `RX1.1-render-log.md` | Umsetzung | autonom | offen |
| RX1.2 | `RX1.2-kontextverlust.md` | Umsetzung | autonom | offen |
| RX1.3 | `RX1.3-xbox-lauf.md` | Workshop | Mensch | offen |
| RX1.4 | `RX1.4-texturprofil.md` | Umsetzung | autonom | offen |
| RX1.5 | `RX1.5-review.md` | Review | autonom | offen |
| RX1.6 | `RX1.6-xbox-abnahme.md` | Workshop | Mensch | offen |

Reihenfolge: RX1.1 → RX1.2 → RX1.3 → RX1.4 → RX1.5 → RX1.6 (RX1.2 loggt über RX1.1, RX1.3 braucht beide, RX1.4 braucht die Werte aus RX1.3).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
