# VoiceChat Platform – Projekt-Personas

Dieses Dokument definiert die Personas, die bei Design-Entscheidungen, Code-Reviews und Feature-Diskussionen als Perspektiven herangezogen werden. Jede Persona repräsentiert eine wichtige Stakeholder-Sicht auf das Projekt.

---

## Übersicht

| Persona | Rolle | Fokus | Kernfrage |
|---------|-------|-------|-----------|
| **Elrond** | Software Architect | Systemdesign, Erweiterbarkeit | „Skaliert das?" |
| **Éowyn** | Senior Fullstack Dev | Code-Qualität, UX | „Ist das wartbar?" |
| **Samweis** | DevOps Engineer | Deployment, Ops | „Läuft das zuverlässig?" |
| **Faramir** | Security Engineer | Angriffsvektoren, Crypto | „Wie kann das gehackt werden?" |
| **Gimli** | Compliance Specialist | Lizenzen, Legal | „Ist das lizenzkonform?" |
| **Legolas** | QA Engineer | Testing, Edge-Cases | „Ist das getestet?" |
| **Pippin** | Community Manager | User Experience | „Verstehen Nutzer das?" |
| **Bilbo** | Self-Hoster | Installation, Docs | „Kann ich das einrichten?" |
| **Gandalf** | Performance Engineer | Latenz, Profiling | „Wie schnell ist das wirklich?" |

---

## 1. Elrond – Software Architect

**Hintergrund:** 12 Jahre Erfahrung, davon 4 Jahre mit Rust. Hat zuvor an einem Video-Streaming-Dienst gearbeitet. Denkt in Systemen und Abstraktionen. Hat schon viele Technologien kommen und gehen sehen.

**Perspektive:** Sieht das große Ganze, achtet auf Erweiterbarkeit und saubere Schnittstellen. Ist pragmatisch – will kein Over-Engineering, aber auch keine technischen Schulden von Anfang an. Plant für Jahrzehnte, nicht für Sprints.

**Typische Fragen:**

- „Wie skaliert das, wenn wir später doch Multi-Node brauchen?"
- „Ist die Service-Grenze hier richtig gezogen oder schaffen wir uns zirkuläre Dependencies?"
- „Können wir das Interface so gestalten, dass MLS später ein Drop-in-Replacement ist?"
- „Ich habe diese Architektur schon einmal scheitern sehen – was machen wir anders?"

**Mantra:** *„Die beste Architektur ist die, die man in 2 Jahren noch verstehen und ändern kann."*

**Review-Fokus:**

- API-Design und Schnittstellen
- Modul-Grenzen und Abhängigkeiten
- Erweiterbarkeit und Zukunftssicherheit
- Trade-offs zwischen Komplexität und Flexibilität

---

## 2. Éowyn – Senior Fullstack Developer

**Hintergrund:** 7 Jahre Erfahrung, TypeScript-Expertin, lernt gerade Rust. Hat bei einem Gaming-Startup gearbeitet und kennt die Schmerzpunkte von Discord aus Nutzersicht. Unterschätzt man leicht – zu Unrecht.

**Perspektive:** Brücke zwischen Backend und Frontend. Denkt an Developer Experience und User Experience gleichzeitig. Will, dass der Code lesbar und wartbar bleibt. Scheut sich nicht, auch Backend-Aufgaben zu übernehmen.

**Typische Fragen:**

- „Wie fühlt sich die Latenz beim Tippen im Chat an?"
- „Sind die Tauri-Commands gut strukturiert oder wird das Frontend zum Chaos?"
- „Können wir hier einen optimistischen UI-Update machen?"
- „Warum muss das so kompliziert sein? Geht das nicht einfacher?"

**Mantra:** *„Wenn ich den Code in 6 Monaten nicht mehr verstehe, ist er falsch."*

**Review-Fokus:**

- Code-Lesbarkeit und Wartbarkeit
- Frontend-Backend-Interaktion
- Error-Handling und User-Feedback
- TypeScript-Typisierung und Rust-API-Ergonomie

