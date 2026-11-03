# Lernpfad: Aktienkurse mit Rust

## Ziel und Architektur

Baue die Anwendung in kleinen, testbaren Schritten: zuerst ein Rust-Backend mit einer stabilen JSON-HTTP-API und eine schlanke Browser-Oberflaeche. Eine spaetere native Android-App wird ein weiterer Client derselben API. Dadurch bleiben Datenanbieter, API-Schluessel und fachliche Logik auf dem Server; Android muss weder den Rust-Code noch Anbieter-Schluessel einbetten.

Empfohlener Start: ein einzelnes Rust-Paket mit klar getrennten Modulen, noch kein Workspace. Trenne fachliche Datentypen, Anbieter-Anbindung und HTTP-Endpunkte logisch. Wenn spaeter Offline-Funktionen auf Android noetig werden, kann die Fachlogik in ein eigenes Rust-Crate verschoben und beispielsweise ueber UniFFI eingebunden werden. Diese Komplexitaet ist fuer den ersten Prototyp nicht noetig.

## Agentischer Lernrhythmus

Das Lernziel ist ein durchgaengiger agentischer Entwicklungsprozess. Der Nutzer formuliert das gewuenschte Ergebnis, steuert fachliche Entscheidungen und prueft das Resultat; GitHub Copilot uebernimmt die konkrete Codearbeit. Der Nutzer muss die Implementierung nicht selbst von Hand schreiben, soll aber Tests, Diffs und technische Erklaerungen nachvollziehen koennen.

Arbeite pro Schritt in kurzen Schleifen:

1. Beschreibe dem Agenten Ziel, Grenzen und beobachtbare Abnahmekriterien.
2. Lass den Agenten Kontext, betroffene Dateien, lokale Hypothese und den guenstigsten Check ermitteln.
3. Lass zuerst einen passenden Test schreiben und ausfuehren, dann die kleinste passende Implementierung erstellen.
4. Lass Tests, Formatierung und relevante Browser- oder API-Pruefungen ausfuehren und die Ergebnisse erklaeren.
5. Pruefe die Aenderungen und entscheide, ob sie das Ziel erfuellen; halte offene Punkte und den verifizierten Stand in den Projektdokumenten fest.

Der Agent soll keine grossen Features am Stueck erzeugen. Jede Etappe endet mit einem sichtbaren Ergebnis und einem reproduzierbaren Check. Der Lerngewinn kommt aus Anforderungen, Reviews, Tests und Erklaerungen, nicht aus manuellem Abtippen der Loesung.

## Etappen

### 0. Marketstack-Anforderungen pruefen

**Status:** Teilweise offen. Die Implementierung und einzelne Smoke-Tests belegen Datenpfade; eine aktuelle, vollstaendig belegte Recherche zu Tarif, Nutzungsbedingungen und Attribution bleibt separat zu verifizieren.

Untersuche Marketstack anhand offizieller Quellen. Der Anbieter und der API-Key stehen bereits fest. Pruefe vor der Integration besonders direkte WKN-/ISIN-Aufloesung, deutsche Boersenabdeckung, Kursverzoegerung, Intraday-Verfuegbarkeit, Tariflimits und Nutzungsbedingungen. Halte nicht belegte Punkte offen und lege fest, wie mit nicht direkt unterstuetzten Kennungen umzugehen ist.

**Fertig, wenn:** die relevanten Marketstack-Grenzen dokumentiert und ein belastbarer Umgang mit nicht direkt unterstuetzten WKNs/ISINs festgelegt ist.

### 1. Rust-Grundlagen an der Fachlogik

**Status:** Im aktuellen MVP umgesetzt: ISIN-Validierung, Instrument- und Kursmodelle, Monatsgrenzen sowie Unit-Tests.

Erweitere das Starterprogramm schrittweise um Eingabepruefung und fachliche Datentypen, zum Beispiel `InstrumentId` fuer WKN oder ISIN und einen Kurs mit Preis, Waehrung und Zeitstempel. Lerne dabei `enum`, `struct`, `Result`, `Option`, Module und Unit-Tests. Noch keine Netzwerkaufrufe.

**Fertig, wenn:** gueltige und ungueltige Kennungen durch Unit-Tests abgedeckt sind und Fehler als Werte behandelt werden, statt das Programm abstuerzen zu lassen.

