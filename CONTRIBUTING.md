# Contributing to Titanio

Hi there! 👋 We're happy you're interested in contributing to Titanio, a fun, modular Telegram bot written in Rust. This guide will help you get started.

## Code of Conduct

Please note that Titanio has a ![Code of Conduct](CODE_OF_CONDUCT.md). We expect all contributors to adhere to it in all project interactions.

## How Can I Contribute?

There are many ways to contribute to Titanio:

* Reporting bugs
* Suggesting new features
* Improving documentation
* Submitting code changes

## Reporting Bugs

If you find a bug, please:

1.  Check the [issue tracker](https://github.com/LoboGuardian/titanio-rust-telegram-bot/issues) to see if it has already been reported.
2.  If not, create a new issue with a clear description of the bug.

Include the following information in your bug report:

* Operating system
* Rust version
* Steps to reproduce the bug
* Expected behavior
* Actual behavior
* Any error messages

## Suggesting Features

If you have an idea for a new feature, please:

1.  Check the [issue tracker](https://github.com/LoboGuardian/titanio-rust-telegram-bot/issues) to see if it has already been suggested.
2.  If not, create a new issue with a clear description of the feature.

Explain your use case and how the feature would benefit Titanio.

## Contributing Code

Here's how you can contribute code to Titanio:

1.  **Fork the repository:** Fork the Titanio repository to your GitHub account.
2.  **Clone the repository:** Clone your fork to your local machine:

    ```
    git clone [https://github.com/](https://github.com/<your-username>/titanio-rust-telegram-bot.git
    ```
3.  **Create a branch:** Create a new branch for your changes:

    ```
    git checkout -b feature/my-new-feature
    ```

    or

    ```
    git checkout -b fix/my-bug-fix
    ```
4.  **Make changes:** Implement your changes, following the project's code style.
5.  **Test your changes:** Ensure your changes work as expected and add tests if necessary. You can run the bot locally using:

    ```
    cargo run
    ```
6.  **Format your code:** Format your code using `cargo fmt`:

    ```
    cargo fmt
    ```
7.  **Run clippy:** Check your code for any linting errors using `cargo clippy`:

    ```
    cargo clippy
    ```
8.  **Commit your changes:** Commit your changes with a clear and concise commit message.
9.  **Push your changes:** Push your changes to your fork:

    ```
    git push origin feature/my-new-feature
    ```
10. **Create a pull request:** Submit a pull request to the `main` branch of the Titanio repository.

    * Provide a clear description of your changes in the pull request.
    * Reference any related issues.

## Adding a New Command

Titanio has a modular command structure, making it easy to add new commands. Here's a basic outline:

1.  Create a new file in the `src/commands/` directory (e.g., `src/commands/my_command.rs`).
2.  Implement your command's logic in the new file. This will likely involve using `teloxide` and potentially other crates (like `reqwest` for external API calls).
3.  Add a handler function, similar to the other commands.
4.  Import your new command module in `src/commands/mod.rs` and add it to the `match` statement in the `dispatch_command` function.
5.  Add your command to the `Command` enum in `src/main.rs`
6.  Update the `/help` command in `src/commands/help.rs` to include your new command.
7.  Document your command in the `README.md`.

## Documentation

* Keep the documentation up-to-date.
* If you add a new feature or command, update the `README.md` file.
* Clear and concise documentation is highly valued.

## Getting Help

If you have any questions or need help, feel free to:

* Open an issue in the [issue tracker](https://github.com/LoboGuardian/titanio-rust-telegram-bot/issues)
* Start a discussion.

Thank you for contributing to Titanio! 🦀❤️
