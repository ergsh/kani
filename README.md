![Kani Metadata Image](https://github.com/kquirapas/kanban-as-code/blob/main/.github/img/kani%20metadata.png)

# Kani 🦀🦀🦀
 A Rust-powered 🦀 Kanban-as-Code TUI binary.

# Kanban-as-Code
Track your repositories progress with a repo-embedded Kanban!

# Problem Statement
If you're like me, you probably multiplex across different threads of work.

It's hard to track the progress of what you're currently working on because it's either your ticketing is in another platform or scattered in different tooling when working with different teams.

How do we foster project alignment with Kanban while also having a single source of truth across different projects?

A Kanban-as-Code

# Solution
Using a TUI binary built with Rust. A terminal-based Kanban board can be invoked which is locally stored and saved within your repository.

# Roadmap
- CLI tool
    - New
    - Config
- Initial Kanban board TUI
- Persistence with `.kani`
- TOML Configuration
- Github-linked card/ticket assignment

# Thoughts
Consider using own filetype, serialization, and conversion instead of `.TOML` because using a singular `.TOML` config is UNREADABLE and hard to fix merge conflicts with should they come.
