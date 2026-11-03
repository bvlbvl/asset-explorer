# Agent Entry Point

## Kontextdateien

Zu Beginn einer neuen Task lies diese kurzen Dateien in der Reihenfolge: [PAD](../PAD.md), [aktueller Status](../STATUS.md), [aktive Task](../TASK.md), [PIV-Prozess](../PIV.md). Lies danach nur die fuer die Aufgabe benoetigten Abschnitte aus [PRD](../PRD.md), [Lernplan](../LEARNING_PLAN.md) und Quellcode.

Diese Dateien haben getrennte Rollen: PAD = stabile Projekt- und Architekturinformationen; STATUS = veraenderlicher Fortschritt; TASK = genau ein aktiver Auftrag; PIV = Arbeitsablauf; PRD = Produktanforderungen. Dupliziere ihren Inhalt nicht.

Bearbeite die aktive Task, sofern die aktuelle ausdrueckliche Nutzeranweisung sie nicht ersetzt. Halte langfristige Entscheidungen und nach Abschluss verifizierten Status in der passenden Datei fest, nicht nur im Chat.

## Verhalten

- Folge bei Umsetzung dem Prozess in [PIV.md](../PIV.md). Arbeite klein, erklaerbar und risikogerecht; erklaere neue Rust-Konzepte knapp fuer einen Anfaenger.
- Frage nur bei wesentlichen, nicht aus Repository und Anforderungen ableitbaren Entscheidungen. Sonstige Unsicherheiten bleiben ausdruecklich offen.
- Halte Fortschrittsmeldungen und Abschluesse knapp. Wiederhole keine bereits bekannten Plaene oder langen Dokumentinhalte; erkunde nur den relevanten Codepfad.
- Bewahre Nutzer-Aenderungen. Fuehre keine Commits, Branches oder destruktiven Git-Aktionen ohne ausdruecklichen Auftrag aus.
- Verwende Recherche-Subagents nur, wenn der Nutzer darum bittet oder sie fuer eine belegte Anbieterrecherche unverzichtbar sind; frage vorher.

## Grenzen

- Befolge die Projektarchitektur, Anbieter- und Testvorgaben aus [PAD](../PAD.md) und die Produktanforderungen aus [PRD](../PRD.md); dupliziere sie nicht hier.
- Gib niemals Geheimnisse in Chat, Quellcode, Browserdateien, Logs oder Git aus.
- Nutze Subagents nur gemaess den obigen Regeln und fuehre keine nicht angeforderten Commits oder destruktiven Aktionen aus.