# Image Processor

CLI-приложение для обработки изображений через динамически загружаемые плагины.

## Структура проекта

```
image_converter/
├── Cargo.toml              # Workspace
├── image_processor/        # Основной бинарный крейт
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── error.rs
│       └── plugin_loader.rs
├── mirror_plugin/          # cdylib: зеркальное отражение
├── blur_plugin/            # cdylib: размытие
├── params/
│   ├── mirror.txt          # horizontal=true,vertical=false
│   └── blur.txt            # sigma=5
└── input.png               # Тестовое изображение
```

## Сборка

```bash
cargo build --workspace
```

## Запуск

```bash
# Зеркальный разворот
cargo run -p image_processor -- \
--input input.png \
--output out_mirror.png \
--plugin mirror \
--params params/mirror.txt

# Размытие
cargo run -p image_processor -- \
--input input.png \
--output out_blur.png \
--plugin blur \
--params params/blur.txt
```

Флаг `--plugin-path` опциональный, по умолчанию `target/debug`.

## Тесты

```bash
cargo test --workspace
```

## Плагины

| Плагин   | Параметры                                             | Описание                        |
|----------|-------------------------------------------------------|---------------------------------|
| `mirror` | `vertical={true or false},horizontal={true or false}` | Зеркальный разворот изображения |
| `blur`   | `sigma=N`                                             | Blur изображения                | 

Формат файла параметров `ключ=значение` через запятую, например:
```
vertical=true,horizontal=true
```
```
sigma=5
```
