# Product Requirements Document: Aktienkurs-Explorer

**Status:** Entwurf  
**Stand:** 2026-11-03

**Projektkontext:** [PAD](PAD.md) · [Status](STATUS.md) · [Aktuelle Task](TASK.md) · [PIV-Prozess](PIV.md)

## 1. Produktueberblick

Der Aktienkurs-Explorer ist ein Rust-Lernprojekt. Nutzer geben im Browser eine ISIN ein und sehen fuer den letzten abgeschlossenen Kalendermonat die verfuegbaren taeglichen Schlusskurse als Chart. Das Backend stellt die Kursdaten ueber eine eigene JSON-API bereit; der Browser ruft Marketstack niemals direkt auf.

Marketstack ist der vorgesehene Marktdatenanbieter. Sein API-Schluessel wird lokal beziehungsweise serverseitig als `MARKETSTACK_API_KEY` bereitgestellt. Da Marketstack die getesteten ISINs nicht direkt in Boersensymbole aufloest, verwendet der MVP eine explizite, gepflegte ISIN-zu-Symbol-Zuordnung. Nicht zugeordnete Kennungen werden klar abgelehnt; Symbole oder Boersenplaetze duerfen nicht erraten werden.

Das Lernziel bleibt zentral: Rust-Konzepte, Tests, API-Design und eine schrittweise Client-Server-Architektur sollen anhand des Produkts nachvollziehbar eingeuebt werden.

## 2. Ziele und Nicht-Ziele

- Eine ISIN in einer responsiven Browser-Oberflaeche eingeben und absenden koennen.
- Fuer den letzten abgeschlossenen Kalendermonat die von Marketstack gelieferten taeglichen Schlusskurse als einfachen Chart anzeigen.
- Instrumentname, ISIN, zugeordnetes Symbol, Boersenplatz, Zeitraum, Schlusskurs und Datenanzahl transparent darstellen, soweit bekannt.
- Marketstack-Aufrufe und API-Schluessel ausschliesslich im Rust-Backend halten.
- Die Implementierung in kleine, erklaerbare und automatisiert testbare Rust-Lernschritte aufteilen.
- Eine spaetere Android-Oberflaeche als weiteren Client derselben Backend-API ermoeglichen.

Nicht-Ziele des MVP:

- Intraday-Daten, frei waehlbare Zeitraeume oder Echtzeit-Kurse.
- Kauf, Verkauf, Portfolioverwaltung, Broker-Anbindung oder Anlageberatung.
- Eine Annahme, dass jede syntaktisch gueltige ISIN einem von Marketstack abfragbaren Symbol entspricht.
- Eine vollstaendige native Android-App oder ein produktiver Betrieb mit Verfuegbarkeitsgarantie.

## 3. Zielgruppen und Nutzungsszenarien

**Primaere Zielgruppe:** Der Projektentwickler als Rust-Anfaenger, der durch ein konkretes Beispiel Programmiergrundlagen, Tests und eine einfache Client-Server-Architektur lernen moechte.

**Primaeres Nutzungsszenario:** Ein Nutzer gibt eine ISIN ein und sieht den Verlauf der taeglichen Schlusskurse des letzten abgeschlossenen Kalendermonats. Ist die ISIN ungueltig, nicht zugeordnet oder gibt es keine Kursdaten, erhaelt er eine verstaendliche Rueckmeldung.

## 4. Funktionale Anforderungen

### ISIN und Instrument

- Das Eingabefeld akzeptiert eine ISIN, entfernt aeussere Leerzeichen und normalisiert Buchstaben auf Grossschreibung.
- Die ISIN wird nach Format und Pruefziffer validiert.
- Eine Marketstack-Symbolzuordnung wird nur verwendet, wenn sie fuer die ISIN und den Boersenplatz explizit geprueft wurde.
- Unbekannte ISINs und mehrdeutige oder fehlende Zuordnungen werden nicht geraten; sie erhalten eine verstaendliche Rueckmeldung.
- Name, ISIN, Marketstack-Symbol und Boersenplatz werden am Ergebnis angezeigt.

### Schlusskurse und Chart

- Das Backend fragt taegliche EOD-Daten fuer den letzten abgeschlossenen Kalendermonat ab.
- Der Chart verwendet ausschliesslich den vom Anbieter gelieferten Schlusskurs pro Handelstag; nicht vorhandene Handelstage werden nicht kuenstlich ergaenzt.
- Die Oberflaeche zeigt Zeitraum, Kurszeitpunkt, Schlusskurs, Datenanzahl, Boersenplatz und Waehrung, sofern verlaesslich bekannt.
- Der Datenstatus wird als taeglicher Schlusskurs gekennzeichnet. Die Anwendung behauptet keine Echtzeit- oder Intraday-Verfuegbarkeit.
- Laden, leere Daten, Fehler und erfolgreiche Darstellung sind erkennbare UI-Zustaende.

### Browser und Android

- Der erste Client laeuft im Browser und passt sich an Desktop- und mobile Bildschirmbreiten an.
- Das Backend stellt einen plattformneutralen JSON-Vertrag bereit.
- Eine spaetere Android-App kann dieselben Such-, Kurs- und Zeitreihen-Endpunkte verwenden.

## 5. Architektur und technische Leitlinien

