# Projektstatus

**Stand:** 2026-11-03  
**Phase:** Browser-MVP mit taeglichen Schlusskursen

**Dokumente:** [PAD](PAD.md) · [PRD](PRD.md) · [Aktive Task](TASK.md) · [PIV-Prozess](PIV.md) · [Agenten-Einstieg](.github/copilot-instructions.md)

## Aktueller Stand

- Rust-Backend mit Axum und statischem Browser-Client ist vorhanden.
- `GET /api/history?isin=...` liefert taegliche Schlusskurse des letzten abgeschlossenen Kalendermonats.
- Der Browser zeigt Instrument, Zeitraum, letzten Schlusskurs, Datenanzahl und Chart.
- Die OpenFIGI-Aufloesung behaelt Ticker, Wertpapiername und ExchCode und gleicht Kandidaten mit Marketstacks `/tickers`-Katalog ab. Der Service prueft EOD-faehige Listings in Reihenfolge, bis ein Listing Kurspunkte liefert. Bei leerem Ergebnis oder Anbieterfehler koennen API und Browser mit `APP_DEBUG` getrennte Diagnosezeilen ausgeben.
- Eine unbekannte, aber syntaktisch gueltige ISIN wird zunaechst ueber OpenFIGI aufgeloest; fehlendes Mapping und fehlende Marketstack-Daten werden unterscheidbar gemeldet.
- Marketstack wird ueber einen Backend-Adapter angesprochen; `.env` bleibt lokal und ist von Git ausgeschlossen.
- Die Architektur-Dokumentation ist in `docs/ARCHITECTURE.md` ergänzt und mit dem tatsächlichen Code-, API- und Teststand abgeglichen.
- Die Projektgrundlagen in [PAD](PAD.md) und [PIV.md](PIV.md) wurden erweitert: neue Features muessen mit einem passenden lokalen Test beginnen und die Regression lokal gruen laufen, bevor sie als fertig gelten.

## Gepruefte ISIN-Zuordnungen

- `IE00B0M62Q58` -> `IQQW.XETRA` (Xetra, EUR)
- `IE00BKM4GZ66` -> `EIMI.XSWX` (SIX Swiss Exchange, Waehrung nicht im internen Modell gesetzt)

Dies sind explizite Testzuordnungen und kein allgemeiner ISIN-Suchdienst. Vor jeder neuen Zuordnung Symbol, Boerse und Datenabdeckung pruefen.

## Bekannte Grenzen

- Marketstack loest die ISIN nicht direkt auf. Der Katalog bestaetigt `VUSA.XLON` (MIC `XLON`) mit `has_eod=true`, aber fuer Oktober 2026 lieferte es keine Punkte; `VUSA.XFRA` lieferte 22. Der Service faellt bei leeren Kursdaten auf weitere EOD-faehige Listings zurueck. Mehrdeutige OpenFIGI-Boersencodes ohne verifizierte Zuordnung werden nicht geraten. `APP_DEBUG=true` zeigt jeden Versuch.
- Intraday ist fuer die bisher getesteten Instrumente im aktuellen Tarif nicht verfuegbar. Der MVP verwendet EOD-Schlusskurse.
- EOD-Datenverfuegbarkeit kann je Boersenlisting abweichen.

## Letzte Verifikation

- `cargo test`: 5 Tests bestanden; Tests verwenden Fake-Daten und kein Netzwerk.
- `cargo fmt --check`: bestanden.
- Diagnoseaenderung: `cargo test` bestanden (12 Tests, 0 Fehler); `cargo fmt --check` bestanden. Der `/tickers`-Regressionstest waehlt `VUSA.XLON`; der Service-Fallbacktest wechselt bei leeren Daten zu einem EOD-Listing mit Punkten.
- Live-Smoke-Test fuer `IE00B3XXRP09`: `VUSA.XLON` lieferte fuer Oktober 2026 keine Punkte; automatischer Fallback auf `VUSA.XFRA` lieferte 22 Tageswerte ueber `GET /api/history`.
- Dokumentationscheck: `docs/ARCHITECTURE.md` bezieht sich auf die realen Module `src/main.rs`, `src/api.rs`, `src/service.rs`, `src/domain.rs`, `src/marketstack.rs`, `public/app.js` und `public/index.html`.
- Manueller Marketstack-Smoke-Test fuer `IE00B0M62Q58` / `IQQW.XETRA`: 22 Tageswerte fuer Oktober 2026; Intraday wurde mit Tarifbeschraenkung abgelehnt.
- Browser-Smoke-Test: erfolgreiche Chartdarstellung, lokale HUD-Auswahl auf der X-Achse und korrekte Prozentrechnung seit dem ersten Chart-Punkt.

## Naechster Auftrag

Siehe [TASK.md](TASK.md). Nach Abschluss sollen dort Status und Abnahmekriterien aktualisiert werden. Details der geplanten Architektur-Dokumentation gehoeren in `docs/ARCHITECTURE.md`, nicht hier.
