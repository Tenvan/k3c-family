import { Flex, Spinner, Text } from '../atoms';

interface Props {
  /** Kennung für Tests und Seiten (`data-loading`), z. B. `list`, `frame`. */
  id: string;
  text: string;
  /** Über einer Grafik: nur der Spinner, absolut in der Ecke des nächsten positionierten Vorfahren; verschiebt nichts. */
  overlay?: boolean;
}

/** Lade-Anzeige (WB2): als Zeile mit Spinner und Text oder als Überlage über einer Grafik (Text dann nur für Screenreader). */
export function LoadingNote({ id, text, overlay = false }: Props) {
  if (overlay) {
    return (
      <div className="wk-loading-overlay" data-loading={id} role="status" aria-label={text}>
        <Spinner size="2" />
      </div>
    );
  }
  return (
    <Flex data-loading={id} role="status" gap="2" align="center">
      <Spinner size="1" />
      <Text size="1" color="gray">{text}</Text>
    </Flex>
  );
}
