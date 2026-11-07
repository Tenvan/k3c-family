import { useEffect, useState } from 'react';
import { Box, Code, Flex, Grid, Heading, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { assetsStyle, type MaterialToene, type Stil } from '../api';

const TOENE = ['licht', 'basis', 'schatten'] as const;

/** Seite Styleguide im Bereich Assets (WB1.2): Materialtafel aus der Stil-Datei, Einzeltöne und Papier. Nur Anzeige;
 *  geändert wird über `assets_style_set` (Entscheidung 005 › 2, 4). */
export function StylePage() {
  const [stil, setStil] = useState<Stil | null>(null);
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    assetsStyle().then((s) => live && setStil(s), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  if (error) return <Text color="red" m="4">{error}</Text>;
  if (!stil) return <Text color="gray" m="4">Lädt …</Text>;
  return (
    <Flex direction="column" gap="4" p="4">
      <Heading size="4">Materialien</Heading>
      <Grid columns="repeat(auto-fill, minmax(220px, 1fr))" gap="3">
        {Object.entries(stil.materialien).map(([name, t]) => <Material key={name} name={name} toene={t} />)}
      </Grid>
      {stil.einzeltoene && (
        <>
          <Heading size="4">Einzeltöne</Heading>
          <Grid columns="repeat(auto-fill, minmax(160px, 1fr))" gap="2">
            {Object.entries(stil.einzeltoene).map(([name, hex]) => <Swatch key={name} label={name} hex={hex} />)}
          </Grid>
        </>
      )}
      {stil.papier && (
        <>
          <Heading size="4">Papier</Heading>
          <Code size="1" style={{ whiteSpace: 'pre-wrap' }}>{JSON.stringify(stil.papier, null, 2)}</Code>
        </>
      )}
    </Flex>
  );
}

function Material({ name, toene }: { name: string; toene: MaterialToene }) {
  return (
    <Flex direction="column" gap="1" data-material={name} className="as-material">
      <Text size="2" weight="bold">{name}</Text>
      {TOENE.map((ton) => <Swatch key={ton} label={ton} hex={toene[ton]} ton={ton} />)}
    </Flex>
  );
}

function Swatch({ label, hex, ton }: { label: string; hex: string; ton?: string }) {
  return (
    <Flex gap="2" align="center" data-ton={ton}>
      <Box data-swatch className="as-swatch" style={{ background: hex }} />
      <Text size="1">{label}</Text>
      <Code size="1">{hex}</Code>
    </Flex>
  );
}
