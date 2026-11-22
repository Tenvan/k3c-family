import { Button, Flex, Text } from '@webkit/atoms';

interface Props {
  /** Gewähltes Teil (Klick in die Vorschau), leer = keins. */
  teil: string;
  ohne: readonly string[];
  onOhne: (ohne: string[]) => void;
}

/** Teil aus- und einblenden (B-734): das gewählte Teil per Knopf, ausgeblendete Teile einzeln zurück (im Bild sind sie
 *  nicht mehr anklickbar). Gilt bis zum Asset-Wechsel, auch beim Abspielen. */
export function TeilSicht({ teil, ohne, onOhne }: Props) {
  const aus = ohne.includes(teil);
  const umschalten = (t: string) => onOhne(ohne.includes(t) ? ohne.filter((o) => o !== t) : [...ohne, t]);
  if (!teil && !ohne.length) return null;
  return (
    <Flex gap="2" align="center" wrap="wrap" data-teil-sicht>
      {teil && (
        <Button size="1" variant="soft" onClick={() => umschalten(teil)}>
          {aus ? `${teil} einblenden` : `${teil} ausblenden`}
        </Button>
      )}
      {ohne.some((t) => t !== teil) && <Text size="1" color="gray">Ausgeblendet:</Text>}
      {ohne.filter((t) => t !== teil).map((t) => (
        <Button key={t} size="1" variant="outline" onClick={() => umschalten(t)}>
          {t} einblenden
        </Button>
      ))}
      {ohne.length > 1 && <Button size="1" variant="ghost" onClick={() => onOhne([])}>alle einblenden</Button>}
    </Flex>
  );
}
