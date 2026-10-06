# SO1.1 · Mixer mit Bussen und Lautstärke je Gerät

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** so1/1-mixer-lautstaerke
- **Abhängig von:** –
- **Tickets:** B-011
- **Kriterien:** AC-01, AC-02

## Ziel

`src/audio/` hat einen Mixer mit den Bussen Musik, Effekte und Ambient, je mit eigener Lautstärke; die Lautstärken werden je Gerät im `localStorage` gespeichert und beim Start gelesen.

## Kontext

- Es gibt keinen Audio-Code in `src/` (Plan § 3). Web Audio API: ein `AudioContext`, je Bus ein `GainNode` an einem Master-Gain. Kein Framework, keine neue Abhängigkeit (Phaser-Sound nur, wenn er die Busse ohne Umweg abbildet; Wahl im Ergebnis begründen).
- **Einstellungen:** S5.1 (B-146) legt einen Einstellungs-Speicher im `localStorage` an (Standard alles an, Lautstärke 100 %, Werte außerhalb 0–100 % begrenzt, kaputter Speicher → Standard). Ist S5.1 gelaufen, die Lautstärken dort einhängen statt eines zweiten Speichers; sonst eigener kleiner Speicher nach demselben Muster (try/catch) und Ticket „zusammenführen“ (CLI).
- Reine Teile (Lautstärke-Begrenzung, Lesen/Schreiben mit Rückfall) als Funktionen mit Vitest-Test; der `AudioContext` wird im Test gestellt (kein echtes Audio in Node).
- `src/scenes` rechnet nichts; der Mixer ist eine Klasse/ein Modul außerhalb der Szenen.

## Erlaubte Dateien

- `src/audio/` (neu, mit Tests)
- `src/core/` (nur Anbindung an den Einstellungs-Speicher aus S5, falls vorhanden)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Entsperren, Format, Sound-Atlas (SO1.2), Dämpfung und Demo-Ereignis (SO1.3), Optionen-Oberfläche (S5), Effekte und Musik (SO2, SO4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Mixer mit drei Bussen und Master; Lautstärke je Bus setzen/lesen.
3. Speicher je Gerät mit Rückfall auf Standard; Begrenzung 0–100 %.
4. Tests: getrennte Busse (Effekte 0 → Musik unverändert); Speichern und Lesen; gesperrter/kaputter Speicher → Standard, kein Fehler.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test belegt getrennte Busse Musik, Effekte, Ambient mit eigener Lautstärke.
- [x] AC-02: Test belegt Speichern und Lesen je Gerät im `localStorage` mit Rückfall.
- [x] `task check` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

Umgesetzt in `src/audio/mixer.ts` (Web Audio API, kein Phaser-Sound: Phaser hat keine eigenen Busse, ein `GainNode` je Bus ist der direkte Weg; keine neue Abhängigkeit). Die Lautstärken hängen in `src/core/settings.ts` (S5.1 lief schon): neues Feld `ambientVolume` (Standard 100 %), kein zweiter Speicher. Optionen-Eintrag für Ambient: B-218. Ticket-Nummern B-216/B-217 sind auf `sprint/s1` und `sprint/gr5` vergeben, daher B-218.

- AC-01: `src/audio/mixer.test.ts` (Busse am Master, Effekte 0 lässt Musik und Ambient unverändert, Begrenzung 0–100 %).
- AC-02: `src/audio/mixer.test.ts` (Speichern, neuer Mixer liest beim Start; gesperrter und kaputter Speicher → Standard, kein Fehler); `src/core/settings.test.ts` angepasst.
- `task check` grün (889 Tests); `mixer.ts` 67 Zeilen, keine Funktion > 60 Zeilen.
