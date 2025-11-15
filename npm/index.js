#!/usr/bin/env node

/**
 * Wrapper script to execute the just-mcp binary
 */

const { spawnSync } = require('child_process');
const path = require('path');

const platform = process.platform;
const binaryName = platform === 'win32' ? 'just-mcp.exe' : 'just-mcp';
const binaryPath = path.join(__dirname, 'bin', binaryName);

// Execute the binary with all command line arguments
const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

process.exit(result.status || 0);
