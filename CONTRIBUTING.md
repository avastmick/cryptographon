# Contributing Guidelines

Super happy for anyone to contribute, either ideas, issues, code, copy or web design.

Usual rules apply:

- Fork the repo
- Do all your work on the `develop` branch
- Make your own changes
- Make sure all the tests pass
- Run against CI:
  - All tests pass
    - Python ``python -m pytest``
    - Rust ``cargo test --all``
  - Make sure the code meets the language formatting rules
    - For Rust, run ``cargo fmt``
  - Make sure code coverage is >90%

If the above works submit a pull request, we'll review and incorporate any changes.

*Please note that this repository is new and highly volatile right now, we will be restructuring and adding and removing code, make sure your version is synced with upstream and fully merged before submitting.*