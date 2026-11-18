# 🗂️ Portfolio Site

A sort of nice portfolio page for Jonas (me!).
It uses Leptos, meaning the whole thing is written in rust.
_Disclaimer_: I am no designer, and I don't plan on becoming one (even though I'm very happy with the site and wrote it myself). The design is just CSS and I probably won't be improving it.

The site is currently hosted at [jonas.baugerud.no](https://jonas.baugerud.no).
It supports CSR for smooth navigation, but also full SSR and hydration. The login system is Google OIDC and only used for managing the portfolio as of right now.
The site is hosted on a VPS using [my own NixOS-anywhere-based deployment tool](https://github.com/MPM-Labs/nixos-deployment-template). It sits behind a Caddy instance.

I do plan on improving it, but cannot say when or how that will be ;)

## Roadmap

| Feature | Status |
|---|---|
| Google OIDC via [`openidconnect`](https://crates.io/crates/openidconnect) | ✅ Done |
| JSON web token generation via [`jsonwebtoken`](https://crates.io/crates/jsonwebtoken) | ✅ Done |
| Refresh tokens in-memory for seemless reauth | ✅ Done |
| Postgres connection via [`sqlx`](https://crates.io/crates/sqlx) | ✅ Done |
| refresh tokens moved to Postgres | ⬜ Planned |
| Admin pages *(hopefully you'll never see these 👀)* | ✅ Done |
| Projects page | ✅ Done |
| Dynamic image storage via [MinIO](https://crates.io/crates/aws-sdk-s3) | ⬜ Planned |
| Postgres backups | ⬜ Planned |
| Logging | ⬜ Planned |
| Collaborator profiles | ⬜ Planned |
