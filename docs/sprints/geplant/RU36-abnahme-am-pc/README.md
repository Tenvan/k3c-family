# RU36 · CLI · Abnahme am PC: Rust-Spiel gegen Go-Spiel (Gate G4)

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

Beide Spiele sind baubar; das Go-Spiel ist die Referenz.

## Ziel

🧑 nimmt das Rust-Spiel am PC mit zwei Pads ab. Am Ende sichtbar: Abnahme-Bericht mit Liste, Messwerten und Entscheidung G4.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 19 von 26, 5 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Abnahme-Liste aus den Stationen des Go-Spiels und Messlauf Rust gegen Go (AC-01, AC-02)
2. Abnahme am PC mit zwei Pads (Mensch, AC-03)
3. Befunde beheben, Runde 1 (AC-04)
4. Befunde beheben, Runde 2 (AC-04)
5. Review

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `docs/messung/rust-abnahme.md`, `rust/crates/**`
- **Abhängig von:** RU32, RU35
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Abnahme-Liste nennt jede Station des Go-Spiels (Menüs, Spiel, Nacht, Bau, Skills, Boss, Seekarte, Speichern, Laden).
- **AC-02** Der Messlauf zeigt FPS, Grafikspeicher und Startzeit beider Spiele am selben PC; Rust ist in keinem Wert schlechter als die Schwellen aus G1.
- **AC-03** 🧑 hat jede Station mit zwei Pads abgenommen oder als Befund notiert.
- **AC-04** Schwere Befunde sind behoben, die übrigen sind Tickets.

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
