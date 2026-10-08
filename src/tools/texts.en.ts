import type { ToolTextKey } from './texts.de';

/** Englische Texte der Werkzeug-Seiten (PL2, B-322); ein fehlender Schlüssel fällt auf Deutsch zurück. */
export const en: Partial<Record<ToolTextKey, string>> = {
  // Level viewer (leveltest.html)
  'level.pageTitle': 'K3C Level Viewer',
  'level.title': 'Level Viewer',
  'level.sub': 'Shows the level a room would get with this seed and biome, without playing. Requires the Go server to be running',
  'level.seed': 'Seed',
  'level.biome': 'Biome',
  'level.random': 'Random seed',
  'level.load': 'Load',
  'level.start': 'Start in game',
  'level.keys': 'Scroll: left stick or arrow keys · Zoom: LB/RB or + / − · Select: D-pad up/down, press A',
  'level.replaces': 'Starting opens the save named after the seed: if it exists it continues, otherwise a new game with this level begins.',
  'level.cannotStart': 'Cannot start in game: {reason}.',
  'level.chunkTitle': 'Chunk {index}: {kind} ({from}–{to} units)',
  'level.objectTitle': '{kind} at {x} units',
  'level.hubTitle': 'Hub center at {x} units',
  'level.loading': 'Loading …',
  'level.summary': 'Seed “{seed}” · {biome} · {width} units · {chunks} chunks · {objects} objects',
  'level.incomplete': 'Incomplete response',
  'level.forestOnly': 'Start only works for the forest (depth 0)',
  'level.badSeed': 'Seed must consist of lowercase letters, digits and - (at most 32 characters)',
  'level.unreachable': 'Server not reachable. Start it with `task start`.',
  'level.invalid': 'The server response is not a valid level.',
  'level.httpError': 'Error {status}',

  // Sound test (soundtest.html)
  'sound.pageTitle': 'K3C Sound Test',
  'sound.title': 'Sound Test',
  'sound.sub': 'Listen to music and effect candidates, ordered by state and event, with source and license.',
  'sound.pressKey': 'Press a key to unlock',
  'sound.stop': 'Stop',
  'sound.fade': 'Crossfade',
  'sound.volDown': 'Quieter −',
  'sound.volDownAria': 'Quieter',
  'sound.volUp': 'Louder +',
  'sound.volUpAria': 'Louder',
  'sound.keys':
    'Controller: D-pad/stick selects, A plays, X stops, Y crossfades to the selected track, LB/RB volume. ' +
    'Keyboard: arrows up/down select, Enter plays, S stops, X crossfades, arrows left/right or −/+ volume. Touch: tap an entry, buttons on top.',
  'sound.kindState': 'State',
  'sound.kindEvent': 'Event',
  'sound.failed': '{name} (does not load)',
  'sound.meta': 'Source: {source} · License: {license}',
  'sound.playing': 'Playing: {name}',
  'sound.silence': 'Silence',
  'sound.status': '{state} · Volume {volume} %',
  'sound.noFormat': 'no playable format',
  'sound.unlocked': 'Audio unlocked',
  'sound.locked': 'Audio locked: press a key, tap the screen or press A on the controller',
};
