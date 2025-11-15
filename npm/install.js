#!/usr/bin/env node

/**
 * Install script for just-mcp npm package
 * Downloads the appropriate pre-built binary for the current platform
 */

const { spawnSync } = require('child_process');
const fs = require('fs');
const https = require('https');
const path = require('path');
const { pipeline } = require('stream');
const { promisify } = require('util');
const zlib = require('zlib');

const streamPipeline = promisify(pipeline);

// Determine platform and architecture
const platform = process.platform;
const arch = process.arch;

// Map Node.js platform/arch to Rust target triples
const PLATFORM_MAP = {
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

const platformKey = `${platform}-${arch}`;
const target = PLATFORM_MAP[platformKey];

if (!target) {
  console.error(`Unsupported platform: ${platformKey}`);
  console.error(`Supported platforms: ${Object.keys(PLATFORM_MAP).join(', ')}`);
  process.exit(1);
}

// Get package version
const packageJson = require('./package.json');
const version = packageJson.version;

// Construct download URL
const baseUrl = `https://github.com/promptexecution/just-mcp/releases/download/v${version}`;
const binaryName = platform === 'win32' ? 'just-mcp.exe' : 'just-mcp';
const archiveName = `just-mcp-${target}.tar.gz`;
const downloadUrl = `${baseUrl}/${archiveName}`;

console.log(`Installing just-mcp v${version} for ${platformKey}...`);
console.log(`Download URL: ${downloadUrl}`);

// Create bin directory if it doesn't exist
const binDir = path.join(__dirname, 'bin');
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

const binaryPath = path.join(binDir, binaryName);

/**
 * Download and extract the binary
 */
async function download() {
  return new Promise((resolve, reject) => {
    https.get(downloadUrl, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        https.get(response.headers.location, (redirectResponse) => {
          if (redirectResponse.statusCode !== 200) {
            reject(new Error(`Download failed with status ${redirectResponse.statusCode}`));
            return;
          }
          extractArchive(redirectResponse, resolve, reject);
        });
      } else if (response.statusCode === 200) {
        extractArchive(response, resolve, reject);
      } else {
        reject(new Error(`Download failed with status ${response.statusCode}`));
      }
    }).on('error', reject);
  });
}

function extractArchive(response, resolve, reject) {
  const gunzip = zlib.createGunzip();
  const chunks = [];

  response.pipe(gunzip);

  gunzip.on('data', (chunk) => {
    chunks.push(chunk);
  });

  gunzip.on('end', () => {
    try {
      const buffer = Buffer.concat(chunks);
      // Simple tar extraction for single file
      // TAR header is 512 bytes, then file content
      // We need to find the binary in the tar archive
      
      // For simplicity, try to extract using tar command if available
      const tarPath = path.join(binDir, 'archive.tar');
      fs.writeFileSync(tarPath, buffer);
      
      // Try to use tar command
      const result = spawnSync('tar', ['-xf', tarPath, '-C', binDir], {
        stdio: 'inherit',
      });
      
      if (result.status === 0) {
        fs.unlinkSync(tarPath);
        fs.chmodSync(binaryPath, 0o755);
        console.log(`Successfully installed just-mcp to ${binaryPath}`);
        resolve();
      } else {
        reject(new Error('Failed to extract archive'));
      }
    } catch (error) {
      reject(error);
    }
  });

  gunzip.on('error', reject);
}

/**
 * Fallback: try to build from source using cargo
 */
function buildFromSource() {
  console.log('Attempting to build from source using cargo...');
  const result = spawnSync('cargo', ['install', '--root', binDir, 'just-mcp'], {
    stdio: 'inherit',
  });

  if (result.status === 0) {
    console.log('Successfully built just-mcp from source');
  } else {
    console.error('Failed to build from source');
    console.error('Please install Rust and cargo, or download the binary manually from:');
    console.error(`https://github.com/promptexecution/just-mcp/releases/tag/v${version}`);
    process.exit(1);
  }
}

// Main installation logic
download()
  .then(() => {
    console.log('Installation complete!');
  })
  .catch((error) => {
    console.warn(`Failed to download binary: ${error.message}`);
    console.warn('Attempting fallback to cargo build...');
    buildFromSource();
  });
