# 📦 Changelog

All notable changes to this project will be documented in this file.  
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)  
This project follows [Semantic Versioning](https://semver.org/).

---

## [0.2.0] - Future?

> ⚠️ This section includes sarcasm and dry humor for entertainment only.  
> It does **not** reflect any negative sentiment toward contributors, users, or features.  
> We respect and uphold our [Code of Conduct](CODE_OF_CONDUCT.md) — this is just a friendly crab trying to keep things fun. 🦀💙

### ✨ Added (Supposedly™)

- A `/define` command because clearly no one knows what words mean anymore
- `/translate` — because we *definitely* plan to go multilingual before we even finish English
- `/todo`, for the brave souls who think a bot can help them get their life together
- `/remind`, perfect for forgetting you ever set a reminder
- `/quote`, because who doesn’t need algorithmically sourced inspiration?
- Admin moderation tools so we can pretend someone is actually in charge
- Command cooldowns — because spamming `/joke` 100 times *isn’t* peak comedy
- Unit & integration tests — or as we call them, “faith boosters”
- GitHub Actions for CI, since pushing to `main` with no tests was starting to feel too powerful

### 🛠 Changed (Allegedly)

- Made `/weather` less ugly. Now with more emoji and fewer regrets.
- Refactored API handling to avoid *yet another* copy-pasted fetch block
- Started i18n/L10n because someday this bot will be famous in at least three languages

### 🐞 Fixed (Maybe?)

- Now complains politely when you forget your Telegram token
- Switched to a public exchange API because who wants to log in just to check if the dollar is still sad

> Just for fun: If this changelog section offends you, we owe you a cookie and a sincere apology. We ❤️ our community and this is all in good humor.

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