# Aktive Task

**Status:** Bereit fuer den naechsten Agentenlauf  
**Typ:** Dokumentation

**Einstieg:** Lies zuerst [Agentenregeln](.github/copilot-instructions.md), dann [PAD](PAD.md), [STATUS](STATUS.md), [PIV-Prozess](PIV.md) und die fuer diese Aufgabe relevanten Teile des [PRD](PRD.md). Lies anschliessend nur den Code, der fuer die Architektur-Erklaerung erforderlich ist.

## Auftrag

Erstelle `docs/ARCHITECTURE.md` als deutschsprachige Architekturdokumentation fuer das bestehende Aktienkurs-Explorer-Projekt. Die Datei soll ein Mermaid-Diagramm der Architektur enthalten und den Ablauf vom ISIN-Formular bis zur Marketstack-Antwort und zum gerenderten Chart nachvollziehbar erklaeren.

## Inhalt

- Systemkontext und Mermaid-Komponentendiagramm.
- Request-Ablauf fuer `GET /api/history?isin=...`.
- Verantwortung der Rust-Module und des statischen Browser-Clients.
- ISIN-Validierung, bekannte Symbolzuordnungen und Verhalten bei unbekannten Kennungen.
- Berechnung des letzten abgeschlossenen Kalendermonats und Abbildung der EOD-Antwort auf interne Typen.
- Fehlerbehandlung, API-Key-Grenze und Offline-Tests.
- Verifizierte Grenzen, insbesondere Marketstack-Symbol-/Boersenabdeckung und Intraday-Verfuegbarkeit.

## Grenzen und Abnahme

- Dokumentiere den tatsaechlichen Code- und Teststand; erfinde keine Funktionen oder Anbieterzusagen.
- Aendere nur Dokumentation. Wenn du einen Code-/PRD-Widerspruch findest, dokumentiere ihn und melde ihn, statt ungefragt Produktcode umzubauen.
- Diagramm und Erklaerungen muessen zusammenpassen und auf existierende Module/Routes verweisen.
- Fuehre passende Dokumentations- oder Projektchecks aus und aktualisiere danach den [STATUS](STATUS.md).

Diese Datei enthaelt genau einen aktiven Auftrag. Der Nutzer ersetzt ihren Inhalt fuer jeden neuen Agentenlauf; allgemeine Regeln gehoeren nicht hierher.
