# inputspy

[![npm version](https://img.shields.io/npm/v/inputspy.svg)](https://www.npmjs.com/package/inputspy)
[![license](https://img.shields.io/npm/l/inputspy.svg)](https://github.com/yourusername/inputspy/blob/main/LICENSE)

A Node.js native addon for global key listening, built with Rust and napi-rs.

## Description

inputspy is a native Node.js addon that uses Rust and the rdev library to capture system-wide keyboard events. It allows real-time monitoring of key presses and can execute callbacks when key events are detected.

## Features

- Global key listening capability
- Real-time event detection
- Cross-platform support (Windows, macOS, Linux)
- High performance through Rust native implementation
- Easy integration with Node.js applications

## Installation

```bash
npm install inputspy
```

## Usage

```javascript
const inputspy = require('inputspy');

// Start monitoring keys
inputspy.startinputspy((key) => {
  console.log('Key pressed:', key);
});
```

## API

### `startinputspy(callback)`

Starts monitoring system-wide key presses.

- `callback`: Function that executes when a key is pressed. Receives the name of the pressed key as a parameter.

## Example

```javascript
const inputspy = require('inputspy');

console.log('Starting key monitoring...');

inputspy.startinputspy((key) => {
  console.log('Key detected:', key);
  
  // Exit if ESC is pressed
  if (key === 'Escape') {
    console.log('Exiting...');
    process.exit(0);
  }
});
```

## Notes

- This library requires special permissions to access system keyboard events.
- On some systems, you may need to run the script with elevated privileges.
- The library works on Windows, macOS, and Linux.

## Requirements

- Node.js version 10 or higher
- npm or yarn package manager

## Building from Source

If you want to build the package from source:

```bash
# Clone the repository
git clone https://github.com/prahzera/inputspy.git
cd inputspy

# Install dependencies
npm install

# Build the native addon
npm run build
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Author

Prahzera