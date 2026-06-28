
## Refactoring - Migration to FSD

Текущая структура проекта
```
src/
├── App.vue
├── main.ts
├── assets/
│   ├── css/main.css
│   ├── icons/
│   └── vue.svg
├── components/
│   ├── RunClientButtonGroup.vue
│   ├── StylePreview.vue
│   └── ui/ModsTable.vue
├── composables/
│   ├── useRustABI.ts
│   └── useSettingsStore.ts
├── layouts/
│   └── ScreenLayout.vue
├── pages/
│   ├── DevMain.vue
│   ├── DevServerMain.vue
│   ├── HomeMain.vue
│   ├── ModsMain.vue
│   ├── NotFound.vue
│   └── WorkInProgress.vue
└── stories/
    └── *.stories.ts
```
Предлагаемая FSD-структура
```
src/
├── app/                          # Инициализация приложения
│   ├── main.ts                   # Точка входа
│   ├── App.vue                   # Корневой компонент
│   ├── providers/                # Провайдеры (router, pinia, sentry)
│   │   ├── index.ts
│   │   ├── router.ts             # Роутинг вынесен отдельно
│   │   └── sentry.ts
│   └── styles/                   # Глобальные стили
│       └── index.css
├── shared/                       # Переиспользуемые модули
│   ├── api/                      # API/Tauri интеграция
│   │   └── tauri.ts              # useRustABI → tauri-api
│   ├── ui/                       # UI-kit (базовые компоненты)
│   │   └── icons/                # Иконки
│   ├── lib/                      # Утилиты
│   └── config/                   # Конфигурация
├── entities/                     # Бизнес-сущности
│   ├── mod/                      # Сущность "мод"
│   │   ├── model/                # Типы, стор
│   │   └── ui/                   # Компоненты сущности
│   └── user/                     # Сущность "пользователь"
├── features/                     # Пользовательские сценарии
│   ├── run-client/               # Запуск клиента
│   │   ├── model/                # Логика запуска
│   │   └── ui/                   # RunClientButtonGroup
│   └── settings/                 # Настройки
│       └── model/                # useSettingsStore
├── widgets/                      # Композиционные блоки
│   └── screen-layout/            # ScreenLayout + навигация
│       └── ui/
└── pages/                        # Страницы (тонкие)
    ├── home/
    ├── dev/
    ├── dev-server/
    ├── mods/
    └── not-found/
```
Ключевые изменения
`composables/useRustABI.ts` -> `shared/api/tauri.ts`
`composables/useSettingsStore.ts` -> `features/settings/model/store.ts`
`components/RunClientButtonGroup.vue` -> `features/run-client/ui/`
`layouts/ScreenLayout.vue` -> `widgets/screen-layout/ui/`
`components/ui/ModsTable.vue` -> `entities/mod/ui/`
`Роутинг в main.ts` -> `app/providers/router.ts`

1. Storybook — оставить в src/stories/;
2. Tauri API будет расширяться (все, что запрещено в tauri на фронте будет в API, как минимум FS);
3. Моды можно включать/выключать (через ссылки), далее может быть расширена. Вероятно это будет сделано через tauri api.
4. Dev-страницы, страницы для отладки, в продакшене их не будет (часть может перенесу в test).
