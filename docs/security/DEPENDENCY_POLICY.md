# Q1 Dependency Policy

Version: 0.1.0
Status: M0 Draft

## Principles

- minimize dependencies, especially in the deterministic protocol core;
- use maintained, reviewed libraries for cryptography;
- never implement custom production cryptography;
- pin resolved versions in committed lockfiles;
- review licenses, maintainers, update history, transitive dependencies, unsafe
  or FFI/cgo use, build scripts, and security advisories;
- separate optional experimental dependencies from consensus-critical code;
- dependency updates require tests and review, not automatic trust.

## Required dependency record

- name and version;
- purpose and importing module;
- direct/transitive status;
- source and integrity mechanism;
- license;
- security/maintenance assessment;
- unsafe/native/build-script behavior;
- approved reviewer and date;
- replacement/removal plan where risk is material.

## Prohibited behavior

- unpinned dependencies in release artifacts;
- packages fetched from undocumented sources;
- secrets in package-manager configuration;
- automatic dependency changes that bypass review;
- AI-generated package substitutions without verification.

## Open decisions

- primary language and package manager;
- audit/SBOM tooling;
- approval threshold;
- update cadence;
- vulnerability exception process;
- vendoring policy.
