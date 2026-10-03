# W6 · CLI · Anzeigen für Bau, Lager, Hub und Bürger

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-117, B-126
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client zeigt weder Wartegrund, Lagerstand und Hub-Stufe noch Berufe, Händler, Truppen-Limit und Heilplatz (B-117, B-126).

## Ziel

Spieler sehen, warum ein Bauplatz wartet, Lagerstand und Hub-Stufe mit Kosten sowie Berufe, Händler, Limit und Heilplatz.

Am Ende sichtbar: HUD und Bauplätze am TV, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spieler (1 bis 4 am TV); 🧑 nimmt die Anzeige am Gerät ab.

## Anforderungen

B-117 › Anforderungen, B-126 › Anforderungen.

## Nicht-Ziele

Grafik-Anbindung (GR3), Ton, Protokoll (W5).

## Regeln und Einschränkungen

Der Client rechnet nichts und zeichnet nur Server-Zustand (`src/scenes/noSim.test.ts`); Logik als reine Funktionen mit Test. Mindest-Schriftgrößen je Split-Viertel nach den Regeln aus F1 (B-136). Seiten-Regeln aus `CLAUDE.md`; B nicht belegen, View + Menu reserviert. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne CLI.

## Beispiele

Bauplatz wartet auf Material → Symbol und Text „Material fehlt“ am Platz.

## Ausnahme- und Fehlerfälle

Händler nicht anwesend → keine Händler-Anzeige; Server-Feld fehlt (älterer Server) → Anzeige bleibt leer, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Reine Funktionen für Wartegrund, Lagerstand-Text, Limit-Text und Berufsanzeige sind getestet (`task test`) (B-117/AC-01, B-126/AC-01).
- **AC-02** HUD und Bauplätze zeigen Wartegrund, Lagerstand, Hub-Stufe und Kosten (B-117/AC-02).
- **AC-03** Berufe, Limit, Händler und Heilplatz werden angezeigt (B-126/AC-02).
- **AC-04** 🧑 hat beide Anzeigen am Gerät abgenommen (B-117/AC-03, B-126/AC-03).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W6.1 Reine Funktionen für Wartegrund, Lagerstand, Limit, Berufe mit Tests (AC-01).
- W6.2 HUD und Bauplätze: Wartegrund, Lager, Hub-Stufe (AC-02).
- W6.3 Bürger-UI: Berufe, Händler, Limit, Heilplatz (AC-03).
- W6.4 Review (Code-Sprint) und 🧑-Abnahme am Gerät (AC-04).

## Abnahme

–
