# RU39 · CLI · Ruckler-Messung: Rust-Client in Wiedergabe gegen das Go-Spiel (Gate G2)

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach RU20 zeichnet der Rust-Client Welt, Figuren, Gebäude, Effekte und HUD aus einer Aufzeichnung. Damit lässt sich die Frage des Projekts beantworten, bevor die Sim portiert wird.

## Ziel

Belegt ist, ob der Rust-Client bei gleicher Szene ruckelfrei läuft und auf einem schwachen PC weniger einbricht als das Go-Spiel. Am Ende sichtbar: Bericht `docs/messung/rust-ruckler.md`, Entscheidung G2.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 8 von 26, 3 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Messlauf beider Programme mit derselben Szene auf dem Entwicklungs-PC und dem schwächsten verfügbaren PC (AC-01, AC-02)
2. Sichtprüfung nebeneinander (Mensch, AC-03)
3. Bericht und Review

### Hinweise für die Umsetzung

- Schwelle (Vorschlag): längstes Bild höchstens das Doppelte des Medians, kein Bild über dem 1,5-Fachen des Medians, das das Programm verursacht.
- Ist Rust hier nicht sichtbar besser als das Go-Spiel nach B-943, empfiehlt der Bericht den Abbruch: Bis hier sind es rund 70 Sessions, danach noch rund 140.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `docs/messung/rust-ruckler.md`, `native/measure.ps1`, `rust/crates/k3c-game/src/replay/**`
- **Abhängig von:** RU38, RU16, RU17, RU20
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Dieselbe Szene (Nacht mit Gegnern, zwei Spieler, 5 Minuten) läuft im Go-Spiel und in der Rust-Wiedergabe; je Programm stehen Median, p99, p99,9, längstes Bild, Bilder über dem 1,5-Fachen des Medians, Rechenzeit je Bild, Grafikspeicher und Arbeitsspeicher im Bericht.
- **AC-02** Dieselben Werte liegen für den schwächsten verfügbaren PC vor (Mini-PC); fehlt das Gerät, steht „angenommen, Validierung offen“ im Bericht.
- **AC-03** 🧑 hat beide Programme nebeneinander gesehen und G2 entschieden: weiter mit dem Sim-Port, Abbruch oder Nacharbeit.

## Offene Fragen

Entscheidung 🧑 am Gate bzw. vor dem Start (B-1014 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
