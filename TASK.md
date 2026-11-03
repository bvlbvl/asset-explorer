# Aktive Task

**Status:** Bereit fuer den naechsten Agentenlauf  
**Typ:** Implementierung + Verifikation

**Einstieg:** Lies zuerst [Agentenregeln](.github/copilot-instructions.md), dann [PAD](PAD.md), [STATUS](STATUS.md), [PIV-Prozess](PIV.md) und die fuer diese Aufgabe relevanten Teile des [PRD](PRD.md). Lies anschliessend nur die Codebereiche, die fuer die zu aendernde Funktion oder den zu schreibenden Test erforderlich sind.

## Auftrag

Erweitere die Projekt-Dokumentation so, dass jede neue Funktion oder jeder neue Feature-Block immer mit einer Test- und Validierungsstruktur beginnt und diese durchgehend lokal verifiziert wird. Im Kern geht es darum, dass Agenten nicht nur Code aendern, sondern vor Abschluss immer die passenden Tests schreiben und diese regressiv laufen lassen. Die Ergebnisse der Tests muessen klar und lokal lesbar sein und dem Agenten direkt sagen, was zu aendern ist, falls ein Check fehlschlaegt.

## Ziel

- Neue Features sollen nicht ohne lokale Regressionstests eingefuehrt werden.
- Test- und Validierungsanforderungen muessen in der Projekt-Doku verbindlich beschrieben werden.
- Der Agent darf die Testausgabe nicht als unlesbaren Kontextpuffer behandeln; sie muss den konkreten Fehler und den naechsten Aenderungspunkt benennen.
- Das Projekt soll so dokumentiert sein, dass bei einer Fehlermeldung der Agent genau weiss, wo und warum zu aendern ist, ohne die gesamte Umgebung rekonstruieren zu muessen.

## Anforderungen an jeden Agentenlauf

1. Vor einer Aenderung an einer Funktion oder einem Feature muss ein passender lokaler Test erstellt oder erweitert werden.
2. Der Test muss den bisher existierenden Fehler oder das neue Verhalten reproduzierbar abdecken.
3. Der Agent muss den Testlauf lokal ausfuehren, bevor er als fertig betrachtet wird.
4. Wenn ein Test fehlschlaegt, muss der Agent die Ursache anhand der lokalen Ausgabe identifizieren und genau im betroffenen Bereich korrigieren.
5. Nach der Korrektur muss derselbe Test erneut lokal ausgefuehrt werden und erfolgreich durchlaufen.
6. Wenn mehrere relevante Tests betroffen sind, sollen die passenden, fokussierten Checks ausgefuehrt werden, damit Regressionen lokal erkannt werden.
7. Die Testausgabe muss klar genug sein, dass aus ihr direkt erkannt werden kann, was zu aendern ist.
8. Die Dokumentation darf keine generischen Aussagen mehr enthalten, sondern muss die Test- und Verifikationspflicht fuer jede implementierte Aenderung explizit nennen.

## Dokumentationspflicht

Ergaenze bzw. aktualisiere die passende Dokumentation, damit sie diese Regeln festlegt:

- Jede neue Funktion oder jeder neue Feature-Block braucht zuerst einen passenden lokalen Test.
- Tests werden regressiv im Projekt ausgefuehrt und sind Teil der Definition von Done.
- Fehlerausgaben muessen die relevante Stelle, den Fehler und die naechste Aenderung beschreiben, damit der Agent den Kontext nicht mit Raten und Wiederholungen vernebeln muss.
- Es ist nicht ausreichend, "irgendwo" einen Test zu hinterlassen; er muss den relevanten Pfad und das verifizierte Verhalten abdecken.

## Abnahme

- Die Dokumentation beschreibt die Test- und Verifikationspflicht eindeutig und verbindlich.
- Es gibt einen klaren, lokal lauffaehigen Ablauf, wie neue Features mit Tests und Regressionen abgesichert werden.
- Die Doku zeigt, dass die Agenten- und Entwickler-Workflow auf lokale, nachvollziehbare Verifikation setzt und nicht auf manuelle, kontextlose KI-Ergebnisse.
- Nach der Umsetzung werden die relevanten lokalen Checks ausgefuehrt und das Ergebnis im [STATUS](STATUS.md) dokumentiert.

Diese Datei enthaelt genau einen aktiven Auftrag. Der Nutzer ersetzt ihren Inhalt fuer jeden neuen Agentenlauf; allgemeine Regeln gehoeren nicht hierher.
