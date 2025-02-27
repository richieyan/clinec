# Python Project Configuration

## Project Structure
```
project_name/
├── cline_docs/
│   ├── .clinerules
│   └── projectBrief.md
├── src/
│   └── main.py
├── tests/
├── requirements.txt
└── README.md
```

## Python Version
Python 3.8+

## Dependencies
- pytest
- black
- flake8
- mypy
- pylint
- isort

## Virtual Environment
```bash
# Create virtual environment
python -m venv venv

# Activate virtual environment
# On macOS/Linux:
source venv/bin/activate
# On Windows:
.\venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt
```

## Linting & Formatting
```bash
# Format code
black src/

# Lint code
flake8 src/

# Type checking
mypy src/

# Sort imports
isort src/
```

## Testing
```bash
# Run tests
pytest tests/

# Generate coverage report
pytest --cov=src tests/
```

## CI/CD Integration
- Add .github/workflows/ci.yml for GitHub Actions
- Configure pre-commit hooks
