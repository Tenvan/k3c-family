# B-1040 · Hub-Gebäude folgen den Monarchenhöhen, die Burg ist das größte Gebäude

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BB1
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 2: In K2C ist das Haupthaus 3,5–4,5 M hoch, Türme 2–2,5 M, Zelte und Werkstätten 1–1,4 M. Bei uns liegen Schmiede, Taverne und Dojo bei 1,2–1,4 M, die Burg ist nicht deutlich größer.

## Ziel

Die Burg ist das Zentrum des Hubs und überragt alles; die Höhen der Gebäude sind stimmig.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`. Abnahme: 🧑.

## Anforderungen

- Höhen aller Hub-Gebäude, Mauern und Türme (je Stufe) in M messen und als Tabelle ins Ergebnis schreiben.
- Burg (höchste Stufe) auf 3,5–4,5 M, Türme 2–2,5 M, Mauern 0,8–1,2 M, Werkstätten 1–1,4 M angleichen.
- Bauplatz-Breiten und Gerüste prüfen; Überlappungen als Ticket.

## Nicht-Ziele

Neue Gebäude, Ausbau-Stufen, Hintergrundmauer-Logik.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/gebaeude-paper/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe, Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Hub der Stufe 3: Burg ragt über Türme und Werkstätten, das Lagerfeuer davor.

## Ausnahme- und Fehlerfälle

Bauwerke der Welt (Dojo, Schrein) bleiben Werkstatt-Größe.

## Akzeptanzkriterien

- **AC-01** Messtabelle aller Hub-Gebäude in M im Ergebnis.
- **AC-02** Abweichungen über 20 % vom Richtwert sind angeglichen, `k3c-grafik` ohne schweren Befund.
- **AC-03** Am TV (🧑): Hub wirkt gestaffelt, Burg ist das größte Gebäude.
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 2, 3, 8, 16).
