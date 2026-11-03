# PIV-Arbeitsprozess

**PIV bedeutet Plan – Implement – Verify.** Dies ist der Ablauf fuer die Ausfuehrung einer aktiven [Task](TASK.md), kein zweites PRD und kein Statusbericht.

**Kontext:** [PAD](PAD.md) · [PRD](PRD.md) · [Status](STATUS.md) · [Aktive Task](TASK.md) · [Agentenregeln](.github/copilot-instructions.md)

## Plan

1. Lies den Agenten-Einstieg, das PAD, den Status und die aktive Task; lies PRD- und Codeabschnitte nur soweit fuer diese Task relevant.
2. Identifiziere den konkreten Code-/Dokumentationsanker, das erwartete Verhalten und eine lokale Hypothese.
3. Lege vor der Aenderung einen moeglichst guenstigen Check fest, der die Hypothese widerlegen kann.
4. Wenn eine neue Funktion oder ein neues Verhalten implementiert wird, schreibe vorher den passenden lokalen Test und verifiziere, dass er den Fehler oder das erwartete Verhalten reproduziert.
5. Frage den Nutzer nur bei einer wesentlichen, nicht aus Repository und Anforderungen ableitbaren Entscheidung. Halte andere Unsicherheiten sichtbar, ohne den Kontext mit Wiederholungen zu fuellen.

## Implement

1. Aendere den kleinsten zusammenhaengenden Bereich, der die Task erfuellt.
2. Bewahre vorhandene Nutzer-Aenderungen und oeffentliche Schnittstellen, ausser die Task verlangt ausdruecklich eine Aenderung.
3. Halte neue Rust-Konzepte knapp und lernorientiert erklaerbar.
4. Aktualisiere dauerhafte Anforderungen im PRD, aktuelle Fortschritte im STATUS und nur den jeweiligen Auftrag in TASK. Dupliziere keine Inhalte zwischen diesen Dateien.

## Verify

1. Fuehre direkt nach der ersten Codeaenderung den geplanten fokussierten Check aus.
2. Wenn der Check fehlschlaegt, lese die lokale Ausgabe genau; sie muss die betroffene Stelle, die Ursache und den naechsten Korrekturpunkt benennen, damit keine Kontextverschmutzung durch Raten entsteht.
3. Korrigiere denselben Bereich und wiederhole denselben Test, bis er lokal gruen ist.
4. Fuehre danach nur weitere Checks aus, die fuer das Risiko der Aenderung oder die Abnahmekriterien noetig sind.
5. Pruefe Dokumentationslinks, Akzeptanzkriterien und den tatsaechlichen Codebezug, wenn die Task Dokumentation betrifft.
6. Aktualisiere STATUS mit ausgefuehrten Checks, Ergebnissen und verbleibenden Grenzen. Behaupte keine nicht ausgefuehrten Pruefungen.

## Abschluss

Melde knapp, was geaendert wurde, welche Checks liefen und was offen bleibt. Wiederhole nicht den vollstaendigen Projektkontext oder die gesamte Task.
