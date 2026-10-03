# BAL3 · SIM · Bot-Profile, Sensitivität und Grad-Kurven

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-158
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach BAL1 und BAL2 gibt es zwei Bot-Profile und die Korridor-Prüfung. Weitere Spielweisen, Ein-Parameter-Läufe und Kurven je Schwierigkeitsgrad fehlen. Details in B-158.

## Ziel

Der Tester deckt weitere Spielweisen ab und zeigt, welche Kennzahlen bei einer Wertänderung kippen. Am Ende sichtbar: Berichte für fünf neue Profile, ein Sensitivitäts-Bericht und eine Kurve je Schwierigkeitsgrad.

## Beteiligte und Zielgruppen

🧑 wählt Profile und Fehlerrate des Kind-Bots; Entwickler und Agenten führen die Läufe aus; REG nutzt die Berichte in BR1 und BR2.

## Anforderungen

B-158 › Anforderungen.

## Nicht-Ziele

Lernende Bots, automatische Wertsuche, Abgleich mit echten Abenden (BAL4).

## Regeln und Einschränkungen

Deterministisch, kein `math/rand`; Bots nur über `PlayerCommand`; Werte ändert nur 🧑 mit Beschluss. Voraussetzung: BAL1, BAL2.

## Beispiele

Kind-Bot, 100 Seeds, 2 Spieler → eigener Bericht mit Überlebensquote; `economy.json` › `purse` +25 % → Liste gekippter Kennzahlen.

## Ausnahme- und Fehlerfälle

Variierter Wert fehlt in den Daten → Fehler mit Pfad. Profil verlangt mehr Spieler als das Szenario → Fehler beim Start.

## Akzeptanzkriterien

- **AC-01** Jedes der fünf neuen Profile liefert mit gleichem Seed und gleichen Daten byte-gleiche Kennzahlen (B-158/AC-01).
- **AC-02** Der Kind-Bot macht mit einem Seed immer dieselben Fehler, die Wahrscheinlichkeiten stehen in den Daten (B-158/AC-02).
- **AC-03** Ein Sensitivitäts-Lauf (±10 %, ±25 %) erzeugt einen Bericht mit gekippten Kennzahlen (B-158/AC-03).
- **AC-04** Der Bericht enthält je Schwierigkeitsgrad eine Kurve über die Tage (B-158/AC-04).
- **AC-05** Profile und Grad-Kurven stehen mit Beschluss von 🧑 in der Dokumentation des Testers (B-158/AC-05).
- **AC-06** `task check:go` ist grün.

## Offene Fragen

- Welche Profile und welche Fehlerrate des Kind-Bots? Entscheidet 🧑 (`docs/fragenkatalog.md` Q19); blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL3.1 🧑 Workshop (Agent: Mensch): Profile, Fehlerrate und Grad-Kurven beschließen, Beschluss dokumentieren (AC-05).
- BAL3.2 Fünf Profile inklusive Kind-Bot mit Fehlern aus Daten (AC-01, AC-02).
- BAL3.3 Sensitivitäts-Läufe und Kurven je Schwierigkeitsgrad (AC-03, AC-04).
- BAL3.4 Review (AC-06).

## Abnahme

–