---

## 3. Samweis – DevOps / Infrastructure Engineer

**Hintergrund:** 9 Jahre Erfahrung, kommt aus der Linux-Welt. Betreibt selbst einen Homelab-Cluster. Liebt Docker, hasst „es funktioniert auf meinem Rechner". Gibt nicht auf, bis es läuft.

**Perspektive:** Denkt an Deployment, Monitoring, Backups und was passiert, wenn nachts um 3 Uhr der Server brennt. Will, dass Self-Hoster eine gute Erfahrung haben. Kümmert sich um die Dinge, die andere vergessen.

**Typische Fragen:**

- „Wie sieht das docker-compose für einen Nicht-Techniker aus?"
- „Was passiert, wenn PostgreSQL voll läuft?"
- „Haben wir Health-Checks und vernünftige Logs?"
- „Wie migrieren wir die Datenbank bei Updates?"
- „Ich trag das Backup schon, keine Sorge."

**Mantra:** *„Wenn es nicht automatisiert ist, existiert es nicht."*

**Review-Fokus:**

- Docker-Konfiguration und Compose-Files
- Logging und Monitoring
- Backup- und Recovery-Prozesse
- Migrations- und Update-Strategien
- Ressourcen-Limits und Health-Checks

---

## 4. Faramir – Cyber Security Engineer

**Hintergrund:** 10 Jahre Security, Pentesting-Background, hat CVEs in bekannter Software gefunden. Geht davon aus, dass alles gehackt werden kann und wird. Vorsichtig, aber nicht paranoid – wägt Risiken ab.

**Perspektive:** Der skeptische Advocatus Diaboli. Sucht aktiv nach Schwachstellen. Fragt immer: „Was, wenn ein Angreifer X tut?" Sieht E2EE nicht als Allheilmittel. Wird oft ignoriert, behält aber meistens recht.

**Typische Fragen/Bedenken:**

- „DTLS-SRTP heißt, der Server sieht Audio – ist das den Nutzern klar?"
- „Wie schützen wir die One-Time-Prekeys vor Depletion-Attacken?"
- „Was passiert bei Key Compromise? Wie ist der Recovery-Prozess?"
- „Rate-Limiting auf Login ist gut, aber was ist mit WebSocket-Flooding?"
- „Der JWT ist 15 Minuten gültig – was wenn er geleakt wird?"
- „Ich würde das nicht so bauen. Aber ich werde es verteidigen, wenn ihr es tut."

**Mantra:** *„Sicherheit ist kein Feature, das man später hinzufügt."*

**Review-Fokus:**

- Authentifizierung und Autorisierung
- Input-Validierung und Injection-Prävention
- Kryptografische Implementierungen
- Rate-Limiting und DoS-Schutz
- Secrets-Management und Key-Rotation

---

## 5. Gimli – Compliance & Licensing Specialist

**Hintergrund:** Juristischer Background mit Tech-Fokus. Arbeitet seit 6 Jahren an Open-Source-Compliance. Hat schon GPL-Verstöße in Unternehmen aufgedeckt. Stur, wenn es um Regeln geht – aber loyal.

**Perspektive:** Paranoid bezüglich Lizenzen. Weiß, dass ein einziger AGPL-Import das ganze Projekt infizieren kann. Liest jeden `Cargo.toml`-Eintrag. Versteht keinen Spaß bei Lizenzfragen.

**Typische Fragen:**

- „Ist libsignal wirklich komplett raus? Auch in transitiven Dependencies?"
- „Was steht in der NOTICE-Datei von ring? Müssen wir das dokumentieren?"
- „Wenn jemand einen Fork macht und MongoDB anbindet, was passiert dann lizenzrechtlich?"
- „Haben wir cargo-deny in der CI?"
- „Das steht so im Vertrag. Und an Verträge hält man sich."

**Mantra:** *„Eine vergessene Lizenz ist eine tickende Zeitbombe."*

**Review-Fokus:**

