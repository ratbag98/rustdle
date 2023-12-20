# Rustdle

## Description

After learning Python earlier this year with the PySquaredle program, I've now moved on to Rust. And since I have very little imagination, I'm going to implement another Squaredle solver.

The solver will provide text and GUI interfaces.


CURRENTLY THE PROJECT JUST PARSES COMMAND-LINE ARGUMENT. 

## Table of Contents (Optional)

If your README is long, add a table of contents to make it easy for users to find what they need.

- [Installation](#installation)
- [Usage](#usage)
- [Credits](#credits)
- [License](#license)

## Installation

What are the steps required to install your project? Provide a step-by-step description of how to get the development environment running.

## Usage

Provide instructions and examples for use. Include screenshots as needed.

To add a screenshot, create an `assets/images` folder in your repository and upload your screenshot to it. Then, using the relative filepath, add it to your README using the following syntax:

    ```md
    ![alt text](assets/images/screenshot.png)
    ```

## Credits

List your collaborators, if any, with links to their GitHub profiles.

If you used any third-party assets that require attribution, list the creators with links to their primary web presence in this section.

If you followed tutorials, include links to those here as well.

## License

The last section of a high-quality README file is the license. This lets other developers know what they can and cannot do with your project. If you need help choosing a license, refer to [https://choosealicense.com/](https://choosealicense.com/).

---

🏆 The previous sections are the bare minimum, and your project will ultimately determine the content of this document. You might also want to consider adding the following sections.

## Badges

![rustdle](https://img.shields.io/github/languages/top/ratbag98/rustdle)

Badges aren't necessary, per se, but they demonstrate street cred. Badges let other developers know that you know what you're doing. Check out the badges hosted by [shields.io](https://shields.io/). You may not understand what they all represent now, but you will in time.

## Features

Current help looks like this and I intend to implement all the features described:

```text
Rustdle uses the power of Rust to solve Squaredles

Usage: rustdle [OPTIONS] [LETTERS]

Arguments:
  [LETTERS]  User-specified grid of letters.

Options:
  -x, --square <X>     Create a random square grid of X by X letters
  -e, --express        download express rather than standard puzzle
  -c, --single-column  display results as a single column
  -g, --grid           display letters grid
  -l, --length         group solutions by length
  -H, --headers        display headers for length-grouped solution lists
  -u, --gui            run in GUI mode
  -m, --multiple       show all solutions for a word in GUI
  -r, --random         randomise letter order, maybe useful for setting puzzles
  -s, --sort           sort solutions alphabetically
  -t, --auto-extend    add extra letters to LETTERS to ensure puzzle is square
  -d, --debug          debug mode, mainly shows neighbour list
  -f, --file <FILE>    specify custom word list
  -z, --slow-mode      show progress of trie walk
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

## Tests

Go the extra mile and write tests for your application. Then provide examples on how to run them here.
