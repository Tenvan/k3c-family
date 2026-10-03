# B-158 · Der Tester kennt weitere Bot-Profile, Sensitivitäts-Läufe und Kurven je Schwierigkeitsgrad

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** BAL3
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BAL1 liefert mindestens zwei Bot-Profile (B-099/AC-02), BAL2 die Zielkorridor-Prüfung (B-157). Die Schwierigkeitsgrade stehen in `data/difficulty.json`. Es gibt keine Läufe, die einen Wert variieren, und keine Profile für Koop mit 2 oder 4 Spielern oder für ungeübte Spieler.

## Ziel

Der Tester deckt mehr Spielweisen ab (Wirtschaft zuerst, Mauern zuerst, Koop 2 und 4 Spieler, Kind-Bot mit Fehlern), zeigt per Sensitivitäts-Lauf, welche Kennzahlen bei einer Wertänderung kippen, und liefert Kurven je Schwierigkeitsgrad. Nutzen: Werte werden gegen echte Spielweisen geprüft, nicht gegen einen einzigen Bot.

## Beteiligte und Zielgruppen

🧑 wählt die Profile und Grad-Kurven (Q19 in `docs/fragenkatalog.md`); Entwickler und Agenten führen die Läufe aus; REG nutzt die Berichte in BR1 und BR2.

## Anforderungen

- Profile: „Wirtschaft zuerst“, „Mauern zuerst“, „Koop 2 Spieler“, „Koop 4 Spieler“, „Kind-Bot mit Fehlern“; jedes spielt nur über `PlayerCommand`.
- Der Kind-Bot macht Fehler (verpasste Zahlung, späte Reaktion) mit Wahrscheinlichkeiten aus Daten und nur über `engine/rng`.
- Sensitivität: Ein-Parameter-Variation eines Werts aus `data/*.json` um ±10 % und ±25 % mit Bericht, welche Kennzahlen aus dem Korridor kippen; es wird nichts automatisch geändert.
- Grad-Kurven: je Schwierigkeitsgrad aus `data/difficulty.json` Kennzahlen über die Tage (z. B. Überleben je Welle) als Tabelle im Bericht.

## Nicht-Ziele

Lernende Bots, automatische Wertsuche (B-099 „Später, optional“), Abgleich mit echten Abenden (B-160).

## Regeln und Einschränkungen

Deterministisch, kein `math/rand`; Werte ändern nur 🧑 mit Beschluss; Datei ≤ 400 Zeilen, Funktion ≤ 60. Voraussetzung: BAL1, BAL2.

## Beispiele

Profil „Kind-Bot“ mit Fehlerrate aus den Daten, 100 Seeds, 2 Spieler → Überleben Welle 3 liegt unter dem des Profils „sparsam“; ein Lauf mit `data/economy.json` › `purse` +25 % nennt die gekippten Kennzahlen.

## Ausnahme- und Fehlerfälle

Variierter Wert existiert nicht in den Daten → Fehler mit Pfad, kein stiller Lauf. Profil verlangt mehr Spieler als das Szenario → Fehler beim Start.

## Akzeptanzkriterien

- **AC-01** Test: Jedes der fünf neuen Profile liefert mit gleichem Seed und gleichen Daten byte-gleiche Kennzahlen.
- **AC-02** Test: Der Kind-Bot macht mit einem Seed immer dieselben Fehler; die Fehlerwahrscheinlichkeiten stehen in den Daten.
- **AC-03** Ein Sensitivitäts-Lauf (±10 %, ±25 % eines Werts) erzeugt einen Bericht mit der Liste gekippter Kennzahlen.
- **AC-04** Der Bericht enthält je Schwierigkeitsgrad aus `data/difficulty.json` eine Kurve der Kennzahlen über die Tage.
- **AC-05** Die gewählten Profile und Grad-Kurven stehen mit Beschluss von 🧑 in der Dokumentation des Testers.

## Offene Fragen

- Welche Profile und welche Fehlerrate des Kind-Bots? Entscheidet 🧑, `docs/fragenkatalog.md` Q19.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene B, BAL3. Grade stammen aus SP13.