- Das Rust-Backend ist die einzige Komponente, die Marketstack aufruft und den API-Schluessel besitzt.
- Browser-Frontend kommuniziert ausschliesslich mit der eigenen JSON-HTTP-API.
- Spaetere Android-App kommuniziert mit derselben API.
- Marketstack-Antworten werden in interne, anbieterunabhaengige Datentypen ueberfuehrt, bevor sie an Clients ausgegeben werden.
- ISIN-/Symbolzuordnung, Anwendungslogik, Marketstack-Adapter und HTTP-Endpunkte bleiben logisch getrennt. Fuer den Lernprototyp genuegt ein einzelnes Rust-Paket mit Modulen.
- API-Schluessel werden ueber eine Umgebungsvariable oder lokale, nicht eingecheckte Konfiguration bereitgestellt. Sie duerfen weder im Browser ausgeliefert noch in Git eingecheckt werden.
- Automatisierte Tests verwenden Fake-/Mock-Antworten und benoetigen weder einen Live-API-Key noch eine Netzwerkverbindung.
- Der aktuelle Browser-Client ist statisches HTML, CSS und JavaScript; er kommuniziert mit `GET /api/history?isin=...`.
- Die lokale Konfiguration wird aus `.env` geladen; `.env` ist von Git ausgeschlossen.

## 6. Nicht-funktionale Anforderungen

- **Bedienbarkeit:** Minimalistische, moderne Oberflaeche mit klarer Hierarchie und ohne unnoetige Dashboard-Elemente.
- **Robustheit:** API-Fehler, Zeitueberschreitungen, Rate Limits und unvollstaendige Antworten duerfen den Server nicht zum Absturz bringen.
- **Datenschutz:** Keine Benutzerkonten und keine Speicherung personenbezogener Daten im MVP.
- **Sicherheit:** Anbieter-Schluessel bleiben serverseitig. Fehlerantworten geben keine Geheimnisse oder internen Stacktraces preis.
- **Testbarkeit:** Fachlogik, Fehlerfaelle, API-Antworten und Anbieter-Mapping sind ohne externe Marktdatenverbindung testbar.
- **Lernbarkeit:** Jede Ausbaustufe hat ein kleines Ergebnis, erklaerbare Rust-Konzepte und einen reproduzierbaren Test oder Check.
- **Datenhinweis:** Kurszeitpunkt, Waehrung und bekannte Verzoegerung werden transparent angezeigt. Die App weist darauf hin, dass Daten zu Lernzwecken dienen und nicht als Grundlage fuer Anlageentscheidungen gedacht sind.

## 7. Akzeptanzkriterien fuer den MVP

1. Ein Nutzer kann eine gueltige, unterstuetzte ISIN eingeben und die Monatsdaten abrufen.
2. Die Anwendung zeigt taegliche Schlusskurse des letzten abgeschlossenen Kalendermonats in einem Chart.
3. Zeitraum, Instrument, Symbol, Boersenplatz, Schlusskurs und bekannte Waehrung werden angezeigt.
4. Ungueltige, nicht zugeordnete ISINs, leere Ergebnisse und Anbieterfehler erzeugen klare UI- und API-Fehlerzustande.
5. Die responsive Oberflaeche funktioniert auf Desktop- und mobilen Browserfenstern.
6. Browser-Code enthaelt keinen Marketstack-API-Schluessel und ruft Marketstack nicht direkt auf.
7. Fachlogik und API-Tests laufen mit Fake-/Mock-Daten ohne Netzwerk und ohne API-Schluessel.
8. Anbieterfelder werden vor der Ausgabe auf eigene, stabile JSON-Datentypen abgebildet.

## 8. Vorgesehene naechste Schritte

1. **Symbolzuordnung ausbauen:** weitere ISINs nur mit verifizierter Boerse und Marketstack-Symbolzuordnung aufnehmen.
2. **Anbieter- und API-Tests:** Adapter-Mapping, Fehlercodes und HTTP-Antworten mit Mock-Antworten abdecken.
3. **Datenhinweise vervollstaendigen:** bekannte Waehrung und Zeitstempel je Boersenlisting verlaesslich liefern.
4. **Android-Vorbereitung:** den JSON-Vertrag dokumentieren; eine native App bleibt ausserhalb des MVP.

## 9. Bekannte Grenzen und offene Punkte

- Der aktuelle Code kennt Zuordnungen fuer `IE00B0M62Q58` (`IQQW.XETRA`) und `IE00BKM4GZ66` (`EIMI.XSWX`). Diese Liste ist kein allgemeiner ISIN-Aufloesungsdienst.
- Ein Fonds kann mehrere Boersenlistings haben. Die Zuordnung muss zum gewuenschten Markt passen und darf nicht allein aufgrund eines gleichen Namens geraten werden.
- Verfuegbarkeit und Aktualitaet der EOD-Daten unterscheiden sich nach Symbol und Boerse. Vor Aufnahme eines weiteren Instruments sind Symbol, Boerse, Datenzeitraum und Tarifzugriff zu pruefen.
- Der aktuelle Marketstack-Tarif erlaubt fuer die bislang getesteten Symbole keinen Intraday-Zugriff; Intraday liegt ausserhalb des MVP.
- Waehrungsangaben muessen pro Listing verifiziert sein. Fehlende Angaben bleiben unbekannt und werden nicht geraten.
- Nutzungs-, Anzeige- und Attributionsbedingungen muessen vor einer Veroeffentlichung anhand offizieller Marketstack-Quellen geprueft werden.