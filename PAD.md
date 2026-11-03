# Project Architecture Document (PAD)

**Rolle dieses Dokuments:** stabile Projektinformationen und Architekturprinzipien. Der aktuelle Fortschritt gehoert ausschliesslich in [STATUS.md](STATUS.md); der jeweils aktive Auftrag steht in [TASK.md](TASK.md).

**Dokumente:** [Produktanforderungen](PRD.md) · [Aktueller Status](STATUS.md) · [Aktive Task](TASK.md) · [PIV-Prozess](PIV.md) · [Agenten-Einstieg](.github/copilot-instructions.md)

## Projekt

Der Aktienkurs-Explorer ist ein Rust-Lernprojekt. Ein Nutzer gibt im Browser eine ISIN ein und sieht taegliche Schlusskurse des letzten abgeschlossenen Kalendermonats als Chart. Das Produkt dient der Information und dem Lernen, nicht dem Handel oder der Anlageberatung. Die verbindlichen Produktanforderungen stehen in [PRD.md](PRD.md).

## Architekturprinzipien

- Ein Rust-Paket mit klar getrennten Modulen; kein Workspace, WASM oder native Android-App ohne konkreten Bedarf.
- Das Backend besitzt den Marketstack-Zugriff und gibt eine stabile, anbieterunabhaengige JSON-Antwort aus.
- Browser und spaetere Clients sprechen nur mit der eigenen HTTP-API.
- Fachlogik und Tests muessen ohne Netzwerk und API-Schluessel funktionieren.
- ISINs werden validiert und nur ueber eine explizit gepruefte Boersen-Symbolzuordnung aufgeloest. Unbekannte Zuordnungen werden nicht geraten.
- Kurszeitpunkt, Boersenplatz, Waehrung und Datenqualitaet werden nur dargestellt, wenn sie bekannt oder verifiziert sind.

## Bausteine im aktuellen Entwurf

- `src/domain.rs`: interne Instrument- und Kursmodelle, ISIN-Pruefung, Symbolzuordnung und Monatsgrenzen.
- `src/service.rs`: Anwendungsablauf fuer die Monatsabfrage.
- `src/marketstack.rs`: Anbieter-Schnittstelle und Marketstack-Adapter.
- `src/api.rs`: HTTP-Route und oeffentliche Fehlerantworten.
- `src/main.rs`: Konfiguration und Serverstart.
- `public/`: statischer Browser-Client.
- `GET /api/history?isin=...`: aktueller JSON-Endpunkt.

Die ausfuehrliche Mermaid-Architektur und der Code-Rundgang werden gemaess der aktiven [Task](TASK.md) in `docs/ARCHITECTURE.md` erstellt. Dieses PAD bleibt eine knappe Uebersicht und soll das Detaildokument nicht duplizieren.

## Geheimnisse

`MARKETSTACK_API_KEY` wird nur serverseitig aus der Prozessumgebung oder einer lokalen, von Git ignorierten `.env` geladen. Ein Schluessel gehoert nie in Quellcode, Browser-Dateien, Logs, Chat oder Git.

## Entwicklungschecks

- `cargo fmt --check`: Rust-Formatierung pruefen.
- `cargo test`: Offline-Tests ohne API-Schluessel oder Netzwerk ausfuehren.
- `cargo run`: lokalen Browser-Prototyp mit Rust-Backend starten.

## Test- und Verifikationspflicht fuer Aenderungen

- Jede neue Funktion oder jeder neue Feature-Block beginnt mit einem passenden lokalen Test, der das erwartete Verhalten oder den zu fixenden Fehler reproduziert.
- Ein Test gilt als Konvention, nicht als optionales Extra: Wenn ein Feature implementiert wird, muss der relevante Testlauf lokal und reproduzierbar durchgefuehrt werden.
- Der Agent darf einen Aenderungsblock nicht als fertig bezeichnen, solange der passende Test noch fehlt oder fehlschlaegt.
- Fehlersichtbarkeit ist Teil der Verifikation: Ein lokaler Testlauf muss die betroffene Stelle, die Ursache und den naechsten Korrekturpunkt klar benennen, damit der Agent nicht mit Kontext-Rauschen und Raten arbeiten muss.
- Regressionspruefungen sollten lokal und fokussiert laufen; bei einem Fehler wird derselbe Test nach der Korrektur erneut ausgefuehrt, bevor ein weiterer Scope betrachtet wird.
- Der Projektworkflow setzt auf lokale Verifikation statt auf manuelle KI-Interpretation externer Ergebnisse.

## Arbeitskontext

- [PRD.md](PRD.md) definiert Ziele, Umfang und Abnahmekriterien.
- [STATUS.md](STATUS.md) haelt den veraenderlichen Implementierungsstand und die zuletzt ausgefuehrten Checks fest.
- [TASK.md](TASK.md) enthaelt genau den Auftrag fuer den naechsten Agentenlauf.
- [PIV.md](PIV.md) beschreibt den Plan-Implement-Verify-Prozess.
- [`.github/copilot-instructions.md`](.github/copilot-instructions.md) ist der kurze, automatisch geladene Agenten-Einstieg.
