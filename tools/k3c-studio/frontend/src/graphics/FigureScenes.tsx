import { Text } from '@webkit/atoms';
import { renderReference, type Card } from '@game/tools/spriteReference';
import { useEffect, useRef } from 'react';
import { biomOf, bossKarten, gegnerKarten, reittierKarten, type GameSceneId } from './gameScenesLogic';

// Szenen „Gegner je Biom“, „Reittiere“ und „Bosse“ (B-598): Figuren über `renderReference` des Spiels (wie die Szene Figuren);
// die Karten kommen aus data/sprites.json, data/biomes und data/bosses.json.

function Karten({ title, cards }: { title: string; cards: Card[] }) {
  const ref = useRef<HTMLDivElement>(null);
  const key = JSON.stringify(cards);
  useEffect(() => {
    const root = ref.current;
    if (!root) return;
    root.replaceChildren();
    return cards.length > 0 ? renderReference(root, [{ title, cards }], true) : undefined;
    // cards ändern sich mit jedem Rendern; maßgeblich ist key
  }, [key, title]);
  return <div ref={ref} className="sz-figuren" />;
}

export function FigureScene({ szene, biom }: { szene: Extract<GameSceneId, 'gegner' | 'reittiere' | 'bosse'>; biom: string }) {
  if (szene === 'gegner') {
    const cards = gegnerKarten(biom);
    return (
      <>
        <Text size="2" color="gray">Gegner von {biomOf(biom).name}: Portal- und Nacht-Gegner aus data/biomes.</Text>
        {cards.length > 0 ? <Karten title={`Gegner · ${biom}`} cards={cards} /> : <div className="sz-gap">Lücke: keine Gegner mit Sprite in {biom}</div>}
      </>
    );
  }
  if (szene === 'reittiere') return <Karten title="Reittiere" cards={reittierKarten()} />;
  return (
    <>
      <Text size="2" color="gray">Bosse mit Phasen; es gibt keine eigenen Boss-Sprites, die Karte zeigt den Standardgegner (1,5-fach).</Text>
      <Karten title="Bosse" cards={bossKarten()} />
    </>
  );
}
