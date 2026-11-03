# Product Requirements Document: Aktienkurs-Explorer

**Status:** Entwurf  
**Stand:** 2026-11-03

## 1. Produktueberblick

Der Aktienkurs-Explorer ist ein Lernprojekt, mit dem Nutzer ein Wertpapier ueber WKN oder ISIN suchen und den zuletzt verfuegbaren Kurs sowie historische Kursdaten als Chart ansehen koennen. Die Anwendung wird zuerst im Browser nutzbar gemacht. Eine spaetere Android-App soll dieselbe Backend-API verwenden.

Die Marktdaten sollen von Marketstack bezogen werden. Der API-Schluessel wird im Marketstack-Konto verwaltet und fuer die Anwendung als `MARKETSTACK_API_KEY` lokal beziehungsweise serverseitig konfiguriert. Ob und wie Marketstack WKN und ISIN direkt aufloest, welche Boersen und Intervalle im verwendeten Tarif verfuegbar sind und wie aktuell die Kurse sind, ist vor der Integration zu pruefen. Dieser PRD behauptet keine nicht verifizierten Anbieterfunktionen.

Neben einem funktionierenden Prototyp ist das Lernziel zentral: Rust-Konzepte, Tests, API-Design und schrittweise Architektur sollen anhand des Produkts nachvollziehbar eingeuebt werden.

## 2. Ziele

- Ein Wertpapier ueber WKN oder ISIN suchen koennen.
- Den zuletzt verfuegbaren Kurs mit Waehrung und Zeitangabe anzeigen.
- Historische Kursdaten in einem Chart fuer kurze und lange Zeitraeume betrachten.
- Zuerst eine responsive Browser-Oberflaeche bereitstellen.
- Eine spaetere Android-Oberflaeche ermoeglichen, ohne eine zweite Marktdaten-Integration zu bauen.
- Die Implementierung in kleine, erklaerbare und automatisiert testbare Rust-Lernschritte aufteilen.
- Marketstack fuer Marktdaten verwenden; API-Schluessel und externe Anbieter-Aufrufe bleiben serverseitig.

## 3. Nicht-Ziele

- Kauf, Verkauf, Portfolioverwaltung, Broker-Anbindung oder Anlageberatung.
- Produktionsbetrieb mit Verfuegbarkeitsgarantie oder verbindlicher Kurslatenz.
- Nutzerkonten, Personalisierung oder dauerhafte Speicherung von Suchverlaeufen.
- Ein in die App integrierter KI-Agent.
- Eine vollstaendige native Android-App im ersten Lieferumfang.
- Eine Annahme, dass jeder beliebige WKN-/ISIN-Wert von Marketstack direkt akzeptiert wird.

## 4. Zielgruppen und Nutzungsszenarien

**Primaere Zielgruppe:** Der Projektentwickler als Rust-Anfaenger, der durch ein konkretes Beispiel Programmiergrundlagen, Tests und eine einfache Client-Server-Architektur lernen moechte.

**Primaeres Nutzungsszenario:** Ein Nutzer gibt eine WKN oder ISIN ein, waehlt ein gefundenes Wertpapier aus und sieht den letzten verfuegbaren Kurs. Anschliessend wechselt er zwischen Chart-Zeitraeumen von Tagen, Wochen, Monaten und Jahren.

## 5. Funktionale Anforderungen

### Suche und Instrument

- Die Suche akzeptiert WKN oder ISIN und behandelt fuehrende/nachfolgende Leerzeichen sowie Gross-/Kleinschreibung robust.
- Die Anwendung zeigt das aufgeloeste Instrument mit Name und Symbol an, sofern diese Angaben verfuegbar sind.
- Wenn ein Identifier nicht direkt von Marketstack verarbeitet werden kann, muss die Anwendung dies erkennen. Eine Symbolzuordnung darf nur verwendet werden, wenn sie eindeutig ist; andernfalls wird der Nutzer um eine Auswahl oder eine andere Eingabe gebeten.
- Ungueltige Eingaben, keine Treffer und mehrdeutige Treffer erhalten verstaendliche Rueckmeldungen.

