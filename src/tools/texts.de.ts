/**
 * Deutsche Texte der Werkzeug-Seiten (PL2, B-322): Quelle der Schlüssel, nach Seite gruppiert. Platzhalter stehen als `{name}`.
 * Deutsch ist Standard und Rückfall.
 */
export const de = {
  // Monitoring-Seite (monitor.html)
  'monitor.pageTitle': 'K3C Monitor',
  'monitor.connecting': 'Verbinde …',
  'monitor.saveToken': 'Token speichern',
  'monitor.tab.overview': 'Übersicht',
  'monitor.tab.history': 'Verlauf',
  'monitor.tab.events': 'Ereignisse',
  'monitor.h.tick': 'Tick-Dauer (max je s)',
  'monitor.h.rtt': 'RTT je Gerät',
  'monitor.h.server': 'Server: Heap und CPU',
  'monitor.legend': 'Rote Punkte: Ausreißer. Striche unten: Diagnose-Ereignisse. Gestrichelt: Neustart. ▼: gewähltes Ereignis.',
  'monitor.keys': 'Controller: Steuerkreuz/Stick wählt, A bestätigt. Touch und Maus: antippen.',
  'monitor.kind.crash': '💥 Absturz',
  'monitor.kind.slow': '🐢 langsamer Tick',
  'monitor.kind.drop': '👋 Trennung',
  'monitor.kind.client': '📨 Client-Fehler',
  'monitor.rooms': 'Räume (letzte 5 min)',
  'monitor.noRoom': 'Kein Raum aktiv.',
  'monitor.noPong': 'kein Pong',
  'monitor.roomLine': 'Tick p99 {tick} · RTT {rtt} · Fehler {errors}',
  'monitor.serverLine': 'Heap {heap} MB · CPU {cpu} · Goroutinen {goroutines}',
  'monitor.restart': 'Neustart',
  'monitor.heapMB': 'Heap MB',
  'monitor.all': 'alle',
  'monitor.roomFilter': 'Raum: {room}',
  'monitor.kindFilter': 'Art: {kind}',
  'monitor.window1h': 'Fenster 1 h',
  'monitor.window5m': 'Fenster 5 min',
  'monitor.overBudget': 'Budget überschritten',
  'monitor.ticks': '{n} Ticks',
  'monitor.noEvents': 'Keine Ereignisse in der letzten Stunde.',
  'monitor.eventRoom': ' · Raum {room}',
  'monitor.tokenAsk': '🔒 Token eingeben (K3C_STATUS_TOKEN des Servers).',
  'monitor.tokenBad': '🔒 Token fehlt oder ist falsch, bitte neu eingeben.',
  'monitor.ok': '✅ Stand {now} · Server seit {since}',
  'monitor.off': '🚫 Diagnose aus, K3C_STATUS_TOKEN am Server setzen.',
  'monitor.offline': '❌ Server nicht erreichbar, nächster Versuch in {wait} s.',

  // Grafik-Seite (grafiken.html) und Credits (lizenzen.html)
  'credits.work': 'Werk',
  'credits.author': 'Urheber:innen',
  'credits.license': 'Lizenz',
  'credits.source': 'Quelle',
  'credits.attribution': 'Namensnennung',

  // Developer page (dev.html) and its tiles; the tiles also appear in the game menu
  'dev.pageTitle': 'K3C Entwicklung',
  'dev.title': 'Entwicklung',
  'dev.sub': 'Werkzeuge für Entwicklung, Performance und Balancing. Die meisten brauchen den laufenden Go-Server.',
  'dev.section.dev': 'Entwicklung',
  'dev.section.perf': 'Performance',
  'dev.section.balance': 'Balancing',
  'dev.empty': 'Noch keine Aufrufe.',
  'tool.opening': 'Öffne „{title}“ …',
  'tile.gamedev.title': 'Spiel (Dev)',
  'tile.gamedev.desc': 'Spiel mit Debug-Overlay (?dev=1) · window.game für die Konsole',
  'tile.monitor.title': 'Monitor',
  'tile.monitor.desc': 'Serverzustand: Ampel je Raum, Verläufe mit Perzentilen, Fehler-Zeitleiste (Go-Server mit K3C_STATUS_TOKEN)',

  // Selection bar of the reference pages
  'sprite.ours': 'Unsere Seite',
  'sprite.enemy': 'Gegenseite',
  'sprite.mountsKing': 'Reittiere (mit König 1)',
  'sprite.monarchs': 'Monarchen',
  'sprite.troops': 'Truppen',
  'sprite.enemies': 'Gegner',
  'sprite.mountsLater': 'Reittiere (noch nicht im Spiel)',
  'sprite.reserve': 'Reserve (noch ohne Rolle)',
  'sprite.noRole': 'noch ohne Rolle',
  'sprite.tinted': 'eingefärbt {tint}',
  'sprite.translucent': 'durchscheinend',
  'sprite.player': 'Spieler {n}',
  'sprite.depth': '{name} · Tiefe {depth}',
  'sprite.height': '{id} · im Spiel {px} px hoch',

  // Licenses page (lizenzen.html); texts with HTML are set via data-t-html
  'lic.pageTitle': 'K3C Lizenzen & Danksagung',
  'lic.title': 'Lizenzen & Danksagung',
  'lic.sub':
    'Family Three Crowns (K3C) ist ein Familienprojekt und steht auf den Schultern vieler großzügiger Menschen. Hier steht, unter welcher Lizenz unser Spiel läuft und wem wir für Grafiken, Bibliotheken und Werkzeuge danken. Scrollen mit Stick oder Steuerkreuz.',
  'lic.h.ours': 'Unsere Lizenz',
  'lic.boxCode':
    '<strong>Quelltext:</strong> <a href="https://polyformproject.org/licenses/noncommercial/1.0.0" target="_blank" rel="noopener">PolyForm Noncommercial License 1.0.0</a> · Datei <code>LICENSE</code><br /><strong>Eigene Grafiken, Animationen, Sounds und Musik:</strong> <a href="https://creativecommons.org/licenses/by-nc/4.0/deed.de" target="_blank" rel="noopener">CC BY-NC 4.0</a> · Datei <code>LICENSE-ASSETS</code>',
  'lic.boxCopy':
    'Copyright © 2026 Ralf Stich. Der Quelltext ist offen einsehbar (<em>source-available</em>), aber <strong>nicht</strong> frei für kommerzielle Nutzung.',
  'lic.ok1': 'Spielen, Lernen, Ausprobieren, Verändern und Weitergeben zu privaten, familiären, Hobby-, Forschungs- und Bildungszwecken.',
  'lic.ok2': 'Weitergabe mit Namensnennung und mit dem Lizenztext.',
  'lic.no1': 'Verkaufen, Vermieten, Werbung oder Bezahlmodelle, Einbau in ein kommerzielles Produkt oder eine bezahlte Dienstleistung.',
  'lic.dim1': 'Kommerzielle Nutzung ist nur mit schriftlicher Erlaubnis von Ralf Stich möglich. Es gilt der jeweilige Lizenztext, diese Zusammenfassung ersetzt ihn nicht.',
  'lic.dim2':
    'Die Nicht-kommerziell-Klausel gilt nur für unsere eigene Arbeit. Das Fremdmaterial unten behält seine ursprüngliche Lizenz und bleibt für alle so nutzbar, wie die Urheber:innen es erlaubt haben.',
  'lic.h.graphics': 'Grafiken',
  'lic.graphics':
    'Alle Grafiken im Spiel sind <strong>eigener Papierschnitt</strong>, aus Specs erzeugt (<code>tools/paper</code>). Es steckt keine Fremdgrafik darin, deshalb gibt es hier keine Grafik-Credits.',
  'lic.creditsNote':
    'Die Einträge stehen in <code>public/audio/README.md</code>; diese Seite liest sie von dort.',
  'lic.h.sound': 'Sound & Musik',
  'lic.sound':
    'Soundeffekte und Musik stammen aus freien Quellen oder sind selbst aufgenommen; Urheber:innen, Lizenz und Quelle stehen unten, eigene Aufnahmen unter CC BY-NC 4.0.',
  'lic.h.software': 'Software',
  'lic.softwareIntro': 'Diese Bausteine laufen im Spiel oder bauen es. Danke an alle, die sie pflegen.',
  'lic.th.part': 'Baustein',
  'lic.th.purpose': 'Wofür',
  'lic.th.license': 'Lizenz',
  'lic.sw.phaser': 'Spiel-Engine im Browser',
  'lic.sw.ws': 'WebSocket für den Online-Modus (Node-Server)',
  'lic.sw.coder': 'WebSocket im Go-Server',
  'lic.sw.vite': 'Dev-Server und Build',
  'lic.sw.ts': 'Programmiersprache',
  'lic.sw.tests': 'Tests und Lint',
  'lic.sw.go': 'Server und Spiel-Engine der Zukunft',
  'lic.softwareNote':
    'Weitere Werkzeuge nur für die Entwicklung (z. B. in <code>tools/k3c-workbench</code>) liefern wir nicht mit dem Spiel aus; ihre Lizenzen liegen in den jeweiligen Paketen.',
  'lic.h.thanks': 'Danke',
  'lic.thanks':
    'Von Herzen an <strong>LuizMelo</strong>, <strong>ansimuz</strong>, <strong>bluecarrot16</strong>, <strong>Jordan Irwin (AntumDeluge)</strong>, <strong>lawnjelly</strong>, <strong>Calciumtrice</strong>, <strong>lemmling</strong>, <strong>Bonsaiheldin</strong>, <strong>vico</strong>, <strong>Rusher_go</strong> und <strong>WolfTech</strong>, an die Plattformen <strong>itch.io</strong> und <strong>OpenGameArt.org</strong> und an alle Open-Source-Entwickler:innen oben. Wer Freude an ihren Werken hat: Schaut bei ihnen vorbei und unterstützt sie, wenn ihr könnt.',
  'lic.thanksNote': 'Fehlt eine Nennung oder ist etwas falsch zugeordnet? Bitte melden, wir korrigieren es umgehend.',
} as const satisfies Record<string, string>;

export type ToolTextKey = keyof typeof de;
