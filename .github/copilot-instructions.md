# Project Guidelines

## Lernstil

- Behandle dieses Projekt als Rust-Lernprojekt fuer einen Anfaenger. Erklaere neue Rust-Konzepte knapp und konkret anhand der aktuellen Aufgabe.
- Arbeite in kleinen, nachvollziehbaren Schritten. Bevorzuge Hinweise und erklaerbare Aenderungen gegenueber grossen, automatisch erzeugten Loesungen.
- Fuehre nach Codeaenderungen passende Tests oder Checks aus und fasse deren Ergebnis kurz zusammen.
- Verwende einen Recherche-Subagenten fuer dieses Projekt nur falls es unbedingt noptwendig ist. Bevorzuge die Nutzung von offiziellen Dokumentationen und Preis-/Nutzungsbedingungen fuer Anbieterinformationen. Frage vor der erstellung eines Subagenten, ob die Aufgabe nicht auch ohne ihn erledigt werden kann.

## Produkt und Architektur

- Massgebliche Produktanforderungen stehen in [PRD.md](../PRD.md); der Lernfahrplan steht in [LEARNING_PLAN.md](../LEARNING_PLAN.md).
- Die Anwendung zeigt Kursinformationen zu Aktien, zuerst im Browser und spaeter gegebenenfalls in einer Android-App. Sie ist kein Trading- oder Anlageberatungsprodukt.
- Verwende Marketstack als vorgesehenen Marktdatenanbieter. Der API-Schluessel stammt aus dem Marketstack-Konto und wird lokal/serverseitig als `MARKETSTACK_API_KEY` bereitgestellt. Pruefe Identifier-Unterstuetzung, Boersenabdeckung, Aktualitaet, Tariflimits und Nutzungsbedingungen anhand verfuegbarer Dokumentation; behaupte keine unbestaetigten Eigenschaften.
- Halte den API-Schluessel serverseitig. Lege keine Schluessel in Quellcode, Browser-Bundles oder Git ab.
- Halte Anbieterantworten hinter einer Rust-Schnittstelle und mappe sie auf eigene, stabile Datentypen und JSON-Antworten. Browser und spaetere Android-App sollen dieselbe Backend-API verwenden.
- Beginne mit einer einfachen Modulstruktur in diesem Rust-Paket. Fuehre keine Workspace-, WASM- oder native Android-Komplexitaet ein, bevor ein konkreter Bedarf besteht.

## Tests und Fehlerbehandlung

- Schreibe Tests fuer Fachlogik, Eingabevalidierung, API-Antworten und Anbieter-Mapping.
- Tests muessen ohne echten API-Schluessel und ohne Netzwerkzugriff laufen; verwende Fake- oder Mock-Antworten.
- Behandle externe Fehler, fehlende Daten, Zeitueberschreitungen und Rate Limits nachvollziehbar. Gib keine Geheimnisse oder internen Stacktraces in API-Fehlerantworten aus.
- Zeige Kurszeitpunkt, Waehrung und bekannte Verzoegerung transparent an. Kennzeichne Daten nicht als Echtzeit, wenn das nicht belegt ist.