# CHIP-8 interpreter

This is a [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter, written in Go. Since I'm very interested in old hardware and how it works, I created this project to get a little bit familiar with emulator programming.

![Screenshot](screenshot.png)

## Usage

### Key mapping

The original CHIP-8 keypad looks like:

<kbd>1</kbd> <kbd>2</kbd> <kbd>3</kbd> <kbd>C</kbd><br/>
<kbd>4</kbd> <kbd>5</kbd> <kbd>6</kbd> <kbd>D</kbd><br/>
<kbd>7</kbd> <kbd>8</kbd> <kbd>9</kbd> <kbd>E</kbd><br/>
<kbd>A</kbd> <kbd>0</kbd> <kbd>B</kbd> <kbd>F</kbd>

The keys are mapped as following:

<kbd>1</kbd> <kbd>2</kbd> <kbd>3</kbd> <kbd>4</kbd><br/>
<kbd>Q</kbd> <kbd>W</kbd> <kbd>E</kbd> <kbd>R</kbd><br/>
<kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> <kbd>F</kbd><br/>
<kbd>Z</kbd> <kbd>X</kbd> <kbd>C</kbd> <kbd>V</kbd>

For German users the <kbd>A</kbd> is also mapped to <kbd>Y</kbd>.

### Special keys

Besides that there are special operations which can be triggered with the following keys:

| Key          | Function              |
| ------------ | --------------------- |
| <kbd>I</kbd> | Reset the interpreter |
| <kbd>P</kbd> | Pause the interpreter |

### Command line arguments

| Argument      | Function                                  |
| ------------- | ----------------------------------------- |
| `-help`       | Show all available command line arguments |
| `-rom [file]` | Specify the path to a rom file            |
