import { Button, Flex, Kbd, Text } from '../atoms';

export interface TransportBarProps {
  /** Was gerade spielt; leer = Stille. */
  playing: string;
  /** Zusätzliche Meldung (z. B. „Datei fehlt“). */
  status?: string;
  /** Lautstärke in Prozent. */
  volume: number;
  onStop: () => void;
  onFade: () => void;
  onVolume: (delta: -1 | 1) => void;
}

/** Abspielleiste (B-677): ein Stopp, Überblenden und Lautstärke für die ganze Seite; die Tasten (S, X, +, −) hängt die Seite an. */
export function TransportBar({ playing, status, volume, onStop, onFade, onVolume }: TransportBarProps) {
  return (
    <Flex gap="2" align="center" wrap="wrap" className="wk-transport" role="toolbar" aria-label="Abspielleiste">
      <Button size="1" variant="soft" color="gray" onClick={onStop}>Stopp <Kbd>S</Kbd></Button>
      <Button size="1" variant="soft" onClick={onFade}>Überblenden <Kbd>X</Kbd></Button>
      <Button size="1" variant="soft" onClick={() => onVolume(-1)} aria-label="Leiser">−</Button>
      <Button size="1" variant="soft" onClick={() => onVolume(1)} aria-label="Lauter">+</Button>
      <Text size="2" role="status">{playing ? `spielt: ${playing}` : 'Stille'} · Lautstärke {volume} %</Text>
      {status && <Text size="1" color="gray">{status}</Text>}
    </Flex>
  );
}