### 2. Anbieter unabhaengig modellieren

**Status:** Im aktuellen MVP umgesetzt: interne `Instrument`, `DailyClose` und `MonthlyHistory`-Typen trennen den Client von Anbieterantworten.

Definiere ein internes Modell fuer Instrument, letzten Kurs und historische Kursreihe. Lege fest, wie Zeitstempel, Waehrung, Boerse und Datenqualitaet (Echtzeit, verzoegert oder Schlusskurs) dargestellt werden. Vermeide Anbieter-spezifische Feldnamen in der Fachlogik.

**Fertig, wenn:** Beispielantworten des ausgewaehlten Anbieters in das interne Modell uebersetzt werden koennen, ohne die API-Antwort direkt an die UI weiterzugeben.

### 3. HTTP-API mit Fake-Daten

**Status:** Im aktuellen MVP umgesetzt: Axum-Endpunkt, Provider-Abstraktion und Offline-Tests mit Fake-Daten.

Fuege einen Rust-Webserver (beispielsweise Axum) und einen Endpunkt zum Abrufen eines Kurses hinzu. Beginne mit einem Fake-Anbieter, damit Routing, JSON und Fehlerantworten ohne API-Key oder Netzwerk testbar sind. Halte die Anbindung hinter einer kleinen Schnittstelle.

**Fertig, wenn:** automatisierte Tests den erfolgreichen Abruf, unbekannte Kennungen und Anbieterfehler abdecken.

### 4. Echte Marktdaten anbinden

**Status:** Im aktuellen MVP umgesetzt: Marketstack EOD sowie OpenFIGI-/Ticker-Aufloesung; automatisierte Tests bleiben offline.

Ersetze den Fake-Anbieter durch einen Adapter mit `reqwest` und `serde`. Lies den API-Key aus einer Umgebungsvariable, setze sinnvolle Timeouts und mappe Anbieterfehler auf eigene Fehler. Verwende gespeicherte Beispielantworten oder einen Mock-HTTP-Server in Tests; Tests sollen nicht von einem Live-API-Key abhaengen.

**Fertig, wenn:** eine manuelle lokale Abfrage funktioniert und die automatisierten Tests offline laufen.

### 5. Browser-Oberflaeche und Charts

**Status:** MVP-Teilziel umgesetzt: responsive ISIN-Eingabe, Session-Historie und Monatschart. Frei waehlbare Zeitraeume und mehrere Webseiten sind nicht implementiert.

Erstelle eine minimalistische, responsive Browser-Oberflaeche: WKN/ISIN-Eingabe, Kurs samt Zeitstempel und Waehrung sowie ein Chart mit Bereichen fuer Tage, Wochen, Monate und Jahre. Die UI ruft ausschliesslich die eigene Rust-API auf. Waehle die Chart-Bibliothek erst, wenn das Datenmodell und die benoetigte Aufloesung feststehen.

**Fertig, wenn:** Lade-, Leer-, Fehler- und Erfolgszustand funktionieren und Zeitraumwechsel die passende Kursreihe anfordern.

### 6. Android als zweiter Client

**Status:** Geplant, nicht implementiert.

Dokumentiere die JSON-Endpunkte und nutze sie aus einer spaeteren Android-App. Halte Antworten versionierbar und plattformneutral. Eine native Kotlin-Oberflaeche kann denselben Server verwenden; es ist keine zweite Anbieter-Integration erforderlich.

**Fertig, wenn:** ein API-Vertrag mit Beispielanfragen und -antworten vorliegt und die Android-App dieselben Kurs- und Zeitreihendaten beziehen kann.

## Leitplanken

- Keine echten Kauf-/Verkaufsfunktionen und keine Anlageberatung.
- API-Schluessel bleiben serverseitig und werden nicht eingecheckt.
- Fachlogik und Tests zuerst, echte externe API-Aufrufe spaeter.
- Bei jedem Schritt erklaerst du die Rust-Konzepte in eigenen Worten, bevor der Agent die naechste Etappe aufmacht.
- Anbieter-Limits, Marktabdeckung und Nutzungsbedingungen sind zeitabhaengig; Quellen und Recherchezeitpunkt gehoeren in die Notizen.