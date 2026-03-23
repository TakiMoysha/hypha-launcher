# Hypha Launcher

Work in Progress

> https://en.wikipedia.org/wiki/Hypha

[For AI / AGENTS.md / Developers](./DEVELOP.md)

## Design

Минимальный ф-ционал:

- авторизация с сохранением сессии
- logout
- открытие директории игры
- показ версии игры (current/previous) - может использовать multiversions?
- [+] поведение при запуске (свернуть/закрыть/ничего не делать)
- autoupdate или проверка обновлений при запуске

Что не будет реализованно из нативного лаунчера: multiaccounts; news; social linkds; patchline; i18 и выбор языка; launch previous; uninstall;

What i want add:

- kill running game;
- disassemble server file;
- multiversions;
- multiaccounts;
- mod list;
- integration with mod resources (like curseforge, modrinth, etc.);
- map/world preview and editor;
- dev server functional;
- inspect mod (what included, verify built mod, etc.);
- modpack supports;
- [?] char preview/preset;
- [?] launch lock screen;
- [?] custom background;

```scheme
hypha-launcher/
├── bun.lock
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── backend
│   │   ├── Cargo.toml
│   │   └── src
│   └── frontend
│       ├── build.rs
│       ├── capabilities
│       ├── Cargo.toml
│       ├── icons
│       ├── src
│       └── tauri.conf.json
├── DEVELOP.md
├── index.html
├── package.json
├── public
│   ├── tauri.svg
│   └── vite.svg
├── README.md
├── src
│   ├── App.vue
│   ├── assets
│   │   └── vue.svg
│   ├── main.ts
│   └── vite-env.d.ts
├── tsconfig.json
├── tsconfig.node.json
└── vite.config.ts
```

## Resources

- [Pandoralauncher / github.com](https://github.com/Moulberry/PandoraLauncher/)
- TLauncher
