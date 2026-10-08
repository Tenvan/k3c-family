import { installPageChrome } from '../core/shell';
import { installSelection } from './selection';
import { applyTexts, t } from './texts';
import { installPadScroll, lineup, renderReference } from './spriteReference';

installPageChrome();
applyTexts();
installPadScroll();
renderReference(document.getElementById('cards')!, lineup(), true, installSelection('k3c-auswahl-aufstellung', t('sel.aufstellung')));
