---
name: wrap-up
description: "Synchronisiert den dokumentierten Projektstand mit Repository und Checks und erstellt auf ausdruecklichen Aufruf einen Commit."
argument-hint: "Optionale Hinweise zum Abschluss oder Commit-Inhalt"
agent: "agent"
---

Fuehre einen vollstaendigen Projekt-Wrap-up fuer dieses Repository aus.

1. Lies zuerst [PAD](../../PAD.md), [STATUS](../../STATUS.md), [TASK](../../TASK.md) und [PIV](../../PIV.md) in dieser Reihenfolge. Befolge ausserdem [Copilot-Instructions](../copilot-instructions.md).
2. Inventarisiere mit `rg --files --hidden -g '*.md' -g '!.git/**' -g '!target/**'` alle Markdown-Dateien. Lies README, Lernplan, PRD, Architektur und alle Dateien, deren Inhalt durch den aktuellen Stand betroffen sein koennte. Behandle Agentenregeln und Prompts als Prozessvorgaben; aendere sie nur, wenn der Auftrag oder der tatsaechliche Stand das erfordert.
3. Pruefe `git status`, den vollstaendigen relevanten Diff, den aktuellen Code und die vorhandenen Testergebnisse. Behaupte keine Funktion, Verifikation, Anbieterbedingung oder offene Aufgabe ohne Beleg.
4. Halte die Rollen der Dokumente getrennt: PAD fuer stabile Architektur, PRD fuer Produktziele und -umfang, STATUS fuer den aktuellen Implementierungsstand und die letzte Verifikation, TASK fuer den letzten beziehungsweise aktiven Auftrag, PIV fuer den Prozess, LEARNING_PLAN fuer Lernziele, README fuer den oeffentlichen Projekteinstieg und `docs/ARCHITECTURE.md` fuer den tatsaechlichen Codepfad. Entferne oder kennzeichne veraltete Statusangaben, statt aktuelle und historische Teststaende zu vermischen.
5. Aktualisiere die betroffenen Markdown-Dateien knapp und konsistent. Markiere `TASK.md` erst dann als abgeschlossen, wenn Abnahmekriterien und Checks erfuellt sind. Bewahre Nutzer-Aenderungen; ueberschreibe keine unbekannten oder sachfremden Aenderungen.
6. Fuehre die fuer den tatsaechlichen Diff passenden Checks aus. Bei Codeaenderungen insbesondere die fokussierten Tests, `cargo test` und `cargo fmt --check`, sofern verfuegbar. Bei reinen Dokumentaenderungen mindestens `git diff --check` und eine Pruefung neu geaenderter lokalen Markdown-Links. Aktualisiere STATUS nur mit Ergebnissen, die du tatsaechlich gesehen hast.
7. Fuehre `git status` und `git diff --check` erneut aus. Stage nur die Dateien und Aenderungsteile, die zu diesem Wrap-up-Auftrag gehoeren. Wenn sachfremde oder nicht sicher trennbare Nutzer-Aenderungen im Commit landen wuerden, committe sie nicht; klaere die Grenze zuerst.
8. Erstelle nach erfolgreicher Verifikation einen Commit. Verwende einen kurzen Betreff, eine Leerzeile und darunter einen Body mit den wichtigsten Aenderungen und den ausgefuehrten Checks. Fuehre keinen Push aus und gib keine Geheimnisse aus.
9. Berichte auf Deutsch knapp ueber geaenderte Dokumente, Checks, Commit-Hash, Commit-Betreff und offene Punkte. Erfinde keinen Commit-Hash und behaupte keinen erfolgreichen Commit, wenn der Befehl fehlgeschlagen ist.
