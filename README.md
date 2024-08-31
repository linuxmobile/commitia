<img src="/assets/commitia.png">

# Commitia

> Commitiá is a commit message generator inspired by Argentine slang. In Argentina, we often say "comentá" with an accent on the final 'a' to mean "comment".
> Similarly, we use "commitiá" to refer to the action of committing changes to the GitHub stage.
> The name combines the English word "commit" with the Argentine way of adding emphasis, making it a playful and culturally unique term for the tool.

## Features:

- **Commit Message Generation**
- **Multi-language Support**
- **Git Integration**
- **First Launch Setup**
- **Google AI Integration**: Uses Google AI for generating commit messages.
- **File Selection**: Allows you to manually select files to commit.
- **Secure Token Storage**: Encrypts and stores your Google AI token securely.

## Installation

To get started with Commitia, follow these steps:

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/yourusername/commitia.git
    cd commitia
    ```

2. **Install Rust**:
    Ensure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).

3. **Build the Project**:
    ```sh
    cargo build --release
    ```

4. **Run the Application**:
    ```sh
    cargo run --release
    ```

<details> <summary><h2>How It Works</h2></summary>

Commitia is a command-line interface (CLI) tool built in Rust. Here's a summary of how it works:

1. **Initial Setup**:
    - When you first run Commitia, it will display a splash screen and prompt you to enter your Google AI token.
    - The token is securely encrypted and stored in a configuration file.

2. **Commit Message Generation**:
    - After setting up the token, Commitia will ask if you want to select files to commit.
    - If you choose to select files, it will display a list of staged files for you to choose from.
    - Once the files are selected, Commitia will generate a commit message using Google AI and Argentine slang.

3. **User Interface**:
    - The UI is built using the `ratatui` library, providing a smooth and interactive experience.
    - It includes features like text input, file selection, and progress gauges.

## Usage

To use Commitia, simply run the following command in your terminal:

```sh
cargo run --release
```

Follow the on-screen instructions to enter your Google AI token and select files for committing.

</details>

## Contribution Guidelines

We welcome contributions to Commitia! Here are some ways you can help:

### To-Do List

- [ ] **Improve Welcome & Banner Section**: Enhance the welcome message and banner for a better first impression.
- [ ] **Work in 'Select Files'**: Implement the file selection feature for committing specific files.
- [ ] **Generate AI Commit Message**: Implement the Google AI integration for generating commit messages.
- [ ] **Enhance UI**: Improve the user interface for better user experience.
- [ ] **Add Unit Tests**: Increase test coverage for the existing codebase.
- [ ] **Optimize Performance**: Profile and optimize the performance of the application.
- [ ] **Bug Fixes**: Identify and fix bugs in the current implementation.
- [ ] **Improve Documentation**: Enhance the README.md and add more detailed documentation.
- [ ] **Feature Requests**: Suggest and implement new features.

### How to Contribute

1. **Fork the Repository**:
    - Click the "Fork" button at the top right of this page to create a copy of the repository in your GitHub account.

2. **Clone Your Fork**:
    ```sh
    git clone https://github.com/yourusername/commitia.git
    cd commitia
    ```

3. **Create a Branch**:
    ```sh
    git checkout -b feature/your-feature-name
    ```

4. **Make Your Changes**:
    - Implement your changes and commit them with clear and concise commit messages.

5. **Push to Your Fork**:
    ```sh
    git push origin feature/your-feature-name
    ```

6. **Create a Pull Request**:
    - Go to the original repository and click on the "New Pull Request" button.
    - Provide a detailed description of your changes and submit the pull request.

## License

Commitia is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any questions or feedback, feel free to reach out to the author at [linuxmobile](mailto:bdiez19@gmail.com).

---

Thank you for using Commitia! We hope it makes your commit messages more enjoyable and engaging.
