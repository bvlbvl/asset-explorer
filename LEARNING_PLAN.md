# Lernpfad: Aktienkurse mit Rust

## Ziel und Architektur

Baue die Anwendung in kleinen, testbaren Schritten: zuerst ein Rust-Backend mit einer stabilen JSON-HTTP-API und eine schlanke Browser-Oberflaeche. Eine spaetere native Android-App wird ein weiterer Client derselben API. Dadurch bleiben Datenanbieter, API-Schluessel und fachliche Logik auf dem Server; Android muss weder den Rust-Code noch Anbieter-Schluessel einbetten.

Empfohlener Start: ein einzelnes Rust-Paket mit klar getrennten Modulen, noch kein Workspace. Trenne fachliche Datentypen, Anbieter-Anbindung und HTTP-Endpunkte logisch. Wenn spaeter Offline-Funktionen auf Android noetig werden, kann die Fachlogik in ein eigenes Rust-Crate verschoben und beispielsweise ueber UniFFI eingebunden werden. Diese Komplexitaet ist fuer den ersten Prototyp nicht noetig.

## Agentischer Lernrhythmus

Arbeite pro Schritt in kurzen Schleifen:

1. Bitte den Agenten, das Lernziel und die betroffenen Dateien zu erklaeren und eine kleine Aufgabe vorzuschlagen.
2. Implementiere die Aufgabe selbst. Lass dir zuerst Hinweise geben, nicht sofort die komplette Loesung.
3. Fuehre den genannten Test oder Check aus und lies die Fehlermeldung.
4. Bitte den Agenten um Review: Verhalten, Fehlerfaelle, Tests und Rust-Konzepte.
5. Uebernimm nur nachvollziehbare Aenderungen und notiere eine Erkenntnis fuer den naechsten Schritt.

Der Agent soll keine grossen Features am Stueck erzeugen. Jede Etappe endet mit einem sichtbaren Ergebnis und einem reproduzierbaren Check.

## Etappen

### 0. Marketstack-Anforderungen pruefen

Untersuche Marketstack anhand offizieller Quellen. Der Anbieter und der API-Key stehen bereits fest. Pruefe vor der Integration besonders direkte WKN-/ISIN-Aufloesung, deutsche Boersenabdeckung, Kursverzoegerung, Intraday-Verfuegbarkeit, Tariflimits und Nutzungsbedingungen. Halte nicht belegte Punkte offen und lege fest, wie mit nicht direkt unterstuetzten Kennungen umzugehen ist.

**Fertig, wenn:** die relevanten Marketstack-Grenzen dokumentiert und ein belastbarer Umgang mit nicht direkt unterstuetzten WKNs/ISINs festgelegt ist.

### 1. Rust-Grundlagen an der Fachlogik

Erweitere das Starterprogramm schrittweise um Eingabepruefung und fachliche Datentypen, zum Beispiel `InstrumentId` fuer WKN oder ISIN und einen Kurs mit Preis, Waehrung und Zeitstempel. Lerne dabei `enum`, `struct`, `Result`, `Option`, Module und Unit-Tests. Noch keine Netzwerkaufrufe.

**Fertig, wenn:** gueltige und ungueltige Kennungen durch Unit-Tests abgedeckt sind und Fehler als Werte behandelt werden, statt das Programm abstuerzen zu lassen.

### 2. Anbieter unabhaengig modellieren

Definiere ein internes Modell fuer Instrument, letzten Kurs und historische Kursreihe. Lege fest, wie Zeitstempel, Waehrung, Boerse und Datenqualitaet (Echtzeit, verzoegert oder Schlusskurs) dargestellt werden. Vermeide Anbieter-spezifische Feldnamen in der Fachlogik.

**Fertig, wenn:** Beispielantworten des ausgewaehlten Anbieters in das interne Modell uebersetzt werden koennen, ohne die API-Antwort direkt an die UI weiterzugeben.

### 3. HTTP-API mit Fake-Daten

Fuege einen Rust-Webserver (beispielsweise Axum) und einen Endpunkt zum Abrufen eines Kurses hinzu. Beginne mit einem Fake-Anbieter, damit Routing, JSON und Fehlerantworten ohne API-Key oder Netzwerk testbar sind. Halte die Anbindung hinter einer kleinen Schnittstelle.

**Fertig, wenn:** automatisierte Tests den erfolgreichen Abruf, unbekannte Kennungen und Anbieterfehler abdecken.

### 4. Echte Marktdaten anbinden

Ersetze den Fake-Anbieter durch einen Adapter mit `reqwest` und `serde`. Lies den API-Key aus einer Umgebungsvariable, setze sinnvolle Timeouts und mappe Anbieterfehler auf eigene Fehler. Verwende gespeicherte Beispielantworten oder einen Mock-HTTP-Server in Tests; Tests sollen nicht von einem Live-API-Key abhaengen.

**Fertig, wenn:** eine manuelle lokale Abfrage funktioniert und die automatisierten Tests offline laufen.

### 5. Browser-Oberflaeche und Charts

Erstelle eine minimalistische, responsive Browser-Oberflaeche: WKN/ISIN-Eingabe, Kurs samt Zeitstempel und Waehrung sowie ein Chart mit Bereichen fuer Tage, Wochen, Monate und Jahre. Die UI ruft ausschliesslich die eigene Rust-API auf. Waehle die Chart-Bibliothek erst, wenn das Datenmodell und die benoetigte Aufloesung feststehen.

**Fertig, wenn:** Lade-, Leer-, Fehler- und Erfolgszustand funktionieren und Zeitraumwechsel die passende Kursreihe anfordern.

### 6. Android als zweiter Client

Dokumentiere die JSON-Endpunkte und nutze sie aus einer spaeteren Android-App. Halte Antworten versionierbar und plattformneutral. Eine native Kotlin-Oberflaeche kann denselben Server verwenden; es ist keine zweite Anbieter-Integration erforderlich.

**Fertig, wenn:** ein API-Vertrag mit Beispielanfragen und -antworten vorliegt und die Android-App dieselben Kurs- und Zeitreihendaten beziehen kann.

## Leitplanken

- Keine echten Kauf-/Verkaufsfunktionen und keine Anlageberatung.
- API-Schluessel bleiben serverseitig und werden nicht eingecheckt.
- Fachlogik und Tests zuerst, echte externe API-Aufrufe spaeter.
- Bei jedem Schritt erklaerst du die Rust-Konzepte in eigenen Worten, bevor der Agent die naechste Etappe aufmacht.
- Anbieter-Limits, Marktabdeckung und Nutzungsbedingungen sind zeitabhaengig; Quellen und Recherchezeitpunkt gehoeren in die Notizen.