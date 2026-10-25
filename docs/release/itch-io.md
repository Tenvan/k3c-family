# itch.io-Seitentext (B-023, PB1)

Vorlage zum Einfügen auf der itch.io-Seite. Veröffentlicht nur Ralf (Session PB1.2). Nur Aussagen, die im Repo belegt sind
(`README.md`, `docs/game-design.md`, `LICENSE`, `LICENSE-ASSETS`, `public/sprites/CREDITS.md`).

## Titel

Family Three Crowns (K3C)

## Kurzbeschreibung (Tagline)

Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns: gemeinsam einen Hub ausbauen und nachts verteidigen. Läuft im Browser, auch auf der Xbox.

## Beschreibung

Family Three Crowns ist ein Koop-Strategie-Side-Scroller für die Couch. Die Monarchen bauen einen Hub aus, rekrutieren
Bürger und verteidigen ihn nachts gegen Gegner aus den Portalen. Gespielt wird im Browser (Xbox-Edge, PC, Handy); der
Spielserver läuft auf dem eigenen Windows-PC im Heimnetz und ist im Download enthalten.

**Hinweis:** Das Paket startet einen eigenen Server im Heimnetz. Es gibt keinen Online-Server für Fremde.

### Funktionen

- Koop für bis zu zwei Spieler pro Gerät (Split-Screen), weitere Geräte im selben Heimnetz, gemischt Couch und Online
- Hub ausbauen: Münzen geben, Bauplätze, Werkstatt, Bürger rekrutieren
- Nachts greifen Gegner an, Mauern halten sie auf, Bogenschützen schießen automatisch
- Tiefen-Eingänge am Levelende führen eine Stufe tiefer, Rückweg über eine gebaute Treppe
- Mehrere Räume laufen parallel auf dem Server
- Pixel-Art mit Figuren, Reittieren und Sound

### Steuerung

| Aktion | Xbox-Controller | Tastatur |
|---|---|---|
| Laufen | Linker Stick / D-Pad | A/D, Pfeile |
| Sprint | RT | Shift |
| Beitreten, Münzen geben, interagieren (halten) | A | Leertaste |
| Schlag | X | E |
| Skills 1–4 | LB, RB, LT, D-Pad hoch | Q, R, T, Z |
| Pause | Menu | Esc |
| Vollbild | Rechten Stick drücken | F |

### Systemvoraussetzungen

- Windows 10 oder 11 (64 Bit) für den Server und den Starter; WebView2-Laufzeit (in Windows 11 enthalten, ungeprüft für Windows 10)
- Ein aktueller Browser (Edge oder Chrome) auf dem Spielgerät, z. B. Edge auf der Xbox
- Heimnetz, in dem Spielgerät und PC verbunden sind; bei der Windows-Abfrage den Server für private Netzwerke zulassen
- Gamepad optional (Xbox-Controller), Tastatur geht ebenfalls

### Installation

1. `k3c-family-windows.zip` entpacken.
2. `k3c-starter.exe` starten, die angezeigte Adresse im Browser öffnen (PC: `http://localhost:8080`).
3. Details und Firewall-Hinweis: `LIES-MICH.txt` im Paket.

## Screenshots (Vorschlag)

Auf dem Branch mit den Präsentationsbildern liegen `site/img/spiel/*.jpg` (auf `sprint/pb1` noch nicht vorhanden); daraus
drei bis fünf auswählen, 630 x 500 px Cover separat zuschneiden.

## Lizenzen und Credits

- Code: `LICENSE` (PolyForm Noncommercial 1.0.0, nur nicht-kommerzielle Nutzung, source-available, nicht Open Source)
- Eigene Grafiken, Sounds, Musik: `LICENSE-ASSETS` (Namensnennung: „Ralf Stich, Family Three Crowns (K3C)“)
- Fremdmaterial behält seine Lizenz (CC0 bzw. CC-BY 3.0, u. a. LuizMelo, ansimuz Gothicvania, LPC-Tiere): `public/sprites/CREDITS.md` und Seite `lizenzen.html`
- Offene Frage aus B-023 (🧑): Erlauben alle Asset-Lizenzen die Weitergabe im Paket? Vor dem Veröffentlichen prüfen, besonders CC-BY-Credits und Sound-Quellen.
