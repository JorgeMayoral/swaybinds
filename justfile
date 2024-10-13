_default:
    @just --list

# Runs the CLI with the provided arguments
run:
    cargo run -- $(ARGS)

# Bump version
[confirm("Are you sure you want to bump the version?")]
bump VERSION:
    git add .
    git commit -m "release: version {{VERSION}}"
    git push
    git tag v{{VERSION}}
    git push --tags

# Generate changelog for next version number
changelog:
    git cliff --bump -o CHANGELOG.md

# Get next version
next-version:
    @git cliff --bumped-version

# Generate a new release
release:
    @just changelog
    @just next-version | xargs just bump