- Neue Dependencies und deren Lizenzen
- Transitive Abhängigkeiten
- THIRD_PARTY_NOTICES.md Aktualität
- cargo-deny Konfiguration
- Attribution und Copyright-Header

---

## 6. Legolas – Quality Assurance Engineer

**Hintergrund:** 8 Jahre QA, davon 3 Jahre in Real-Time-Systemen. Hat ein Händchen dafür, Edge-Cases zu finden, an die niemand gedacht hat. Sieht Bugs, bevor sie entstehen.

**Perspektive:** Denkt in Testszenarien und User-Flows. Fragt: „Was passiert, wenn..." Interessiert sich für Reproduzierbarkeit und Testautomatisierung. Präzise und detailorientiert.

**Typische Fragen:**

- „Wie testen wir Voice-Qualität automatisiert?"
- „Was passiert, wenn ein User während des Sprechens die Verbindung verliert?"
- „Können wir E2EE-Flows testen ohne die Crypto zu mocken?"
- „Wie simulieren wir 50 gleichzeitige Voice-User?"
- „Was ist die Test-Strategie für SSO mit verschiedenen Providern?"
- „Da war etwas. Im dritten Request. Habt ihr das auch gesehen?"

**Mantra:** *„Wenn es keinen Test gibt, ist es kaputt – wir wissen es nur noch nicht."*

**Review-Fokus:**

- Test-Coverage und Test-Qualität
- Edge-Cases und Fehlerszenarien
- Integration-Tests und E2E-Tests
- Testbarkeit des Codes
- Reproduzierbarkeit von Bugs

---

## 7. Pippin – Community Manager / Early Adopter

**Hintergrund:** Enthusiastischer Gamer, moderiert mehrere Discord-Server. Kein Entwickler, aber technisch interessiert. Repräsentiert die Zielgruppe. Fragt Dinge, die Entwickler für selbstverständlich halten.

**Perspektive:** Die Stimme der Nutzer. Testet Features aus User-Sicht. Gibt ehrliches Feedback, auch wenn es wehtut. Findet UX-Probleme durch Ausprobieren. Manchmal chaotisch, aber bringt frischen Wind.

**Typische Fragen:**

- „Warum muss ich hier dreimal klicken? Bei Discord geht das mit einem."
- „Was bedeutet ‚DTLS-SRTP Handshake fehlgeschlagen'? Das sagt mir nichts."
- „Kann ich meine Freunde einladen, ohne dass sie IT studiert haben?"
- „Die Emojis sind zu klein. Das ist wichtig, glaubt mir."
- „Oh, was macht dieser Knopf?"

**Mantra:** *„Wenn ich es nicht verstehe, versteht es niemand in meiner Community."*

**Review-Fokus:**

- Fehlermeldungen und deren Verständlichkeit
- Onboarding-Flow für neue Nutzer
- Feature-Discoverability
- Vergleich mit Discord/TeamSpeak/Mumble
- Community-relevante Features (Emojis, Mentions, etc.)

---

## 8. Bilbo – Self-Hoster Enthusiast

**Hintergrund:** Technisch versiert, aber kein Entwickler. Betreibt zu Hause einen kleinen Server mit Nextcloud und Pi-hole. Will Kontrolle über seine Daten. Abenteuerlustig, aber schätzt gute Dokumentation.

**Perspektive:** Testet die Installations-Dokumentation. Repräsentiert den typischen Self-Hoster: motiviert, aber begrenzte Zeit und Geduld. Wenn Bilbo es installieren kann, kann es jeder.

**Typische Fragen:**

- „Steht irgendwo, welche Ports ich freigeben muss?"
- „Was bedeutet ‚OIDC_ISSUER_URL'? Brauche ich das?"
- „Kann ich das auch ohne Docker installieren?"
- „Was mache ich, wenn das Update schiefgeht?"
- „Das mit dem Backup – muss das sein, oder ist das optional?"
- „Ein Abenteuer! Aber bitte mit Anleitung."

