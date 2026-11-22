import { useEffect, useState } from 'react';
import { Box, Button, Code, Flex, Grid, Heading, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { assetsStyle, type MaterialToene, type Stil } from '../api';
import { PromptButtons } from '../prompts/PromptButtons';
import { useVorlagen, type VorlagenJeOrt } from '../prompts/useVorlagen';

const TOENE = ['licht', 'basis', 'schatten'] as const;

/** Gewähltes Element der Stil-Datei: ein Material, ein Einzelton oder das Papier (B-727). */
type Wahl = { ort: 'material'; name: string } | { ort: 'ton'; name: string } | { ort: 'papier' };
type Waehle = (w: Wahl) => void;
const gleich = (a: Wahl | null, b: Wahl) => a?.ort === b.ort && (a.ort === 'papier' || (b.ort !== 'papier' && a.name === b.name));

/** Seite Styleguide im Bereich Assets (WB1.2): Materialtafel aus der Stil-Datei, Einzeltöne und Papier. Klick auf ein
 *  Material, einen Ton oder das Papier zeigt Prompt-Buttons (B-727); geändert wird über `assets_style_set`
 *  (Entscheidung 005 › 2, 4). */
export function StylePage() {
  const [stil, setStil] = useState<Stil | null>(null);
  const [error, setError] = useState('');
  const [wahl, setWahl] = useState<Wahl | null>(null);
  const vorlagen = useVorlagen();
  useEffect(() => {
    let live = true;
    assetsStyle().then((s) => live && setStil(s), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  if (error) return <Text color="red" m="4">{error}</Text>;
  if (!stil) return <Text color="gray" m="4">Lädt …</Text>;
  const an = (w: Wahl) => gleich(wahl, w);
  return (
    <Flex direction="column" gap="4" p="4">
      {vorlagen.error && <Text color="red">{vorlagen.error}</Text>}
      {wahl ? <Auswahl wahl={wahl} jeOrt={vorlagen.jeOrt} /> : <Text size="1" color="gray">Klick auf ein Material, einen Ton oder das Papier zeigt Prompts</Text>}
      <Heading size="4">Materialien</Heading>
      <Grid columns="repeat(auto-fill, minmax(220px, 1fr))" gap="3">
        {Object.entries(stil.materialien).map(([name, t]) => (
          <Material key={name} name={name} toene={t} an={an({ ort: 'material', name })} waehle={setWahl} />
        ))}
      </Grid>
      {stil.einzeltoene && (
        <>
          <Heading size="4">Einzeltöne</Heading>
          <Grid columns="repeat(auto-fill, minmax(160px, 1fr))" gap="2">
            {Object.entries(stil.einzeltoene).map(([name, hex]) => (
              <Swatch key={name} label={name} hex={hex} wahl={{ ort: 'ton', name }} an={an({ ort: 'ton', name })} waehle={setWahl} />
            ))}
          </Grid>
        </>
      )}
      {stil.papier && (
        <>
          <Flex gap="2" align="center">
            <Heading size="4">Papier</Heading>
            <Wahlknopf label="Prompts" wahl={{ ort: 'papier' }} an={an({ ort: 'papier' })} waehle={setWahl} />
          </Flex>
          <Code size="1" style={{ whiteSpace: 'pre-wrap' }}>{JSON.stringify(stil.papier, null, 2)}</Code>
        </>
      )}
    </Flex>
  );
}

function Auswahl({ wahl, jeOrt }: { wahl: Wahl; jeOrt: VorlagenJeOrt }) {
  const caption = wahl.ort === 'papier' ? 'Papier' : `${wahl.ort === 'material' ? 'Material' : 'Ton'}: ${wahl.name}`;
  const werte = wahl.ort === 'material' ? { material: wahl.name } : wahl.ort === 'ton' ? { ton: wahl.name } : {};
  return (
    <Box data-auswahl={wahl.ort}>
      <PromptButtons key={caption} caption={caption} vorlagen={jeOrt(wahl.ort)} werte={werte} leer={`Keine Prompt-Vorlagen für ${caption}`} />
    </Box>
  );
}

function Wahlknopf({ label, wahl, an, waehle }: { label: string; wahl: Wahl; an: boolean; waehle: Waehle }) {
  return (
    <Button size="1" variant={an ? 'solid' : 'ghost'} aria-pressed={an} data-wahl={wahl.ort} onClick={() => waehle(wahl)}>
      {label}
    </Button>
  );
}

function Material({ name, toene, an, waehle }: { name: string; toene: MaterialToene; an: boolean; waehle: Waehle }) {
  return (
    <Flex direction="column" gap="1" data-material={name} className="as-material">
      <Flex><Wahlknopf label={name} wahl={{ ort: 'material', name }} an={an} waehle={waehle} /></Flex>
      {TOENE.map((ton) => <Swatch key={ton} label={ton} hex={toene[ton]} ton={ton} />)}
    </Flex>
  );
}

interface SwatchProps {
  label: string;
  hex: string;
  ton?: string;
  /** Einzeltöne sind wählbar (B-727), die Töne eines Materials nicht. */
  wahl?: Wahl;
  an?: boolean;
  waehle?: Waehle;
}

function Swatch({ label, hex, ton, wahl, an = false, waehle }: SwatchProps) {
  return (
    <Flex gap="2" align="center" data-ton={ton} data-einzelton={wahl ? label : undefined}>
      <Box data-swatch className="as-swatch" style={{ background: hex }} />
      {wahl && waehle ? <Wahlknopf label={label} wahl={wahl} an={an} waehle={waehle} /> : <Text size="1">{label}</Text>}
      <Code size="1">{hex}</Code>
    </Flex>
  );
}
