# 📦 Changelog

All notable changes to this project will be documented in this file.  
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)  
This project follows [Semantic Versioning](https://semver.org/).

---

## [0.1.0] - 2025-04-14
### ✨ Added

- Modular command framework with `/help`, `/joke`, `/ping`, `/roll`, `/weather`, `/currency`, etc.
- Structured logging for all command usage, including user info and outcome
- `.env` environment variable support with `.env` loading via `dotenv`
- Initial `env.example` template for required config values
- Basic logging using `log` + `pretty_env_logger`
- Clean project structure with clearly separated command modules
- User feedback and fallback handling for unknown commands
- Refactored `dispatch_command` to use centralized logging and command matching
- README documentation for features, setup, supported commands, and project structure

### 🛠 Changed

- 

### 🐞 Fixed

- 

---