use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Registry of justfile paths permitted for introspection and execution.
///
/// The registry implements the sandbox gate: only registered justfiles
/// are visible to just-mcp. Unregistered justfiles don't produce an
/// "access denied" error — they simply don't exist from the agent's view.
///
/// Two modes:
/// - **Permissive** (empty registry): all justfiles allowed — backward compatible.
/// - **Strict** (non-empty registry): only registered absolute paths allowed.
#[derive(Debug, Clone, Default)]
pub struct JustfileRegistry {
    /// Canonicalized absolute paths of registered justfiles.
    /// Empty = permissive mode (any justfile allowed).
    allowed: HashSet<PathBuf>,
}

impl JustfileRegistry {
    /// Create a permissive registry (no restrictions).
    pub fn permissive() -> Self {
        Self::default()
    }

    /// Create a strict registry from a list of allowed paths.
    /// Paths are canonicalized; non-existent paths are silently dropped.
    pub fn from_paths(paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        let allowed = paths
            .into_iter()
            .filter_map(|p| p.as_ref().canonicalize().ok())
            .collect();
        Self { allowed }
    }

    /// Register a single path. Non-existent paths are silently dropped.
    pub fn register(&mut self, path: impl AsRef<Path>) {
        if let Ok(canonical) = path.as_ref().canonicalize() {
            self.allowed.insert(canonical);
        }
    }

    /// Check if a justfile path is in scope for this registry.
    ///
    /// Returns `true` in permissive mode (empty registry) or when the
    /// canonicalized path is registered.
    pub fn is_in_scope(&self, path: impl AsRef<Path>) -> bool {
        if self.is_permissive() {
            return true;
        }
        path.as_ref()
            .canonicalize()
            .map(|canonical| self.allowed.contains(&canonical))
            .unwrap_or(false)
    }

    /// True when the registry is in permissive mode (no restrictions).
    pub fn is_permissive(&self) -> bool {
        self.allowed.is_empty()
    }

    /// Number of registered justfiles (0 = permissive).
    pub fn len(&self) -> usize {
        self.allowed.len()
    }

    /// Iterate registered paths.
    pub fn registered_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.allowed.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn permissive_allows_any_path() {
        let registry = JustfileRegistry::permissive();
        assert!(registry.is_permissive());
        // Permissive allows even non-existent paths
        assert!(registry.is_in_scope("/any/path/justfile"));
    }

    #[test]
    fn strict_registry_gates_correctly() {
        let dir = tempdir().unwrap();
        let allowed = dir.path().join("justfile");
        let denied = dir.path().join("other/justfile");
        fs::write(&allowed, "default:\n    echo hi").unwrap();

        let registry = JustfileRegistry::from_paths([&allowed]);
        assert!(!registry.is_permissive());
        assert!(registry.is_in_scope(&allowed));
        assert!(!registry.is_in_scope(&denied));
    }

    #[test]
    fn nonexistent_path_excluded_from_registry() {
        let registry = JustfileRegistry::from_paths(["/does/not/exist/justfile"]);
        // Non-existent path silently dropped → permissive mode
        assert!(registry.is_permissive());
    }
}