**Mantra:** *„Ich will es selbst hosten, nicht selbst debuggen."*

**Review-Fokus:**

- README und Installations-Dokumentation
- docker-compose.yml Verständlichkeit
- Umgebungsvariablen und deren Dokumentation
- Troubleshooting-Guides
- Upgrade-Dokumentation

---

## 9. Gandalf – Performance Engineer

**Hintergrund:** 15 Jahre Erfahrung, hat an Low-Latency-Systemen gearbeitet (Börsenhandel, Gaming-Server). Versteht, was auf CPU-Cycle-Ebene passiert. Kommt genau dann, wenn man ihn braucht.

**Perspektive:** Fokus auf Latenz-Optimierung, Profiling, Memory-Leaks. Weiß, dass Performance-Probleme meist architektonische Ursachen haben. Misst alles, vermutet nichts.

**Typische Fragen:**

- „Warum allokieren wir hier bei jedem Frame neu?"
- „Haben wir Flame-Graphs vom Voice-Path?"
- „Was ist die P99-Latenz unter Last?"
- „Dieser Lock hier – wie lange wird der gehalten?"
- „50ms ist zu viel. 20ms ist akzeptabel. 10ms ist das Ziel."
- „Ein Performance-Problem ist nie zu spät erkannt – nur zu spät behoben."

**Mantra:** *„Premature optimization ist das Problem. Aber mature optimization ist die Lösung."*

**Review-Fokus:**

- Hot-Paths und deren Optimierung
- Allokationen und Memory-Management
- Lock-Contention und Concurrency
- Benchmarks und Performance-Tests
- Profiling-Ergebnisse und Flame-Graphs

---

## Verwendung der Personas

### In Design-Diskussionen

Bei neuen Features oder Architektur-Entscheidungen sollten folgende Fragen gestellt werden:

1. **Elrond:** Passt das in die Gesamtarchitektur?
2. **Faramir:** Welche Sicherheitsrisiken entstehen?
3. **Gimli:** Gibt es Lizenzprobleme?
4. **Gandalf:** Welche Performance-Implikationen hat das?

### In Code-Reviews

Je nach Art der Änderung sollten verschiedene Personas priorisiert werden:

| Art der Änderung | Primäre Personas |
|------------------|------------------|
| Neue Dependency | Gimli, Faramir |
| API-Änderung | Elrond, Éowyn |
| Performance-kritischer Code | Gandalf, Legolas |
| UI/UX-Änderung | Pippin, Éowyn |
| Deployment/Config | Samweis, Bilbo |
| Sicherheitsrelevant | Faramir, Legolas |

### In der Dokumentation

- **README.md:** Bilbo-Perspektive (Self-Hoster)
- **ARCHITECTURE.md:** Elrond-Perspektive (Architektur)
- **SECURITY.md:** Faramir-Perspektive (Security)
- **CONTRIBUTING.md:** Éowyn-Perspektive (Developer)

---

## Persona-Checkliste für PRs

```markdown
## Persona-Check

- [ ] **Elrond:** Architektur-Impact geprüft?
- [ ] **Éowyn:** Code lesbar und wartbar?
- [ ] **Samweis:** Deployment-Impact bedacht?
- [ ] **Faramir:** Security-Implikationen geprüft?
- [ ] **Gimli:** Neue Dependencies lizenzkonform?
- [ ] **Legolas:** Tests vorhanden und sinnvoll?
- [ ] **Pippin:** UX-Impact für Endnutzer?
- [ ] **Bilbo:** Dokumentation aktualisiert?
- [ ] **Gandalf:** Performance-kritische Pfade geprüft?
```

---

## Referenzen

- [PROJECT_SPEC.md](../project/specification.md) – Projektanforderungen
- [ARCHITECTURE.md](../architecture/overview.md) – Technische Architektur
- [STANDARDS.md](../development/standards.md) – Verwendete Standards
- [LICENSE_COMPLIANCE.md](../ops/license-compliance.md) – Lizenzprüfung
