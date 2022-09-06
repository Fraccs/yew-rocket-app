# yew-rocket-app

> Example of a Rust SPA made with the yew-rocket stack.

## Stack

> *Some elements of the stack may change depending on my future needs.*

### Frontend:

- FE framework: <a href="https://yew.rs/" target="_blank">Yew</a>

- Client-sided routing: <a href="https://yew.rs/docs/next/concepts/router" target="_blank">Yew Router</a>

- HTTP Requests: <a href="https://docs.rs/reqwest/latest/reqwest/" target="_blank">Reqwest</a>

- CSS: <a href="https://tailwindcss.com" target="_blank">TailwindCSS</a>

### Backend:

- ORM: <a href="https://diesel.rs/" target="_blank">Diesel</a>

- JSON: <a href="https://serde.rs/" target="_blank">Serde</a>

- Server: <a href="https://yew.rs/" target="_blank">Rocket</a>

## Workflow

### Frontend:

*Develop the front-end in live-reload mode with the following command:*

> ```trunk serve --open```

### Backend:

- Live reload: <a href="https://crates.io/crates/cargo-watch" target="_blank">Cargo watch</a>

*In order to re-compile the program after each change, run it with the following command:*

> ```cargo watch -x run```
