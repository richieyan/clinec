# Go Project Rules

## Code Style
- Use gofmt for formatting
- Follow Effective Go guidelines
- Use camelCase for variables and functions
- Use PascalCase for exported names

## Testing
- Write tests using testing package
- Use table-driven tests where appropriate
- Maintain 80%+ test coverage

## Documentation
- Add GoDoc comments for all exported functions
- Include package-level documentation
- Use examples in documentation

## Dependencies
- Use Go modules for dependency management
- Keep dependencies to a minimum
- Use go mod tidy to clean up unused dependencies

## Error Handling
- Return errors rather than panic
- Wrap errors with context
- Use errors.Is and errors.As for error checking
