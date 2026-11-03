# Aktive Task

**Status:** Erfuellt  
**Typ:** Fehlerkorrektur / Provider-Mapping

## Auftrag

OpenFIGI liefert einen ISIN-Ticker samt Boersencode, waehrend Marketstack fuer EOD das konkrete Symbol des Listings verlangt. Gleiche die OpenFIGI-Antwort mit Marketstacks `/tickers`-Katalog ab und verwende zuerst das EOD-faehige Listing der passenden Boerse. Liefert dieses fuer den Zeitraum keine Punkte, pruefe die weiteren EOD-faehigen Listings in deterministischer Reihenfolge. Bei unbekannter oder mehrdeutiger Zuordnung nicht raten, sondern den Grund ueber `APP_DEBUG` anzeigen.

## Abnahme

- `VUSA` mit OpenFIGI-Boersencode `LN` wird auf das vorhandene Marketstack-Listing `VUSA.XLON` mit `has_eod=true` aufgeloest.
- Listings ohne EOD-Unterstuetzung werden nicht an `/eod` gesendet.
- Wenn `VUSA.XLON` keine Punkte hat, liefert der Fallback fuer `IE00B3XXRP09` Daten aus `VUSA.XFRA`.
- Mehrdeutige, nicht verifizierte Boersenzuordnungen werden mit den Kandidaten diagnostiziert statt geraten.
- Ticker, Listingname und Marketstack-Boersencode erscheinen im Ergebnis und in der Debugdiagnose.
- Rust-Tests und `cargo fmt --check` laufen erfolgreich.