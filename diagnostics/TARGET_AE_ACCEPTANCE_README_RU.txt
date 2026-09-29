ElasticGridFX — реальный функциональный тест fd69988

Перед запуском:
1. На Mac уже должен быть установлен кандидат fd69988 через отдельный updater.
2. Сохрани рабочие проекты.
3. Открой ровно один Adobe After Effects 2025.
4. Создай НОВЫЙ ПУСТОЙ несохранённый проект.
5. Не запускай Render Queue/RAM Preview параллельно.
6. Запусти RUN_ELASTICGRID_AE_TEST_MAC.command.

Проверка автоматически:
- сверяет полный установленный payload и подпись;
- подтверждает реально загруженный модуль по live image UUID/path;
- создаёт только синтетический тестовый контент;
- снимает 10 кадров, включая Adjustment Layer → ElasticGrid → Corner Pin;
- сравнивает пиксели и ловит чёрный кадр/полосы/неработающую деформацию;
- удаляет только созданные тестом объекты и восстанавливает bit depth;
- сохраняет ZIP отчёта в Desktop/ElasticGridFX-AE-Acceptance.

Если проект не пустой/сохранённый/dirty или версия не та, тест останавливается ДО
опасных действий. AE не закрывается, процессы не завершаются, preferences и кэш
не очищаются. Raw process sample остаётся локально и в ZIP не попадает.

FUNCTIONAL TEST PASS означает только: именно fd69988 была загружена и этот
детерминированный pixel/Corner Pin smoke прошёл. Release всё равно BLOCKED до
performance, interaction/project compatibility, 8/16/32-bpc host matrix,
aerender/MFR/cancellation и physical Metal gates.