### Kursanzeige

- Die Detailansicht zeigt den letzten vom Anbieter gelieferten Kurs, die Waehrung und den Zeitstempel der Kursdaten, sofern verfuegbar.
- Die Anwendung kennzeichnet Daten als Echtzeit, verzoegert oder Schlusskurs, wenn Marketstack diese Information bereitstellt oder sie fuer den gewaehlten Datenzugang verlaesslich bekannt ist.
- Nicht verfuegbare Aktualitaetsangaben werden nicht als Echtzeit ausgegeben.

### Charts

- Fuer ein ausgewaehltes Instrument kann eine historische Kursreihe geladen und als Chart dargestellt werden.
- Der Nutzer kann zwischen Zeitraeumen fuer Tage, Wochen, Monate und Jahre wechseln.
- Die angeforderte Aufloesung und Datenmenge richten sich nach dem Zeitraum und den von Marketstack verfuegbaren Intervallen.
- Falls Intraday-Daten fuer einen Zeitraum oder Tarif nicht verfuegbar sind, zeigt die Anwendung die bestmoegliche verfuegbare Aufloesung oder erklaert, dass keine Daten geladen werden konnten.
- Laden, leere Daten, Fehler und erfolgreiche Darstellung sind erkennbare UI-Zustaende.

### Browser und Android

- Der erste Client laeuft im Browser und passt sich an Desktop- und mobile Bildschirmbreiten an.
- Das Backend stellt einen plattformneutralen JSON-Vertrag bereit.
- Eine spaetere Android-App kann dieselben Such-, Kurs- und Zeitreihen-Endpunkte verwenden.

## 6. Architektur und technische Leitlinien

- Rust-Backend als einzige Komponente, die Marketstack aufruft und den API-Schluessel besitzt.
- Browser-Frontend kommuniziert ausschliesslich mit der eigenen JSON-HTTP-API.
- Spaetere Android-App kommuniziert mit derselben API.
- Marketstack-Antworten werden in interne, anbieterunabhaengige Datentypen ueberfuehrt, bevor sie an Clients ausgegeben werden.
- Anbieterzugriff, fachliche Typen und HTTP-Endpunkte bleiben logisch getrennt. Fuer den Lernprototyp genuegt ein einzelnes Rust-Paket mit Modulen.
- API-Schluessel werden ueber eine Umgebungsvariable oder lokale, nicht eingecheckte Konfiguration bereitgestellt. Sie duerfen weder im Browser ausgeliefert noch in Git eingecheckt werden.
- Automatisierte Tests verwenden Fake-/Mock-Antworten und benoetigen weder einen Live-API-Key noch eine Netzwerkverbindung.
- Zuerst wird die JSON-API mit Fake-Daten entwickelt; anschliessend wird der Marketstack-Adapter ergaenzt.
- Ein konkretes Frontend-Framework und eine Chart-Bibliothek sind noch nicht festgelegt.

## 7. Nicht-funktionale Anforderungen

- **Bedienbarkeit:** Minimalistische, moderne Oberflaeche mit klarer Hierarchie und ohne unnoetige Dashboard-Elemente.
- **Robustheit:** API-Fehler, Zeitueberschreitungen, Rate Limits und unvollstaendige Antworten duerfen den Server nicht zum Absturz bringen.
- **Datenschutz:** Keine Benutzerkonten und keine Speicherung personenbezogener Daten im MVP.
- **Sicherheit:** Anbieter-Schluessel bleiben serverseitig. Fehlerantworten geben keine Geheimnisse oder internen Stacktraces preis.
- **Testbarkeit:** Fachlogik, Fehlerfaelle, API-Antworten und Anbieter-Mapping sind ohne externe Marktdatenverbindung testbar.
- **Lernbarkeit:** Jede Ausbaustufe hat ein kleines Ergebnis, erklaerbare Rust-Konzepte und einen reproduzierbaren Test oder Check.
- **Datenhinweis:** Kurszeitpunkt, Waehrung und bekannte Verzoegerung werden transparent angezeigt. Die App weist darauf hin, dass Daten zu Lernzwecken dienen und nicht als Grundlage fuer Anlageentscheidungen gedacht sind.

