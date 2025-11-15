#!/bin/bash
# Script to help publish npm and pip packages after a release
# Usage: ./scripts/publish-packages.sh <version>

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

# Remove 'v' prefix if present
VERSION=${VERSION#v}

echo "Publishing packages for version $VERSION"

# Update npm package version
echo "Updating npm package version..."
cd npm
npm version $VERSION --no-git-tag-version
cd ..

# Update Python package version
echo "Updating Python package version..."
sed -i.bak "s/^version = .*/version = \"$VERSION\"/" python/pyproject.toml
sed -i.bak "s/__version__ = .*/__version__ = \"$VERSION\"/" python/just_mcp/__init__.py
rm python/pyproject.toml.bak python/just_mcp/__init__.py.bak

echo ""
echo "Package versions updated to $VERSION"
echo ""
echo "Next steps:"
echo "1. Test the packages locally:"
echo "   cd npm && npm pack && npm install -g just-mcp-$VERSION.tgz"
echo "   cd python && pip install build && python -m build && pip install dist/just_mcp-$VERSION-py3-none-any.whl"
echo ""
echo "2. Publish to npm:"
echo "   cd npm && npm publish"
echo ""
echo "3. Publish to PyPI:"
echo "   cd python && pip install twine && python -m twine upload dist/*"
echo ""
echo "4. Commit the version changes:"
echo "   git add npm/package.json python/pyproject.toml python/just_mcp/__init__.py"
echo "   git commit -m \"chore: update npm and pip package versions to $VERSION\""
echo "   git push"
