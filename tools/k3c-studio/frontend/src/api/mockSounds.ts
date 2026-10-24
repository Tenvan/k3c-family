import type { AssetChoice, AssetSlot } from './types';

// Zusätzliche Sound-Slots des Mocks (WZ30.2): genug fürs Register der Szenen Skills, Gegner und Bosse, Bauen, Reiten, UI und
// Warnungen samt Misch-, Lautheits- und Abstandsprobe. Dateien wie im übrigen Mock (audio/atlas.ogg, probe-a/-b).
// Gewollte Lücken zeigen die Herkunft: `sfx:skill:mage` ohne Zuordnung und kein Slot für Tank/Dieb → Rückfall `sfx:skill`.

const atlas = (sprite: string): AssetChoice[] => [{ datei: 'audio/atlas.ogg', sprite }];
const file = (name: 'probe-a' | 'probe-b'): AssetChoice[] => [{ datei: `audio/${name}.ogg` }];

// Lücken liegen in eigenen Kategorien: die Szene „Ereignis-SFX“ (Kategorie sfx) bleibt lückenlos wie in den Tests der Seite.
const sfx = (id: string, beschreibung: string, wahl: AssetChoice[], kategorie = 'sfx'): AssetSlot => ({
  id, art: 'sound', kategorie, beschreibung, wahl, status: wahl.length ? 'zugeordnet' : 'luecke', bus: 'sfx',
});

export function soundSlots(): AssetSlot[] {
  return [
    sfx('sfx:hit', 'Treffer', atlas('hit')),
    sfx('sfx:kill', 'Tod eines Gegners', atlas('kill')),
    sfx('sfx:arrow', 'Pfeil', file('probe-b')),
    sfx('sfx:strike', 'Schlag', atlas('strike')),
    sfx('sfx:build', 'Gebäude fertig', file('probe-a')),
    sfx('sfx:buildProgress', 'Baufortschritt', atlas('build')),
    sfx('sfx:hub', 'Hub-Ausbau', file('probe-b')),
    sfx('sfx:coinGive', 'Münze geben', atlas('coin')),
    sfx('sfx:boss', 'Boss erscheint', file('probe-b')),
    sfx('sfx:warn', 'Warnung', file('probe-a')),
    sfx('sfx:ride', 'Hufschlag', file('probe-a')),
    sfx('sfx:uiSelect', 'UI: Auswahl', atlas('select')),
    sfx('sfx:uiConfirm', 'UI: Bestätigen', [], 'ui'),
    sfx('sfx:skill', 'Skill-Punkt, Rückfall für Skills', atlas('skill')),
    sfx('sfx:skill:healer', 'Skill: Heiler', file('probe-a')),
    sfx('sfx:skill:mage', 'Skill: Zauberer', [], 'skills'),
  ];
}