## 8. Akzeptanzkriterien fuer das MVP

1. Ein Nutzer kann im Browser eine WKN oder ISIN eingeben und erhaelt entweder ein eindeutig aufgeloestes Instrument oder eine verstaendliche Fehlermeldung.
2. Zu einem aufgeloesten Instrument werden Kurs, Waehrung und Datenzeitpunkt angezeigt, sofern der Anbieter sie liefert.
3. Ein Nutzer kann die Zeitraeume Tage, Wochen, Monate und Jahre auswaehlen und erhaelt die jeweils verfuegbare historische Kursreihe.
4. Die Oberflaeche hat sichtbare Lade-, Erfolgs-, Leer- und Fehlerzustaende und funktioniert auf Desktop- sowie mobilen Browserfenstern.
5. Browser-Code enthaelt keinen Marketstack-API-Schluessel und ruft Marketstack nicht direkt auf.
6. Backend-Tests decken mindestens erfolgreiche Antworten sowie ungueltige Eingaben, unbekannte Instrumente und Anbieterfehler ab und laufen ohne Netzwerkzugriff.
7. Die API-Antworten sind nicht an Marketstack-Feldnamen gebunden und koennen von einem spaeteren Android-Client verwendet werden.
8. Die Anwendung kennzeichnet Kursaktualitaet nicht staerker, als es die vorhandenen Anbieterinformationen rechtfertigen.

## 9. Vorgesehene Lieferetappen

1. **Fachliche Grundlagen:** WKN-/ISIN-Eingabe, interne Typen, Validierung und Unit-Tests.
2. **JSON-API mit Fake-Daten:** Endpunkte und Fehlerverhalten ohne externe Abhaengigkeit.
3. **Marketstack-Integration:** Identifier-Aufloesung, Quote und Zeitreihe anhand offizieller Dokumentation und manueller Tests pruefen.
4. **Browser-MVP:** Suche, Kursanzeige, Zeitraum-Auswahl, Chart und UI-Zustaende.
5. **Stabilisierung:** Mock-basierte Tests, API-Key-Konfiguration, Rate-Limit- und Fehlerbehandlung sowie Nutzungs-/Datenhinweise.
6. **Android-Vorbereitung:** API-Vertrag dokumentieren; native Android-Implementierung bleibt ein spaeterer Schritt.

## 10. Offene Punkte und Risiken

- Unterstuetzt Marketstack die direkte Abfrage deutscher WKNs und ISINs? Falls nicht, welche belastbare Zuordnung auf ein Marketstack-Symbol wird verwendet?
- Welche deutschen Boersen und Instrumente sind im verfuegbaren Zugang abgedeckt?
- Welche Aktualitaet, Intraday-Intervalle, Historientiefe und Limits gelten fuer den gewaehlten API-Zugang?
- Welche Nutzungs-, Anzeige- und Attributionsbedingungen gelten fuer einen lokalen Lernprototyp?
- Wie werden mehrere Treffer, unterschiedliche Boersenplaetze und Waehrungen dargestellt?
- Welches Frontend-Framework und welche Chart-Bibliothek werden fuer den Browser gewaehlt?
- Soll Android spaeter nativ (zum Beispiel Kotlin) oder als installierbare Web-App umgesetzt werden? Der MVP legt dies nicht fest.

Diese Punkte muessen vor der echten Marketstack-Integration anhand der offiziellen Dokumentation und kleiner manueller Requests geprueft werden. Die Auswahl von Marketstack ist eine Produktentscheidung des Nutzers, keine bestaetigte Aussage ueber kostenlose Nutzung, Identifier-Abdeckung oder Kursaktualitaet.