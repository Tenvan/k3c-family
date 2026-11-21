# LA1.4 · Stimmung an den K2C-Screenshots abgleichen und die Abweichungen festhalten (Mensch)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/la1
- **Abhängig von:** LA1.1
- **Tickets:** B-1164
- **Kriterien:** AC-04

## Ziel

🧑 und der Agent haben an den K2C-Screenshots festgehalten, welche Merkmale der Stimmung im Rust-Spiel fehlen, mit Rangfolge, als Eingabe für LA1.5.

## Kontext

Gerät: PC. Ticket B-1164. Das Rust-Spiel läuft aus `dist/k3c-game`; Vergleich mit den K2C-Screenshots (Auswertung: `docs/standards/bildaufbau.md`, Notiz `reference_k2c-bildauswertung`). Der Agent legt vorher Vergleichsbilder (Rust-Aufnahmen gleicher Szenen, Tag, Dämmerung, Nacht) in `.work/` bereit; 🧑 nennt je Szene, was fehlt: Farbe, Licht, Dunst, Tiefe, Details, Himmel, Wasser. Ergebnis ist eine Liste mit Rangfolge unter `docs/messung/stimmung-abgleich.md`.

## Erlaubte Dateien

- `docs/messung/stimmung-abgleich.md`

## Nicht-Ziele

Codeänderungen, Umsetzung (LA1.5).

## Schritte

1. Agent: Vergleichsbilder und Merkmals-Raster vorbereiten.
2. 🧑: je Szene Abweichungen nennen, Rangfolge festlegen.
3. Agent: Liste mit Rangfolge eintragen, LA1.5 mit konkreten Punkten füllen; Status `fertig`.

## Fertig, wenn

- [x] AC-04: `docs/messung/stimmung-abgleich.md` nennt je Szene die fehlenden Merkmale in Rangfolge, abgenommen von 🧑.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-04 erfüllt: `docs/messung/stimmung-abgleich.md` nennt je Szene die fehlenden Merkmale in Rangfolge, abgenommen von 🧑 am 2026-11-21 (Rangliste: 1 Lichthöfe, 2 Nachthimmel, 3 Himmel mit Verlauf).
- Aufnahmen: 16 Rust-Bilder (Tag, Abend, Nacht, Morgengrauen; eine und zwei Ansichten) unter `.work/stimmung/` (nicht eingecheckt). Die K2C-Screenshots lagen nur bei 🧑, der Agent hat sie nicht gesehen.
- Nicht aufgenommen (kein Schalter in der EXE): Blutmond, dichter Wald mit Fackel, gerodeter Hub, Lava-Stufe.
- Abweichung vom Plan: Die Abgleich-Liste kam im Dialog aus Vorschlägen des Agenten statt aus einem Workshop am Gerät; Rang 1 und der Tausch von Rang 2 und 3 stammen von 🧑. Der Hinweis zu den Wellenstrichen (bei Bedarf in den K2C-Screenshots nachsehen) geht an LA1.5.
- Gerät: PC. Neue Tickets: keine.
