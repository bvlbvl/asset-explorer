# Projektstatus

**Stand:** 2026-11-03  
**Phase:** Browser-MVP mit taeglichen Schlusskursen

**Dokumente:** [PAD](PAD.md) · [PRD](PRD.md) · [Aktive Task](TASK.md) · [PIV-Prozess](PIV.md) · [Agenten-Einstieg](.github/copilot-instructions.md)

## Aktueller Stand

- Rust-Backend mit Axum und statischem Browser-Client ist vorhanden.
- `GET /api/history?isin=...` liefert taegliche Schlusskurse des letzten abgeschlossenen Kalendermonats.
- Der Browser zeigt Instrument, Zeitraum, letzten Schlusskurs, Datenanzahl und Chart.
- Eine unbekannte, aber syntaktisch gueltige ISIN wird mit einer klaren Meldung abgelehnt.
- Marketstack wird ueber einen Backend-Adapter angesprochen; `.env` bleibt lokal und ist von Git ausgeschlossen.
- Die Architektur-Dokumentation ist in `docs/ARCHITECTURE.md` ergänzt und mit dem tatsächlichen Code-, API- und Teststand abgeglichen.

## Gepruefte ISIN-Zuordnungen

- `IE00B0M62Q58` -> `IQQW.XETRA` (Xetra, EUR)
- `IE00BKM4GZ66` -> `EIMI.XSWX` (SIX Swiss Exchange, Waehrung nicht im internen Modell gesetzt)

Dies sind explizite Testzuordnungen und kein allgemeiner ISIN-Suchdienst. Vor jeder neuen Zuordnung Symbol, Boerse und Datenabdeckung pruefen.

## Bekannte Grenzen

- Marketstack loest die ISIN nicht direkt auf; weitere ISINs benoetigen eine gepruefte Symbolzuordnung.
- Intraday ist fuer die bisher getesteten Instrumente im aktuellen Tarif nicht verfuegbar. Der MVP verwendet EOD-Schlusskurse.
- EOD-Datenverfuegbarkeit kann je Boersenlisting abweichen.

## Letzte Verifikation

- `cargo test`: 4 Tests bestanden; Tests verwenden Fake-Daten und kein Netzwerk.
- `cargo fmt --check`: bestanden.
- Dokumentationscheck: `docs/ARCHITECTURE.md` bezieht sich auf die realen Module `src/main.rs`, `src/api.rs`, `src/service.rs`, `src/domain.rs`, `src/marketstack.rs`, `public/app.js` und `public/index.html`.
- Manueller Marketstack-Smoke-Test fuer `IE00B0M62Q58` / `IQQW.XETRA`: 22 Tageswerte fuer Oktober 2026; Intraday wurde mit Tarifbeschraenkung abgelehnt.
- Browser-Smoke-Test: erfolgreiche Chartdarstellung und klare Meldung fuer eine nicht zugeordnete ISIN.

## Naechster Auftrag

Siehe [TASK.md](TASK.md). Nach Abschluss sollen dort Status und Abnahmekriterien aktualisiert werden. Details der geplanten Architektur-Dokumentation gehoeren in `docs/ARCHITECTURE.md`, nicht hier.
