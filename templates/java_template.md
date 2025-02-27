# Java Project Configuration

## Project Structure
```
project_name/
├── cline_docs/
│   ├── .clinerules
│   └── projectBrief.md
├── src/
│   └── main/
│       └── java/
│           └── Main.java
├── test/
│   └── java/
├── pom.xml
└── README.md
```

## Java Version
Java 11+

## Dependencies
- JUnit 5
- Mockito
- Checkstyle
- SpotBugs

## Build & Test
```bash
# Build project
mvn clean package

# Run tests
mvn test

# Check style
mvn checkstyle:check

# Static analysis
mvn spotbugs:check
```

## IDE Configuration
- Use IntelliJ IDEA or Eclipse
- Enable Checkstyle plugin
- Enable SpotBugs plugin
