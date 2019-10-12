# Repository Cleaner

Clean up those huge `target`s and `node_module`s etc. in your old forgotten repositories.

## Usage example

```
cargo run -- ~
```

## How it works

1. It looks for all your Git repositories under the specified path.
2. If it finds a `.gitignore` file at the root of the repository, it finds all the ignored resources in the repository.
3. You can see all the repositories sorted by size and which of their resources are being ignored. These very often include huge stuff like `node_module` directories.
4. You can delete unneeded resources.

## TODOs

* Refactor UI
* Improve UI
  * Show a simple tree in the detail view
  * Show a progress bar while deleting
