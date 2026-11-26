# B-1202 · ECO1 ist am PC abgenommen: Siegel, Werkzeuggürtel, tiefe Ebenen, Minen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint ECO1 (B-1191 bis B-1198) liefert Schwierigkeits-Siegel (Papierschnitt, Lobby, Spielstandliste, HUD), Werkzeuggürtel und Handwerker, Produktion nur in Stufe 0 (`topOnly`), Besorgung von Ausrüstung von oben und Erzadern erst nach dem Freiräumen. Gen-, Sim- und Race-Läufe sowie die Sichtprüfung am PC sind nach B-742 ausgelagert und stehen hier.

## Ziel

ECO1 ist am PC gesehen und gemessen abgenommen; offene 50:50-Entscheidungen sind von Ralf entschieden.

## Beteiligte und Zielgruppen

Ralf (🧑) prüft am PC mit Pad und entscheidet die offenen Fragen; Claude fährt die Läufe.

## Anforderungen

- **AC-01** Sim: nur über `sim_test`, blockierend gewartet; keine Regression gegen den Stand vor ECO1.
- **AC-02** Balance: Werkzeuggürtel und Handwerker-Anteil spielen sich ohne Stau (Ralf am PC).
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün (fünf Siegel-Icons `siegel-*`).
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Sichtprüfung Siegel: Lobby (Grad-Zeile), Spielstandliste und HUD-Radar zeigen das Siegel passend zum Grad; Hauptmenü „Weiter“ ohne Siegel.
- **AC-06** Sichtprüfung Besorgung: Bürger ohne Werkstatt in tieferer Stufe holen Ausrüstung von oben und kehren zurück, ohne Pendel-Schleife.
- **AC-07** Bekannte rote Tests aus B-1190 AC-04 sind unverändert; neu rot: `still_hud pad-tastatur` (4,020 > 4,0) und `ablauf_wie_go_zwei_laeufe_gleich` sind zu klären.

## Nicht-Ziele

Neue Mechaniken, neues Balancing über ECO1 hinaus.

## Regeln und Einschränkungen

B-742 (Läufe nur im Abnahme-Ticket), Sim-Sperre (Beschluss 2026-11-11), Entscheidung 013.

- **Bereich:** `rust/**`, `data/**`
- **Abhängig von:** B-1191, B-1192, B-1193, B-1194, B-1195, B-1196, B-1197, B-1198
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Hard-Spielstand laden: Liste zeigt das Hard-Siegel (drei Punkte), im HUD steht es links neben der Biome-Zeile.

## Ausnahme- und Fehlerfälle

Unbekannter Grad: Siegel `normal` als Rückfall.

## Akzeptanzkriterien

- **AC-01** Sim über `sim_test` ohne Regression.
- **AC-02** Balance am PC ohne Stau durch Werkzeuggürtel und Handwerker.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Siegel in Lobby, Liste und HUD sichtbar, Hauptmenü ohne Siegel.
- **AC-06** Besorgung von oben ist sichtbar und ohne Pendel-Schleife.
- **AC-07** Rote Tests `still_hud pad-tastatur` und `ablauf_wie_go_zwei_laeufe_gleich` geklärt.

## Offene Fragen

🧑 Ralf entscheidet (50:50, im Nachgang): Taverne als Produktion (`topOnly`) und Bogner unten erlaubt? Rückkehr der Besorger ohne sichtbaren Weg ok? Truppen-Limit unten?

## Notizen

Muster: B-1190.
