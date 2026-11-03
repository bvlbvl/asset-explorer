---
name: run-task
description: "Fuehre die aktive Projektaufgabe aus TASK.md nach den Repository-Regeln und dem PIV-Prozess aus."
argument-hint: "Optionale ausdrueckliche Task fuer diesen Lauf"
agent: "agent"
---

Fuehre die aktive Aufgabe in [TASK.md](../../TASK.md) vollstaendig aus.

1. Lies zuerst die Kontextdateien in der vorgeschriebenen Reihenfolge: [PAD](../../PAD.md), [STATUS](../../STATUS.md), [TASK](../../TASK.md), [PIV](../../PIV.md). Befolge zusaetzlich die automatisch geltenden Regeln aus [Copilot-Instructions](../copilot-instructions.md).
2. Eine ausdrueckliche Task, die der Nutzer zusammen mit diesem Kommando angibt, ersetzt fuer diesen Lauf den Auftrag in `TASK.md`; sonst ist `TASK.md` massgeblich.
3. Ermittle vor der Aenderung den konkreten Codeanker, eine falsifizierbare lokale Hypothese und den guenstigsten passenden Check. Bei neuem Verhalten zuerst einen Regressionstest schreiben und ausfuehren.
4. Implementiere die kleinste passende Aenderung. Bewahre nicht zugehoerige Nutzer-Aenderungen; fuehre keine Commits, Branches oder destruktiven Git-Aktionen aus.
5. Fuehre direkt nach jeder wesentlichen Aenderung den fokussierten Check aus. Danach nur die fuer die Aufgabe benoetigten Regressionen und Projektchecks ausfuehren. Behaupte keine Verifikation, deren Ausgabe oder Ergebnis nicht vorliegt.
6. Aktualisiere nach Abschluss `STATUS.md` mit Ergebnis, ausgefuehrten Checks und offenen Grenzen. Halte `TASK.md` auf dem tatsaechlichen Aufgabenstatus. Dauerhafte Architektur- oder Produktentscheidungen gehoeren in `PAD.md` beziehungsweise `PRD.md`.
7. Beende den Lauf erst, wenn die Abnahmekriterien erfuellt sind oder ein konkreter Blocker mit Belegen benannt ist. Berichte knapp auf Deutsch: Aenderung, Checks, Ergebnis und offene Punkte.

Falls der Nutzer nach diesem Kommando eine ausdrueckliche Task mitgibt, behandle sie als die Task dieses Laufs und aktualisiere `TASK.md` entsprechend.