# B-764 · Texturen auf die Xbox als Zielplattform auslegen (höchstens 2048 px, bei Bedarf laden)

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Papierschnitt-Grafik (`public/grafik`, 2054 PNG) enthält Sheets bis 3072×1920 px (z. B. `gebaeude/hg-wald-nah-v1..v5-128.png`, je ca. 23 MiB als RGBA im Speicher). Verdacht: Die Xbox verliert deshalb den WebGL-Kontext (B-763). Noch nicht bestätigt; die Werte liefert B-762.

## Ziel

Die Xbox (Edge) ist die Zielplattform für die Grafik: Es gibt nur ein Texturprofil, und es ist auf die Xbox ausgelegt. Der Texturbedarf bleibt so klein, dass die Xbox den WebGL-Kontext hält.

## Beteiligte und Zielgruppen

🧑 testet auf der Xbox; Claude wertet die Log-Werte aus.

## Anforderungen

- Erst nach B-762 und einem Xbox-Lauf: Ursache über Log-Werte bestätigen (max. Texturgröße, Texturspeicher).
- Die Xbox legt die Grenzen fest (max. Texturgröße und Texturspeicher aus den B-762-Werten); alle Sheets werden dafür erzeugt, vorerst höchstens 2048 px je Kante (Renderer `tools/paper`, Vertrag in Entscheidung 005).
- Texturen nur laden, wenn eine Stufe sie braucht, statt alle beim Start.
- Das Budget (Kante, Speicher je Stufe) steht als Regel an einer Stelle und wird im Test geprüft.

## Nicht-Ziele

Kein zweites Profil (kein Low-Profil für die Xbox, keine Zusatzauflösungen für andere Geräte), kein Wechsel der Grafikbibliothek, keine neue Grafik; B-763 (Kontextverlust abfangen) bleibt eigen.

## Regeln und Einschränkungen

PNG und Manifeste sind nicht eingecheckt (B-737), Änderung am Generator und an den Specs; Grafik-Golden pflegen; Sim/Balance/Gen-Läufe nur im Abnahme-Ticket (B-742).

## Beispiele

Waldhintergrund → Sheets höchstens 2048 px, Texturspeicher je Stufe sinkt; Xbox-Start ohne 'WebGL Context lost'.

## Ausnahme- und Fehlerfälle

Bestätigt B-762 die Texturvermutung nicht, wird dieses Ticket verworfen und die Ursache neu gesucht.

## Akzeptanzkriterien

- **AC-01** Test: Kein generiertes Sheet ist größer als 2048 px je Kante.
- **AC-02** Test: Der Client lädt beim Start nur die Texturen der ersten Stufe.
- **AC-03** Abnahme (🧑): Xbox-Start ohne 'WebGL Context lost' im Server-Log, FPS-Wert notiert.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
