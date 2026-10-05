/**
 * Deutsche Spieltexte (S5.3, B-172): Quelle der Schlüssel. `texts.en.ts` hat denselben Typ, der Compiler erzwingt alle Schlüssel.
 * Platzhalter stehen als `{name}`. Deutsch ist Standard und Rückfall.
 */
export const de = {
  // Netz (Client-Verbindung)
  'net.connecting': 'Verbinde …',
  'net.reconnecting': 'Verbindung weg, verbinde neu …',
  'net.lost': 'Server nicht erreichbar',
  'net.replaced': 'An anderer Stelle geöffnet',
  'net.version': 'Veraltete Version, Seite neu laden',
  'net.unknownBiome': 'Unbekanntes Biom, Seite neu laden',

  // Laden
  'load.progress': 'Lade Grafiken … {pct} %',
  'load.failed': 'Laden fehlgeschlagen:\n{files}\n\nSeite neu laden (F5) oder Server prüfen.',
  'stage.loading': 'Stufe wird geladen',
  'stage.loadingN': 'Stufe {depth} wird geladen',

  // Lobby
  'lobby.title': 'Family Three Crowns',
  'lobby.hint': 'Pfeile/Stick wählen · A / Enter / Tippen bestätigen',
  'lobby.play': 'Spielen  ({save})',
  'lobby.retry': 'Erneut versuchen',
  'lobby.reload': 'Seite neu laden',
  'lobby.room': '{code}  {name}  ·  Stufe {depth}  ·  {taken}/4 Plätze  ·  {state}',
  'lobby.running': 'läuft',
  'lobby.paused': 'pausiert',

  // HUD: Hinweise
  'hud.hint.touch': 'Links/rechts berühren = laufen · Münz-Taste halten = Münzen geben',
  'hud.hint.pad': 'A halten = Münzen geben · RT = sprinten · RS = Vollbild',
  'hud.hint.keyboard': 'Leertaste halten = Münzen geben · Shift = sprinten · F = Vollbild',
  'hud.join.touch': 'Münz-Taste drücken zum Beitreten',
  'hud.join.pad': 'A drücken zum Beitreten',
  'hud.join.keyboard': 'Leertaste drücken zum Beitreten',
  'hud.joinCenter': 'Drücke  A  (Controller), Leertaste oder die Münz-Taste zum Beitreten',
  'hud.room': 'Raum {code} · {name} · {n} Spieler',

  // HUD: Werte
  'hud.travelUp': 'Aufstieg in Tiefe {depth}',
  'hud.travelDown': 'Abstieg in Tiefe {depth}',
  'hud.skillPoints': 'Skill-Punkte {n}',
  'hud.wave': 'Welle {wave}: {n} Gegner',
  'hud.down': 'gefallen · zurück in {s} s',
  'hud.downShort': '{s} s',
  'hud.hp': 'HP {hp}',
  'hud.playerShort': 'P{p} · {gold}/{max} · {status} · {stage}',
  'hud.playerFull': 'P{p}  ·  Gold {gold}/{max}  ·  {status}  ·  {stage}',
  'hud.clockDay': 'Tag {day}  ·  Dämmerung in {left}',
  'hud.clockDusk': 'Dämmerung  ·  Nacht in {left}',
  'hud.clockNight': 'Nacht {day}  ·  Morgen in {left}',
  'hud.aggression': '  ·  Aggression {pct}%',

  // Meldungen (Ereignisse)
  'ev.dusk': 'Nacht naht!',
  'ev.night': 'Nacht {day}',
  'ev.dawn': 'Tag {day}  –  die Sonne geht auf',
  'ev.wave': 'Portal öffnet sich!\n{n} Gegner kommen',
  'ev.chest': 'P{p} findet {gold} Gold',
  'ev.skillPoint': 'Skill-Punkt gefunden!',
  'ev.armed': 'Neuer Bogenschütze',
  'ev.built': '{name} fertig',
  'ev.destroyed': '{name} zerstört!',
  'ev.goldStolen': 'P{p}: {amount} Gold geklaut!',
  'ev.playerDown': 'P{p} ist gefallen',
  'ev.castleFallen': 'Die Burg ist gefallen!\nGebäude, Truppen und die Hälfte der Vorräte sind verloren',

  // Welt
  'exit.deeper': 'Tiefe {depth}\nalle hierher',
  'exit.blocked': 'verschüttet',
  'site.waitingWorker': '{name}: wartet auf Bauer',
  'site.missing': '{n} {res} fehlt',
  'site.ready': 'bereit',
  'site.bow': 'Bogen',

  // Namen (Schlüssel = ID aus den Daten, Daten bleiben unverändert)
  'site.wall': 'Mauer',
  'site.tower': 'Turm',
  'site.gate': 'Tor',
  'site.workshop': 'Werkstatt',
  'site.storage': 'Lager',
  'site.stairsUp': 'Treppe hoch',
  'site.stairsDown': 'Treppe runter',
  'site.farm': 'Farm',
  'site.barracks': 'Kaserne',
  'site.tavern': 'Taverne',
  'site.healer': 'Heilplatz',
  'site.smithy': 'Schmiede',
  'site.armory': 'Rüstkammer',
  'res.wood': 'Holz',
  'res.stone': 'Stein',
  'res.copper': 'Kupfer',
  'biome.forest': 'Oberwelt (Wald)',
  'biome.cave': 'Tiefe 1 (Höhle)',
  'biome.mine': 'Tiefe 2 (Mine)',

  // Optionen und Pause
  'opt.title': 'Optionen',
  'opt.music': 'Musik',
  'opt.sfx': 'Effekte',
  'opt.screenshake': 'Wackeln',
  'opt.flash': 'Blitze',
  'opt.colorblind': 'Farbschwäche-Symbole',
  'opt.language': 'Sprache',
  'opt.guide': 'Hinweise (erste Nacht)',
  'opt.resume': 'Weiter',
  'opt.leave': 'Spiel verlassen',
  'opt.on': 'an',
  'opt.off': 'aus',
  'opt.hint': 'Hoch/Runter wählen · Links/Rechts ändern · A / Enter umlegen · Menu / Esc schließen',
  'opt.openButton': 'Optionen',
  // Skill-Menü und Skill-Leiste (S3.2)
  'skill.title': 'Skills  ·  {n} Punkte frei',
  'skill.learned': 'gelernt',
  'skill.learnable': 'lernen',
  'skill.locked': '–',
  'skill.passive': 'passiv',
  'skill.respec': 'Alle Skills zurücksetzen (Respec)',
  'skill.free': '–',
  'skill.ready': 'bereit',
  'skill.hint.pad': '◀ ▶ wählen · A bestätigen · {menu} schließen',
  'skill.hint.keyboard': 'A / D wählen · Leertaste bestätigen · {menu} schließen',
  'skill.hint.touch': 'Links/rechts wählen · Münz-Taste bestätigt · {menu} schließt',
  // Aktionen-Overlay (S3.3)
  'hint.hold': '{key} halten: {what}',
  'hint.press': '{key}: {what}',
  'hint.space': 'Leertaste',
  'hint.build': '{name} bauen',
  'hint.pay': '{name} kaufen',
  'hint.revive': 'Wiederbeleben',
  'hint.attack': 'Schlag',
  'hint.learn': 'Skill lernen',
  'hint.respec': 'Skills zurücksetzen',
  // Geführte erste Nacht (S6.3): Hinweise über den Objekten, {key} = Glyph
  'guide.coin': 'Hinlaufen: Münze aufheben',
  'guide.pay': '{key} halten: Bauplatz bezahlen',
  'guide.dusk': 'Die Nacht naht! Zurück zur Burg',
  'guide.recruit': '{key} halten: Münze geben, er wird Bauer',
  'lang.de': 'Deutsch',
  'lang.en': 'English',
};

export type TextKey = keyof typeof de;
