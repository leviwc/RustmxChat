# RustmxChat
The idea for this is to write a chat in rust with htmx

Development
run
pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch
cargo watch -x run
sea-orm-cli generate entity --with-serde both -o src/entities
