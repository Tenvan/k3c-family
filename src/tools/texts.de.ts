/**
 * Deutsche Texte der Werkzeug-Seiten (PL2, B-322): Quelle der Schlüssel, nach Seite gruppiert. Platzhalter stehen als `{name}`.
 * Deutsch ist Standard und Rückfall.
 */
export const de = {
  // Level-Betrachter (leveltest.html)
  'level.pageTitle': 'K3C Level-Betrachter',
  'level.title': 'Level-Betrachter',
  'level.sub': 'Zeigt das Level, das ein Raum mit diesem Seed und Biom bekäme, ohne zu spielen. Voraussetzung: der Go-Server läuft',
  'level.seed': 'Seed',
  'level.biome': 'Biom',
  'level.random': 'Zufälliger Seed',
  'level.load': 'Laden',
  'level.start': 'Im Spiel starten',
  'level.keys': 'Scrollen: linker Stick oder Pfeiltasten · Zoom: LB/RB oder + / − · Auswahl: Steuerkreuz hoch/runter, A drücken',
  'level.replaces': 'Der Start öffnet den Spielstand mit dem Seed als Namen: gibt es ihn schon, wird er weitergespielt, sonst entsteht ein neues Spiel mit diesem Level.',
  'level.cannotStart': 'Im Spiel starten nicht möglich: {reason}.',
  'level.chunkTitle': 'Abschnitt {index}: {kind} ({from}–{to} Units)',
  'level.objectTitle': '{kind} bei {x} Units',
  'level.hubTitle': 'Hub-Mitte bei {x} Units',
  'level.loading': 'Lade …',
  'level.summary': 'Seed „{seed}“ · {biome} · {width} Units · {chunks} Abschnitte · {objects} Objekte',
  'level.incomplete': 'Antwort unvollständig',
  'level.forestOnly': 'Start nur für den Wald (Tiefe 0)',
  'level.badSeed': 'Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 32 Zeichen)',
  'level.unreachable': 'Server nicht erreichbar. Starte ihn mit `task start`.',
  'level.invalid': 'Die Antwort des Servers ist kein gültiges Level.',
  'level.httpError': 'Fehler {status}',

  // Hörprobe (soundtest.html)
  'sound.pageTitle': 'K3C Hörprobe',
  'sound.title': 'Hörprobe',
  'sound.sub': 'Kandidaten für Musik und Effekte anhören, nach Zustand und Ereignis geordnet, mit Quelle und Lizenz.',
  'sound.pressKey': 'Taste drücken zum Entsperren',
  'sound.stop': 'Stopp',
  'sound.fade': 'Überblenden',
  'sound.volDown': 'Leiser −',
  'sound.volDownAria': 'Leiser',
  'sound.volUp': 'Lauter +',
  'sound.volUpAria': 'Lauter',
  'sound.keys':
    'Controller: Steuerkreuz/Stick wählt, A spielt, X stoppt, Y überblendet das gewählte Stück ein, LB/RB Lautstärke. ' +
    'Tastatur: Pfeile hoch/runter wählen, Enter spielt, S stoppt, X überblendet, Pfeile links/rechts oder −/+ Lautstärke. Touch: Eintrag antippen, Knöpfe oben.',
  'sound.kindState': 'Zustand',
  'sound.kindEvent': 'Ereignis',
  'sound.failed': '{name} (lädt nicht)',
  'sound.meta': 'Quelle: {source} · Lizenz: {license}',
  'sound.playing': 'Spielt: {name}',
  'sound.silence': 'Stille',
  'sound.status': '{state} · Lautstärke {volume} %',
  'sound.noFormat': 'kein abspielbares Format',
  'sound.unlocked': 'Audio entsperrt',
  'sound.locked': 'Audio gesperrt: Taste drücken, Bildschirm antippen oder A am Controller',
} as const satisfies Record<string, string>;

export type ToolTextKey = keyof typeof de;
