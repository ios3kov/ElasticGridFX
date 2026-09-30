Обновление ElasticGridFX: 6d3b846 → fd69988

Цель этого пакета — только тестовая обратимая замена уже установленного
кандидата 6d3b846 на новый проверенный кандидат fd69988.

Перед заменой скрипт:
- требует полностью закрытые Adobe host/render процессы;
- проверяет, что установленная копия ТОЧНО совпадает с 6d3b846 по всем
  production-файлам, Build ID и подписи;
- проверяет новый ZIP/manifest и подпись;
- сохраняет текущую 6d3b846 целиком как previous.plugin;
- выполняет атомарную замену без copy/delete fallback;
- повторно проверяет новый установленный payload.

Новый кандидат:
commit: fd69988c10b25268eb8cad6ee6ced7f6a28bee9d
Build ID: EGFX-f442513cb6528f14295d6d45
ZIP SHA-256: 68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e

Предыдущий кандидат для rollback:
commit: 6d3b846463410f37198fda4b625e56e4cea44c22
Build ID: EGFX-603e9d3e4025d271e0488201

1. Сохрани работу и полностью закрой After Effects и остальные Adobe-программы.
2. Запусти UPDATE_ELASTICGRID_MAC.command.
3. Не удаляй папку Test Backups: ROLLBACK_ELASTICGRID_MAC.command возвращает
   именно предыдущую 6d3b846.

Меняется только:
~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin

Пароль администратора не нужен. Другие плагины, проекты, preferences и системные
настройки не изменяются. Скрипт не запускает/не закрывает AE и не скачивает код.

INSTALLED_FOR_TEST означает только успешную замену файлов. После этого обязательны
runtime Build ID, Adjustment Layer → ElasticGrid → Corner Pin smoke и дальнейшие
AE/performance проверки. Это не финальный релиз.
