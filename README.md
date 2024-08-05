<img src="https://i.ibb.co/X7LR5J7/banner2.png">

# Commitia

> Commitiá is a commit message generator inspired by Argentine slang. In Argentina, we often say "comentá" with an accent on the final 'a' to mean "comment".
> Similarly, we use "commitiá" to refer to the action of committing changes to the GitHub stage.
> The name combines the English word "commit" with the Argentine way of adding emphasis, making it a playful and culturally unique term for the tool.

## Features:

- **Commit Message Generation**
- **Multi-language Support**
- **Git Integration**
- **First Launch Setup**
- **File Selection**

## Installation

To install Commitia, run the following script:

```bash
./install.sh
```

By default, this script will install Commitia in the `~/.local/bin` directory. If you prefer to install it in a different directory, you can specify the directory using the `--dir=DIR` argument:

### Prerequisites

Make sure you have Bun installed on your system. You can install Bun by following the instructions on their official website.

### Running Commitiá

To run Commitia, use the following command:

```bash
commitia ...
```

If you want to be able to execute Commitia from any directory, you need to add `~/.local/bin` to your **shell's PATH**. You can do this by adding the following line to your `~/.bashrc` (for Bash) or `~/.zshrc` (for Zsh) file:

```bash
export PATH="$HOME/.local/bin:$PATH"

# Next to it:
source ~/.bashrc # for Bash
source ~/.zshrc  # for zsh
```

<details> <summary><h2>How It Works</h2></summary>

Commitia is a Command Line Interface (CLI) built with **Bun**, **JavaScript**, and **TypeScript**. Below is an explanation of how the main components of the project work:

### commitia.js

This file is the entry point of the CLI. It uses various libraries to handle user interactions and Git operations. The main flow of the program is as follows:

- **Git Repository Verification**: Checks if the current directory is a Git repository.
- **File Selection**: Allows the user to select the files they want to commit.
- **Commit Message Generation**: Uses the Vercel SDK to generate a commit message based on the changes made.
- **Confirmation and Commit**: Requests user confirmation before making the commit.

### gitStageManager.ts

This file contains functions to handle Git operations, such as:

- **checkIfGitRepo**: Verifies if the current directory is a Git repository.
- **addStagedFiles**: Adds files to the Git stage.
- **getDiffSummary**: Gets a summary of the differences between the staged files.
- **commitStagedFiles**: Commits the staged files.

### PROMPT_GENERATOR.ts

This file uses the Vercel SDK to generate commit messages. The main function is `generatePrompt`, which takes the added and deleted changes and generates a commit message using an AI model.

</details>

## Why Bun, JavaScript, and TypeScript?

I chose Bun because it is a fast, all-in-one runtime for JavaScript and TypeScript. This makes it easier to create efficient and fast CLI applications. JavaScript and TypeScript are versatile and widely-used languages, making the project accessible to a broad audience of developers.

## Contributions

Contributions are welcome. If you have any suggestions or find any issues, **please open an issue or submit a pull request**.

## License

This project is licensed under the MIT License.

_This project was created using `bun init` in bun v1.1.20. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime._
