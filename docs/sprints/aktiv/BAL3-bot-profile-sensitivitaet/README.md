# BAL3 · SIM · Bot-Profile, Sensitivität und Grad-Kurven

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-158
- **Start-Commit:** ada3483
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Nach BAL1 und BAL2 gibt es zwei Bot-Profile und die Korridor-Prüfung. Weitere Spielweisen, Ein-Parameter-Läufe und Kurven je Schwierigkeitsgrad fehlen. Details in B-158.

## Ziel

Der Tester deckt weitere Spielweisen ab und zeigt, welche Kennzahlen bei einer Wertänderung kippen. Am Ende sichtbar: Berichte für vier neue Profile, ein Sensitivitäts-Bericht und eine Kurve je Schwierigkeitsgrad.

## Beteiligte und Zielgruppen

🧑 wählt Profile (Q19); Entwickler und Agenten führen die Läufe aus; REG nutzt die Berichte in BR1 und BR2.

## Anforderungen

B-158 › Anforderungen.

## Nicht-Ziele

Lernende Bots, automatische Wertsuche, Abgleich mit echten Abenden (BAL4), Kind-Bot (Q19: später).

## Regeln und Einschränkungen

Deterministisch, kein `math/rand`; Bots nur über `PlayerCommand`; Werte ändert nur 🧑 mit Beschluss. Voraussetzung: BAL1, BAL2.

## Beispiele

„Koop 2 Spieler“, 100 Seeds → eigener Bericht mit Überlebensquote; `economy.json` › `purse` +25 % → Liste gekippter Kennzahlen.

## Ausnahme- und Fehlerfälle

Variierter Wert fehlt in den Daten → Fehler mit Pfad. Profil verlangt mehr Spieler als das Szenario → Fehler beim Start.

## Akzeptanzkriterien

- **AC-01** Jedes der vier neuen Profile liefert mit gleichem Seed und gleichen Daten byte-gleiche Kennzahlen (B-158/AC-01).
- **AC-02** Kein Kind-Bot: Er ist Nicht-Ziel (Q19, später); der Tester enthält kein Fehler-Profil und keine Fehler-Daten dafür (B-158/AC-02).
- **AC-03** Ein Sensitivitäts-Lauf (±10 %, ±25 %) erzeugt einen Bericht mit gekippten Kennzahlen (B-158/AC-03).
- **AC-04** Der Bericht enthält je Schwierigkeitsgrad eine Kurve über die Tage (B-158/AC-04).
- **AC-05** Profile und Grad-Kurven stehen mit Beschluss von 🧑 in der Dokumentation des Testers (B-158/AC-05).
- **AC-06** `task check:dev` ist grün (der Tester liegt unter `tools/k3c-dev`, B-099).

## Offene Fragen

- Keine. Profile geklärt durch Q19 (`docs/fragenkatalog.md`), Kind-Bot später.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BAL3.1 | `BAL3.1-workshop-profile.md` | Workshop | Mensch | fertig |
| BAL3.2 | `BAL3.2-profile-kind-bot.md` | Umsetzung | autonom | offen |
| BAL3.3 | `BAL3.3-sensitivitaet-kurven.md` | Umsetzung | autonom | offen |
| BAL3.4 | `BAL3.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
