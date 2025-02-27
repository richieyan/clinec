# Java Spring Boot Project Configuration

## Project Structure
```
project_name/
├── cline_docs/
│   ├── .clinerules
│   └── projectBrief.md
├── src/
│   └── main/
│       ├── java/
│       │   └── com/
│       │       └── example/
│       │           └── Application.java
│       └── resources/
│           ├── application.properties
│           ├── static/
│           └── templates/
├── test/
│   └── java/
│       └── com/
│           └── example/
├── pom.xml
└── README.md
```

## Java Version
Java 11+

## Spring Boot Version
Spring Boot 2.7+

## Dependencies
- Spring Boot Starter Web
- Spring Boot Starter Data JPA
- Spring Boot Starter Security
- Spring Boot DevTools
- Lombok
- Spring Boot Starter Test
- Spring Boot Starter Actuator

## Build & Run
```bash
# Build project
mvn clean package

# Run application
mvn spring-boot:run

# Run tests
mvn test
```

## Common Commands
```bash
# Generate a new Spring Boot project
mvn archetype:generate -DgroupId=com.example -DartifactId=myapp -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

# Package application
mvn clean package

# Run application with profile
mvn spring-boot:run -Dspring.profiles.active=dev
```

## IDE Configuration
- Use IntelliJ IDEA with Spring Boot plugin
- Enable Lombok plugin
- Configure Spring Boot DevTools for hot reload
