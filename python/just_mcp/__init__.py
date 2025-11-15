"""
just-mcp: Production-ready MCP server for Justfile integration

This package provides a Python wrapper for the just-mcp binary,
which is a Rust-based MCP (Model Context Protocol) server.
"""

__version__ = "0.1.0"

import os
import platform
import subprocess
import sys
import tarfile
import urllib.request
from pathlib import Path


def get_binary_name():
    """Get the appropriate binary name for the current platform."""
    if platform.system() == "Windows":
        return "just-mcp.exe"
    return "just-mcp"


def get_target_triple():
    """Map Python platform to Rust target triple."""
    system = platform.system()
    machine = platform.machine()
    
    if system == "Linux":
        if machine in ("x86_64", "AMD64"):
            return "x86_64-unknown-linux-gnu"
        elif machine in ("aarch64", "arm64"):
            return "aarch64-unknown-linux-gnu"
    elif system == "Darwin":
        if machine == "x86_64":
            return "x86_64-apple-darwin"
        elif machine == "arm64":
            return "aarch64-apple-darwin"
    elif system == "Windows":
        if machine in ("x86_64", "AMD64"):
            return "x86_64-pc-windows-msvc"
    
    raise RuntimeError(f"Unsupported platform: {system} {machine}")


def get_binary_path():
    """Get the path to the just-mcp binary."""
    package_dir = Path(__file__).parent
    bin_dir = package_dir / "bin"
    binary_name = get_binary_name()
    return bin_dir / binary_name


def download_binary(version=None):
    """Download the just-mcp binary for the current platform."""
    if version is None:
        version = __version__
    
    target = get_target_triple()
    binary_name = get_binary_name()
    
    # Construct download URL
    base_url = f"https://github.com/promptexecution/just-mcp/releases/download/v{version}"
    archive_name = f"just-mcp-{target}.tar.gz"
    download_url = f"{base_url}/{archive_name}"
    
    print(f"Installing just-mcp v{version} for {platform.system()} {platform.machine()}...")
    print(f"Downloading from: {download_url}")
    
    # Create bin directory
    package_dir = Path(__file__).parent
    bin_dir = package_dir / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    
    binary_path = bin_dir / binary_name
    
    # Download and extract
    try:
        archive_path = bin_dir / archive_name
        urllib.request.urlretrieve(download_url, archive_path)
        
        # Extract tar.gz
        with tarfile.open(archive_path, "r:gz") as tar:
            tar.extractall(path=bin_dir)
        
        # Remove archive
        archive_path.unlink()
        
        # Make executable (Unix-like systems)
        if platform.system() != "Windows":
            os.chmod(binary_path, 0o755)
        
        print(f"Successfully installed just-mcp to {binary_path}")
        return binary_path
        
    except Exception as e:
        print(f"Failed to download binary: {e}", file=sys.stderr)
        print("You can try building from source with: cargo install just-mcp", file=sys.stderr)
        raise


def ensure_binary():
    """Ensure the binary is available, downloading if necessary."""
    binary_path = get_binary_path()
    
    if not binary_path.exists():
        print("just-mcp binary not found, downloading...")
        download_binary()
    
    return binary_path


def main():
    """Main entry point for the just-mcp command."""
    try:
        binary_path = ensure_binary()
        
        # Execute the binary with all command line arguments
        result = subprocess.run(
            [str(binary_path)] + sys.argv[1:],
            check=False
        )
        sys.exit(result.returncode)
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
