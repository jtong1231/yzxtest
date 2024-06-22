lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Статус"),
        ("Your Desktop", "Ваш рабочий стол"),
        ("desk_tip", "Ваш рабочий стол доступен с этим ID и паролем."),
        ("Password", "Пароль"),
        ("Ready", "Готов"),
        ("Established", "Установлено"),
        ("connecting_status", "Подключение к сети RustDesk..."),
        ("Enable service", "Включить службу"),
        ("Start service", "Запустить службу"),
        ("Service is running", "Служба запущена"),
        ("Service is not running", "Служба не запущена"),
        ("not_ready_status", "Не подключено. Проверьте соединение."),
        ("Control Remote Desktop", "Управление удалённым рабочим столом"),
        ("Transfer file", "Передать файлы"),
        ("Connect", "Подключиться"),
        ("Recent sessions", "Последние сеансы"),
        ("Address book", "Адресная книга"),
        ("Confirmation", "Подтверждение"),
        ("TCP tunneling", "TCP-туннелирование"),
        ("Remove", "Удалить"),
        ("Refresh random password", "Обновить случайный пароль"),
        ("Set your own password", "Установить свой пароль"),
        ("Enable keyboard/mouse", "Использовать клавиатуру/мышь"),
        ("Enable clipboard", "Использовать буфер обмена"),
        ("Enable file transfer", "Использовать передачу файлов"),
        ("Enable TCP tunneling", "Использовать туннелирование TCP"),
        ("IP Whitelisting", "Список разрешённых IP-адресов"),
        ("ID/Relay Server", "ID/Ретранслятор"),
        ("Import server config", "Импортировать конфигурацию сервера"),
        ("Export Server Config", "Экспортировать конфигурацию сервера"),
        ("Import server configuration successfully", "Конфигурация сервера успешно импортирована"),
        ("Export server configuration successfully", "Конфигурация сервера успешно экспортирована"),
        ("Invalid server configuration", "Неправильная конфигурация сервера"),
        ("Clipboard is empty", "Буфер обмена пуст"),
        ("Stop service", "Остановить службу"),
        ("Change ID", "Изменить ID"),
        ("Your new ID", "Новый ID"),
        ("length %min% to %max%", "длина %min%...%max%"),
        ("starts with a letter", "начинается с буквы"),
        ("allowed characters", "допустимые символы"),
        ("id_change_tip", "Допускаются только символы a-z, A-Z, 0-9 и _ (подчёркивание). Первой должна быть буква a-z, A-Z. Длина от 6 до 16."),
        ("Website", "Сайт"),
        ("About", "О приложении"),
        ("Slogan_tip", "Сделано с душой в этом безумном мире!"),
        ("Privacy Statement", "Заявление о конфиденциальности"),
        ("Mute", "Отключить звук"),
        ("Build Date", "Дата сборки"),
        ("Version", "Версия"),
        ("Home", "Главная"),
        ("Audio Input", "Аудиовход"),
        ("Enhancements", "Улучшения"),
        ("Hardware Codec", "Аппаратный кодек"),
        ("Adaptive bitrate", "Адаптивный битрейт"),
        ("ID Server", "Сервер ID"),
        ("Relay Server", "Ретранслятор"),
        ("API Server", "Сервер API"),
        ("invalid_http", "Адрес должен начинаться с http:// или https://"),
        ("Invalid IP", "Неправильный IP-адрес"),
        ("Invalid format", "Неправильный формат"),
        ("server_not_support", "Пока не поддерживается сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Слишком часто"),
        ("Cancel", "Отмена"),
        ("Skip", "Пропустить"),
        ("Close", "Закрыть"),
        ("Retry", "Повтор"),
        ("OK", "ОК"),
        ("Password Required", "Требуется пароль"),
        ("Please enter your password", "Введите пароль"),
        ("Remember password", "Запомнить пароль"),
        ("Wrong Password", "Неправильный пароль"),
        ("Do you want to enter again?", "Повторить вход?"),
        ("Connection Error", "Ошибка подключения"),
        ("Error", "Ошибка"),
        ("Reset by the peer", "Сброшено удалённым узлом"),
        ("Connecting...", "Подключение..."),
        ("Connection in progress. Please wait.", "Выполняется подключение. Подождите."),
        ("Please try 1 minute later", "Попробуйте через минуту"),
        ("Login Error", "Ошибка входа"),
        ("Successful", "Успешно"),
        ("Connected, waiting for image...", "Подключено, ожидание изображения..."),
        ("Name", "Имя"),
        ("Type", "Тип"),
        ("Modified", "Изменено"),
        ("Size", "Размер"),
        ("Show Hidden Files", "Показать скрытые файлы"),
        ("Receive", "Получить"),
        ("Send", "Отправить"),
        ("Refresh File", "Обновить файл"),
        ("Local", "Локальный"),
        ("Remote", "Удалённый"),
        ("Remote Computer", "Удалённый компьютер"),
        ("Local Computer", "Локальный компьютер"),
        ("Confirm Delete", "Подтвердить удаление"),
        ("Delete", "Удалить"),
        ("Properties", "Свойства"),
        ("Multi Select", "Множественный выбор"),
        ("Select All", "Выбрать все"),
        ("Unselect All", "Снять все"),
        ("Empty Directory", "Пустая папка"),
        ("Not an empty directory", "Папка не пуста"),
        ("Are you sure you want to delete this file?", "Удалить этот файл?"),
        ("Are you sure you want to delete this empty directory?", "Удалить пустую папку?"),
        ("Are you sure you want to delete the file of this directory?", "Удалить файл из этой папки?"),
        ("Do this for all conflicts", "Применить ко всем конфликтам"),
        ("This is irreversible!", "Это необратимо!"),
        ("Deleting", "Удаление"),
        ("files", "файлы"),
        ("Waiting", "Ожидание"),
        ("Finished", "Завершено"),
        ("Speed", "Скорость"),
        ("Custom Image Quality", "Заданное пользователем качество изображения"),
        ("Privacy mode", "Режим конфиденциальности"),
        ("Block user input", "Заблокировать ввод на удалённом устройстве"),
        ("Unblock user input", "Разблокировать ввод на удалённом устройстве"),
        ("Adjust Window", "Настроить окно"),
        ("Original", "Оригинал"),
        ("Shrink", "Уменьшить"),
        ("Stretch", "Растянуть"),
        ("Scrollbar", "Полоса прокрутки"),
        ("ScrollAuto", "Автопрокрутка"),
        ("Good image quality", "Лучшее качество изображения"),
        ("Balanced", "Баланс между качеством и откликом"),
        ("Optimize reaction time", "Лучшее время отклика"),
        ("Custom", "Заданное пользователем"),
        ("Show remote cursor", "Показывать удалённый курсор"),
        ("Show quality monitor", "Показывать монитор качества"),
        ("Disable clipboard", "Отключить буфер обмена"),
        ("Lock after session end", "Заблокировать учётную запись после сеанса"),
        ("Insert", "Вставить"),
        ("Insert Lock", "Заблокировать учётную запись"),
        ("Refresh", "Обновить"),
        ("ID does not exist", "ID не существует"),
        ("Failed to connect to rendezvous server", "Невозможно подключиться к промежуточному серверу"),
        ("Please try later", "Попробуйте позже"),
        ("Remote desktop is offline", "Удалённое устройство не в сети"),
        ("Key mismatch", "Несоответствие ключей"),
        ("Timeout", "Истекло время ожидания"),
        ("Failed to connect to relay server", "Невозможно подключиться к ретранслятору"),
        ("Failed to connect via rendezvous server", "Невозможно подключиться через промежуточный сервер"),
        ("Failed to connect via relay server", "Невозможно подключиться через ретранслятор"),
        ("Failed to make direct connection to remote desktop", "Невозможно установить прямое подключение к удалённому устройству"),
        ("Set Password", "Установить пароль"),
        ("OS Password", "Пароль входа в ОС"),
        ("install_tip", "В некоторых случаях из-за UAC RustDesk может работать неправильно на удалённом узле. Чтобы избежать возможных проблем с UAC, нажмите кнопку ниже для установки RustDesk в системе."),
        ("Click to upgrade", "Нажмите, чтобы обновить"),
        ("Click to download", "Нажмите, чтобы загрузить"),
        ("Click to update", "Нажмите, чтобы обновить"),
        ("Configure", "Настроить"),
        ("config_acc", "Чтобы удалённо управлять своим рабочим столом, вы должны предоставить RustDesk права \"доступа\""),
        ("config_screen", "Для удалённого доступа к рабочему столу вы должны предоставить RustDesk права \"снимок экрана\""),
        ("Installing ...", "Установка..."),
        ("Install", "Установить"),
        ("Installation", "Установка"),
        ("Installation Path", "Путь установки"),
        ("Create start menu shortcuts", "Создать ярлыки в меню \"Пуск\""),
        ("Create desktop icon", "Создать значок на рабочем столе"),
        ("agreement_tip", "Начиная установку, вы принимаете условия лицензионного соглашения."),
        ("Accept and Install", "Принять и установить"),
        ("End-user license agreement", "Лицензионное соглашение с конечным пользователем"),
        ("Generating ...", "Генерация..."),
        ("Your installation is lower version.", "Установлена более ранняя версия"),
        ("not_close_tcp_tip", "Не закрывать это окно при использовании туннеля."),
        ("Listening ...", "Ожидание..."),
        ("Remote Host", "Удалённый узел"),
        ("Remote Port", "Удалённый порт"),
        ("Action", "Действие"),
        ("Add", "Добавить"),
        ("Local Port", "Локальный порт"),
        ("Local Address", "Локальный адрес"),
        ("Change Local Port", "Изменить локальный порт"),
        ("setup_server_tip", "Для более быстрого подключения настройте собственный сервер."),
        ("Too short, at least 6 characters.", "Слишком короткий, минимум 6 символов."),
        ("The confirmation is not identical.", "Подтверждение не совпадает"),
        ("Permissions", "Разрешения"),
        ("Accept", "Принять"),
        ("Dismiss", "Отклонить"),
        ("Disconnect", "Отключить"),
        ("Enable file copy and paste", "Разрешить копирование и вставку файлов"),
        ("Connected", "Подключено"),
        ("Direct and encrypted connection", "Прямое и зашифрованное подключение"),
        ("Relayed and encrypted connection", "Ретранслируемое и зашифрованное подключение"),
        ("Direct and unencrypted connection", "Прямое и незашифрованное подключение"),
        ("Relayed and unencrypted connection", "Ретранслируемое и незашифрованное подключение"),
        ("Enter Remote ID", "Введите удалённый ID"),
        ("Enter your password", "Введите пароль"),
        ("Logging in...", "Вход..."),
        ("Enable RDP session sharing", "Использовать общий доступ к сеансу RDP"),
        ("Auto Login", "Автоматический вход в учётную запись"),
        ("Enable direct IP access", "Использовать прямой IP-доступ"),
        ("Rename", "Переименовать"),
        ("Space", "Место"),
        ("Create desktop shortcut", "Создать ярлык на рабочем столе"),
        ("Change Path", "Изменить путь"),
        ("Create Folder", "Создать папку"),
        ("Please enter the folder name", "Введите имя папки"),
        ("Fix it", "Исправить"),
        ("Warning", "Предупреждение"),
        ("Login screen using Wayland is not supported", "Вход в систему с использованием Wayland не поддерживается"),
        ("Reboot required", "Требуется перезагрузка"),
        ("Unsupported display server", "Неподдерживаемый сервер отображения"),
        ("x11 expected", "Ожидается X11"),
        ("Port", "Порт"),
        ("Settings", "Настройки"),
        ("Username", "Имя пользователя"),
        ("Invalid port", "Неправильный порт"),
        ("Closed manually by the peer", "Закрыто удалённым узлом вручную"),
        ("Enable remote configuration modification", "Разрешить удалённое изменение конфигурации"),
        ("Run without install", "Запустить без установки"),
        ("Connect via relay", "Подключится через ретранслятор"),
        ("Always connect via relay", "Всегда подключаться через ретранслятор"),
        ("whitelist_tip", "Только IP-адреса из белого списка могут получить доступ к моему устройству."),
        ("Login", "Войти"),
        ("Verify", "Проверить"),
        ("Remember me", "Запомнить"),
        ("Trust this device", "Доверенное устройство"),
        ("Verification code", "Проверочный код"),
        ("verification_tip", "Обнаружено новое устройство, на зарегистрированный адрес электронной почты отправлен проверочный код. Введите его, чтобы продолжить вход в систему."),
        ("Logout", "Выйти"),
        ("Tags", "Метки"),
        ("Search ID", "Поиск по ID"),
        ("whitelist_sep", "Разделение запятой, точкой с запятой, пробелом или новой строкой."),
        ("Add ID", "Добавить ID"),
        ("Add Tag", "Добавить ключевое слово"),
        ("Unselect all tags", "Отменить выбор всех меток"),
        ("Network error", "Ошибка сети"),
        ("Username missed", "Имя пользователя отсутствует"),
        ("Password missed", "Забыли пароль"),
        ("Wrong credentials", "Неправильные учётные данные"),
        ("The verification code is incorrect or has expired", "Проверочный код неправильный или устарел"),
        ("Edit Tag", "Изменить метку"),
        ("Forget Password", "Не сохранять пароль"),
        ("Favorites", "Избранное"),
        ("Add to Favorites", "Добавить в избранное"),
        ("Remove from Favorites", "Удалить из избранного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Недопустимое имя папки"),
        ("Socks5 Proxy", "SOCKS5-прокси"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s)-прокси"),
        ("Discovered", "Найдено"),
        ("install_daemon_tip", "Для запуска при загрузке необходимо установить системную службу"),
        ("Remote ID", "Удалённый ID"),
        ("Paste", "Вставить"),
        ("Paste here?", "Вставить сюда?"),
        ("Are you sure to close the connection?", "Завершить подключение?"),
        ("Download new version", "Скачать новую версию"),
        ("Touch mode", "Сенсорный режим"),
        ("Mouse mode", "Режим мыши/тачпада"),
        ("One-Finger Tap", "Нажатие одним пальцем"),
        ("Left Mouse", "Левая кнопка мыши"),
        ("One-Long Tap", "Долгое нажатие одним пальцем"),
        ("Two-Finger Tap", "Нажатие двумя пальцами"),
        ("Right Mouse", "Правая кнопка мыши"),
        ("One-Finger Move", "Перемещение одним пальцем"),
        ("Double Tap & Move", "Двойное нажатие и перемещение"),
        ("Mouse Drag", "Перетаскивание мышью"),
        ("Three-Finger vertically", "Тремя пальцами по вертикали"),
        ("Mouse Wheel", "Колесо мыши"),
        ("Two-Finger Move", "Перемещение двумя пальцами"),
        ("Canvas Move", "Перемещение холста"),
        ("Pinch to Zoom", "Масштабирование щипком"),
        ("Canvas Zoom", "Масштаб холста"),
        ("Reset canvas", "Сбросить масштаб холста"),
        ("No permission of file transfer", "Нет разрешения на передачу файлов"),
        ("Note", "Заметка"),
        ("Connection", "Подключение"),
        ("Share Screen", "Демонстрация экрана"),
        ("Chat", "Чат"),
        ("Total", "Всего"),
        ("items", "элементы"),
        ("Selected", "Выбрано"),
        ("Screen Capture", "Захват экрана"),
        ("Input Control", "Управление вводом"),
        ("Audio Capture", "Захват аудио"),
        ("File Connection", "Подключение передачи файлов"),
        ("Screen Connection", "Подключение просмотра/управления экраном"),
        ("Do you accept?", "Вы согласны?"),
        ("Open System Setting", "Открыть настройки системы"),
        ("How to get Android input permission?", "Как получить разрешение на ввод Android?"),
        ("android_input_permission_tip1", "Чтобы удалённое устройство могло управлять вашим Android-устройством с помощью мыши или нажатий, необходимо разрешить RustDesk использовать службу \"Специальные возможности\"."),
        ("android_input_permission_tip2", "Перейдите на соответствующую страницу системных настроек, найдите и войдите в \"Установленные службы\", включите службу \"RustDesk Input\"."),
        ("android_new_connection_tip", "Получен новый запрос на управление вашим текущим устройством."),
        ("android_service_will_start_tip", "Включение захвата экрана автоматически запускает службу, позволяя другим устройствам запрашивать подключение к этому устройству."),
        ("android_stop_service_tip", "Закрытие службы автоматически закроет все установленные подключения."),
        ("android_version_audio_tip", "Текущая версия Android не поддерживает захват звука, обновите её до Android 10 или выше."),
        ("android_start_service_tip", "Нажмите [Запустить службу] или разрешите [Захват экрана], чтобы запустить службу демонстрации экрана."),
        ("android_permission_may_not_change_tip", "Разрешения для установленных подключений не могут быть изменены, необходимо переподключение."),
        ("Account", "Аккаунт"),
        ("Overwrite", "Перезаписать"),
        ("This file exists, skip or overwrite this file?", "Файл существует, пропустить или перезаписать его?"),
        ("Quit", "Выйти"),
        ("Help", "Помощь"),
        ("Failed", "Не выполнено"),
        ("Succeeded", "Выполнено"),
        ("Someone turns on privacy mode, exit", "Кто-то включил режим конфиденциальности, выход"),
        ("Unsupported", "Не поддерживается"),
        ("Peer denied", "Отклонено удалённым узлом"),
        ("Please install plugins", "Установите плагины"),
        ("Peer exit", "Удалённый узел отключён"),
        ("Failed to turn off", "Невозможно отключить"),
        ("Turned off", "Отключён"),
        ("Language", "Язык"),
        ("Keep RustDesk background service", "Держать в фоне службу RustDesk"),
        ("Ignore Battery Optimizations", "Игнорировать оптимизацию потребления батареи"),
        ("android_open_battery_optimizations_tip", "Перейдите на следующую страницу настроек"),
        ("Start on boot", "Запускать при загрузке"),
        ("Start the screen sharing service on boot, requires special permissions", "Запускать службу демонстрации экрана при загрузке (требуются специальные разрешения)"),
        ("Connection not allowed", "Подключение не разрешено"),
        ("Legacy mode", "Устаревший режим"),
        ("Map mode", "Режим сопоставления"),
        ("Translate mode", "Режим перевода"),
        ("Use permanent password", "Использовать постоянный пароль"),
        ("Use both passwords", "Использовать оба пароля"),
        ("Set permanent password", "Установить постоянный пароль"),
        ("Enable remote restart", "Разшешить удалённую перезагрузку"),
        ("Restart remote device", "Перезапустить удалённое устройство"),
        ("Are you sure you want to restart", "Вы уверены, что хотите выполнить перезагрузку?"),
        ("Restarting remote device", "Перезагрузка удалённого устройства"),
        ("remote_restarting_tip", "Удалённое устройство перезапускается. Закройте это сообщение и через некоторое время переподключитесь, используя постоянный пароль."),
        ("Copied", "Скопировано"),
        ("Exit Fullscreen", "Выйти из полноэкранного режима"),
        ("Fullscreen", "Полноэкранный режим"),
        ("Mobile Actions", "Мобильные действия"),
        ("Select Monitor", "Выберите монитор"),
        ("Control Actions", "Действия по управлению"),
        ("Display Settings", "Настройки отображения"),
        ("Ratio", "Соотношение"),
        ("Image Quality", "Качество изображения"),
        ("Scroll Style", "Стиль прокрутки"),
        ("Show Toolbar", "Показать панель инструментов"),
        ("Hide Toolbar", "Скрыть панель инструментов"),
        ("Direct Connection", "Прямая связь"),
        ("Relay Connection", "Ретранслируемое подключение"),
        ("Secure Connection", "Безопасное подключение"),
        ("Insecure Connection", "Небезопасное подключение"),
        ("Scale original", "Оригинальный масштаб"),
        ("Scale adaptive", "Адаптивный масштаб"),
        ("General", "Общие"),
        ("Security", "Безопасность"),
        ("Theme", "Тема"),
        ("Dark Theme", "Тёмная тема"),
        ("Light Theme", "Светлая тема"),
        ("Dark", "Тёмная"),
        ("Light", "Светлая"),
        ("Follow System", "Системная"),
        ("Enable hardware codec", "Использовать аппаратный кодек"),
        ("Unlock Security Settings", "Разблокировать настройки безопасности"),
        ("Enable audio", "Включить передачу звука"),
        ("Unlock Network Settings", "Разблокировать сетевые настройки"),
        ("Server", "Сервер"),
        ("Direct IP Access", "Прямой IP-доступ"),
        ("Proxy", "Прокси"),
        ("Apply", "Применить"),
        ("Disconnect all devices?", "Отключить все устройства?"),
        ("Clear", "Очистить"),
        ("Audio Input Device", "Источник звука"),
        ("Use IP Whitelisting", "Использовать белый список IP"),
        ("Network", "Сеть"),
        ("Pin Toolbar", "Закрепить панель инструментов"),
        ("Unpin Toolbar", "Открепить панель инструментов"),
        ("Recording", "Запись"),
        ("Directory", "Папка"),
        ("Automatically record incoming sessions", "Автоматически записывать входящие сеансы"),
        ("Change", "Изменить"),
        ("Start session recording", "Начать запись сеанса"),
        ("Stop session recording", "Остановить запись сеанса"),
        ("Enable recording session", "Включить запись сеанса"),
        ("Enable LAN discovery", "Включить обнаружение в локальной сети"),
        ("Deny LAN discovery", "Запретить обнаружение в локальной сети"),
        ("Write a message", "Написать сообщение"),
        ("Prompt", "Подсказка"),
        ("Please wait for confirmation of UAC...", "Дождитесь подтверждения UAC..."),
        ("elevated_foreground_window_tip", "Текущее окно удалённого рабочего стола требует более высоких привилегий для работы, поэтому временно невозможно использовать мышь и клавиатуру. Можно попросить удалённого пользователя свернуть текущее окно или нажать кнопку повышения прав в окне управления подключением. Чтобы избежать этой проблемы в дальнейшем, рекомендуется выполнить установку программного обеспечения на удалённом устройстве."),
        ("Disconnected", "Отключено"),
        ("Other", "Другое"),
        ("Confirm before closing multiple tabs", "Подтверждать закрытие нескольких вкладок"),
        ("Keyboard Settings", "Настройки клавиатуры"),
        ("Full Access", "Полный доступ"),
        ("Screen Share", "Демонстрация экрана"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland требуется Ubuntu версии 21.04 или новее."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Для Wayland требуется более поздняя версия дистрибутива Linux. Используйте рабочий стол X11 или смените ОС."),
        ("JumpLink", "Просмотр"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Выберите экран для демонстрации (работайте на одноранговой стороне)."),
        ("Show RustDesk", "Показать RustDesk"),
        ("This PC", "Этот компьютер"),
        ("or", "или"),
        ("Continue with", "Продолжить с"),
        ("Elevate", "Повысить"),
        ("Zoom cursor", "Масштабировать курсор"),
        ("Accept sessions via password", "Принимать сеансы по паролю"),
        ("Accept sessions via click", "Принимать сеансы нажатием кнопки"),
        ("Accept sessions via both", "Принимать сеансы по паролю и нажатию кнопки"),
        ("Please wait for the remote side to accept your session request...", "Подождите, пока удалённая сторона примет ваш запрос на сеанс..."),
        ("One-time Password", "Одноразовый пароль"),
        ("Use one-time password", "Использовать одноразовый пароль"),
        ("One-time password length", "Длина одноразового пароля"),
        ("Request access to your device", "Запрос доступа к вашему устройству"),
        ("Hide connection management window", "Скрывать окно управления подключениями"),
        ("hide_cm_tip", "Разрешать скрытие в случае, если принимаются сеансы по паролю или используется постоянный пароль"),
        ("wayland_experiment_tip", "Поддержка Wayland находится на экспериментальной стадии, используйте X11, если вам требуется автоматический доступ."),
        ("Right click to select tabs", "Выбор вкладок щелчком правой кнопки мыши"),
        ("Skipped", "Пропущено"),
        ("Add to address book", "Добавить в адресную книгу"),
        ("Group", "Группа"),
        ("Search", "Поиск"),
        ("Closed manually by web console", "Закрыто вручную через веб-консоль"),
        ("Local keyboard type", "Тип локальной клавиатуры"),
        ("Select local keyboard type", "Выберите тип локальной клавиатуры"),
        ("software_render_tip", "Если у вас видеокарта Nvidia и удалённое окно закрывается сразу после подключения, может помочь установка драйвера Nouveau и выбор использования программной визуализации. Потребуется перезапуск."),
        ("Always use software rendering", "Использовать программную визуализацию"),
        ("config_input", "Чтобы управлять удалённым рабочим столом с помощью клавиатуры, необходимо предоставить RustDesk разрешения \"Мониторинг ввода\"."),
        ("config_microphone", "Чтобы разговаривать с удалённой стороной, необходимо предоставить RustDesk разрешение \"Запись аудио\"."),
        ("request_elevation_tip", "Также можно запросить повышение прав, если кто-то есть на удалённой стороне."),
        ("Wait", "Ждите"),
        ("Elevation Error", "Ошибка повышения прав"),
        ("Ask the remote user for authentication", "Запросить аутентификацию у удалённого пользователя"),
        ("Choose this if the remote account is administrator", "Выберите это, если удалённый аккаунт является администратором"),
        ("Transmit the username and password of administrator", "Передать имя пользователя и пароль администратора"),
        ("still_click_uac_tip", "По-прежнему требуется, чтобы удалённый пользователь нажал \"OK\" в окне UAC при запуске RustDesk."),
        ("Request Elevation", "Запросить повышение"),
        ("wait_accept_uac_tip", "Подождите, пока удалённый пользователь подтвердит запрос UAC."),
        ("Elevate successfully", "Права повышены"),
        ("uppercase", "заглавные"),
        ("lowercase", "строчные"),
        ("digit", "цифры"),
        ("special character", "спецсимволы"),
        ("length>=8", "8+ символов"),
        ("Weak", "Слабый"),
        ("Medium", "Средний"),
        ("Strong", "Стойкий"),
        ("Switch Sides", "Переключить стороны"),
        ("Please confirm if you want to share your desktop?", "Подтверждаете, что разрешаете демонстрацию рабочего стола?"),
        ("Display", "Отображение"),
        ("Default View Style", "Стиль отображения по умолчанию"),
        ("Default Scroll Style", "Стиль прокрутки по умолчанию"),
        ("Default Image Quality", "Качество изображения по умолчанию"),
        ("Default Codec", "Кодек по умолчанию"),
        ("Bitrate", "Битрейт"),
        ("FPS", "Частота кадров"),
        ("Auto", "Авто"),
        ("Other Default Options", "Другие параметры по умолчанию"),
        ("Voice call", "Голосовой вызов"),
        ("Text chat", "Текстовый чат"),
        ("Stop voice call", "Завершить голосовой вызов"),
        ("relay_hint_tip", "Прямое подключение может оказаться невозможным. В этом случае можно попытаться подключиться через ретранслятор. \nКроме того, если вы хотите сразу использовать ретранслятор, можно добавить к ID суффикс \"/r\" или включить \"Всегда подключаться через ретранслятор\" в настройках удалённого узла."),
        ("Reconnect", "Переподключить"),
        ("Codec", "Кодек"),
        ("Resolution", "Разрешение"),
        ("No transfers in progress", "Передача не осуществляется"),
        ("Set one-time password length", "Установить длину одноразового пароля"),
        ("RDP Settings", "Настройки RDP"),
        ("Sort by", "Сортировка"),
        ("New Connection", "Новое подключение"),
        ("Restore", "Восстановить"),
        ("Minimize", "Свернуть"),
        ("Maximize", "Развернуть"),
        ("Your Device", "Ваше устройство"),
        ("empty_recent_tip", "Нет последних сеансов!\nПора спланировать новый."),
        ("empty_favorite_tip", "Ещё нет избранных удалённых узлов?\nДавайте найдём, кого можно добавить в избранное!"),
        ("empty_lan_tip", "Не найдено удалённых узлов."),
        ("empty_address_book_tip", "В адресной книге нет удалённых узлов."),
        ("eg: admin", "например: admin"),
        ("Empty Username", "Пустое имя пользователя"),
        ("Empty Password", "Пустой пароль"),
        ("Me", "Я"),
        ("identical_file_tip", "Файл идентичен файлу на удалённом узле"),
        ("show_monitors_tip", "Показывать мониторы на панели инструментов"),
        ("View Mode", "Режим просмотра"),
        ("login_linux_tip", "Чтобы включить сеанс рабочего стола X, необходимо войти в удалённый аккаунт Linux."),
        ("verify_rustdesk_password_tip", "Подтвердить пароль RustDesk"),
        ("remember_account_tip", "Запомнить этот аккаунт"),
        ("os_account_desk_tip", "Этот аккаунт используется для входа в удалённую ОС и включения сеанса рабочего стола в режиме headless."),
        ("OS Account", "Аккаунт ОС"),
        ("another_user_login_title_tip", "Другой пользователь уже вошёл в систему"),
        ("another_user_login_text_tip", "Отключить"),
        ("xorg_not_found_title_tip", "Xorg не найден"),
        ("xorg_not_found_text_tip", "Установите Xorg"),
        ("no_desktop_title_tip", "Нет доступных рабочих столов"),
        ("no_desktop_text_tip", "Установите GNOME Desktop"),
        ("No need to elevate", "Повышение прав не требуется"),
        ("System Sound", "Системный звук"),
        ("Default", "По умолчанию"),
        ("New RDP", "Новый RDP"),
        ("Fingerprint", "Отпечаток"),
        ("Copy Fingerprint", "Копировать отпечаток"),
        ("no fingerprints", "отпечатки отсутствуют"),
        ("Select a peer", "Выберите удалённый узел"),
        ("Select peers", "Выберите удалённые узлы"),
        ("Plugins", "Плагины"),
        ("Uninstall", "Удалить"),
        ("Update", "Обновить"),
        ("Enable", "Включить"),
        ("Disable", "Отключить"),
        ("Options", "Настройки"),
        ("resolution_original_tip", "Исходное разрешение"),
        ("resolution_fit_local_tip", "Соответствие локальному разрешению"),
        ("resolution_custom_tip", "Произвольное разрешение"),
        ("Collapse toolbar", "Свернуть панель инструментов"),
        ("Accept and Elevate", "Принять и повысить"),
        ("accept_and_elevate_btn_tooltip", "Разрешить подключение и повысить права UAC."),
        ("clipboard_wait_response_timeout_tip", "Время ожидания копирования буфера обмена истекло"),
        ("Incoming connection", "Входящее подключение"),
        ("Outgoing connection", "Исходящее подключение"),
        ("Exit", "Выход"),
        ("Open", "Открыть"),
        ("logout_tip", "Вы действительно хотите выйти?"),
        ("Service", "Служба"),
        ("Start", "Запустить"),
        ("Stop", "Остановить"),
        ("exceed_max_devices", "Достигнуто максимальное количество управляемых устройств."),
        ("Sync with recent sessions", "Синхронизация последних сеансов"),
        ("Sort tags", "Сортировка меток"),
        ("Open connection in new tab", "Открыть подключение в новой вкладке"),
        ("Move tab to new window", "Переместить вкладку в отдельное окно"),
        ("Can not be empty", "Не может быть пустым"),
        ("Already exists", "Уже существует"),
        ("Change Password", "Изменить пароль"),
        ("Refresh Password", "Обновить пароль"),
        ("ID", "ID"),
        ("Grid View", "Сетка"),
        ("List View", "Список"),
        ("Select", "Выбор"),
        ("Toggle Tags", "Переключить метки"),
        ("pull_ab_failed_tip", "Невозможно обновить адресную книгу"),
        ("push_ab_failed_tip", "Невозможно синхронизировать адресную книгу с сервером"),
        ("synced_peer_readded_tip", "Устройства, присутствовавшие в последних сеансах, будут синхронизированы с адресной книгой."),
        ("Change Color", "Изменить цвет"),
        ("Primary Color", "Основной цвет"),
        ("HSV Color", "Цвет HSV"),
        ("Installation Successful!", "Установка выполнена успешно!"),
        ("Installation failed!", "Установка не выполнена!"),
        ("Reverse mouse wheel", "Реверсировать колесо мыши"),
        ("{} sessions", "{} сеансов"),
        ("scam_title", "Вы можете быть ОБМАНУТЫ!"),
        ("scam_text1", "Если вы разговариваете по телефону с кем-то, кого вы НЕ ЗНАЕТЕ и НЕ ДОВЕРЯЕТЕ, и он просит вас использовать RustDesk и запустить его службу, не продолжайте и немедленно прервите разговор."),
        ("scam_text2", "Скорее всего, это мошенник, пытающийся украсть ваши деньги или другую личную информацию."),
        ("Don't show again", "Больше не показывать"),
        ("I Agree", "Принимаю"),
        ("Decline", "Отказ"),
        ("Timeout in minutes", "Время ожидания (минут)"),
        ("auto_disconnect_option_tip", "Автоматически закрывать входящие сеансы при неактивности пользователя"),
        ("Connection failed due to inactivity", "Подключение не выполнено из-за неактивности"),
        ("Check for software update on startup", "Проверять обновления программы при запуске"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Обновите RustDesk Server Pro до версии {} или новее!"),
        ("pull_group_failed_tip", "Невозможно обновить группу"),
        ("Filter by intersection", "Фильтровать по пересечению"),
        ("Remove wallpaper during incoming sessions", "Скрывать обои рабочего стола при входящем сеансе"),
        ("Test", "Тест"),
        ("display_is_plugged_out_msg", "Дисплей отключён, переключитесь на первый дисплей."),
        ("No displays", "Нет дисплеев"),
        ("elevated_switch_display_msg", "Переключитесь на основной дисплей, поскольку в режиме повышенных прав несколько дисплеев не поддерживаются."),
        ("Open in new window", "Открыть в новом окне"),
        ("Show displays as individual windows", "Показывать дисплеи в отдельных окнах"),
        ("Use all my displays for the remote session", "Использовать все мои дисплеи для удалённого сеанса"),
        ("selinux_tip", "На вашем устройстве включён SELinux, что может помешать правильной работе RustDesk на контролируемой стороне."),
        ("Change view", "Вид"),
        ("Big tiles", "Большие значки"),
        ("Small tiles", "Маленькие значки"),
        ("List", "Список"),
        ("Virtual display", "Виртуальный дисплей"),
        ("Plug out all", "Отключить все"),
        ("True color (4:4:4)", "True color (4:4:4)"),
        ("Enable blocking user input", "Разрешить блокировать ввод на устройстве"),
        ("id_input_tip", "Можно ввести идентификатор, прямой IP-адрес или домен с портом (<домен>:<порт>).\nЕсли необходимо получить доступ к устройству на другом сервере, добавьте адрес сервера (<id>@<адрес_сервера>?key=<ключ_значение>), например:\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nЕсли необходимо получить доступ к устройству на общедоступном сервере, введите \"<id>@public\", ключ для публичного сервера не требуется."),
        ("privacy_mode_impl_mag_tip", "Режим 1"),
        ("privacy_mode_impl_virtual_display_tip", "Режим 2"),
        ("Enter privacy mode", "Включить режим конфиденциальности"),
        ("Exit privacy mode", "Отключить режим конфиденциальности"),
        ("idd_not_support_under_win10_2004_tip", "Драйвер непрямого отображения не поддерживается. Требуется Windows 10 версии 2004 или новее."),
        ("switch_display_elevated_connections_tip", "Переключение на неосновной дисплей не поддерживается в режиме повышенных прав при наличии нескольких подключений. Повторите попытку после установки, если хотите управлять несколькими дисплеями."),
        ("input_source_1_tip", "Источник ввода 1"),
        ("input_source_2_tip", "Источник ввода 2"),
        ("capture_display_elevated_connections_tip", "Захват экрана нескольких дисплеев не поддерживается в режиме повышенных прав. Повторите попытку после установки, если хотите управлять несколькими дисплеями."),
        ("Swap control-command key", "Поменять местами значения кнопок Ctrl и Command"),
        ("swap-left-right-mouse", "Поменять местами значения левой и правой кнопок мыши"),
        ("2FA code", "Код двухфакторной аутентификации"),
        ("More", "Ещё"),
        ("enable-2fa-title", "Использовать двухфакторную аутентификацию"),
        ("enable-2fa-desc", "Настройте приложение аутентификации. Используйте, например, Authy, Microsoft или Google Authenticator, на телефоне или компьютере.\n\nОтсканируйте QR-код с помощью приложения аутентификации и введите код, который отобразит это приложение, чтобы включить двухфакторную аутентификацию."),
        ("wrong-2fa-code", "Невозможно подтвердить код. Проверьте код и настройки местного времени."),
        ("enter-2fa-title", "Двухфакторная аутентификация"),
        ("Email verification code must be 6 characters.", "Код подтверждения электронной почты должен состоять из 6 символов."),
        ("2FA code must be 6 digits.", "Код двухфакторной аутентификации должен состоять из 6 цифр."),
        ("Multiple Windows sessions found", "Обнаружено несколько сеансов Windows"),
        ("Please select the session you want to connect to", "Выберите сеанс, к которому хотите подключиться"),
        ("powered_by_me", "Основано на RustDesk"),
        ("outgoing_only_desk_tip", "Это специализированная версия.\nВы можете подключаться к другим устройствам, но другие устройства не могут подключиться к вашему."),
        ("preset_password_warning", "Это специализированная версия с предустановленным паролем. Любой, кто знает этот пароль, может получить полный контроль над вашим устройством. Если это для вас неожиданно, немедленно удалите данное программное обеспечение."),
        ("Security Alert", "Предупреждение о безопасности"),
        ("My address book", "Моя адресная книга"),
        ("Personal", "Личная"),
        ("Owner", "Владелец"),
        ("Set shared password", "Установить общий пароль"),
        ("Exist in", "Существует в"),
        ("Read-only", "Только чтение"),
        ("Read/Write", "Чтение и запись"),
        ("Full Control", "Полный доступ"),
        ("share_warning_tip", "Поля выше являются общими и видны другим."),
        ("Everyone", "Все"),
        ("ab_web_console_tip", "Больше в веб-консоли"),
        ("allow-only-conn-window-open-tip", "Разрешать подключение только при открытом окне RustDesk"),
        ("no_need_privacy_mode_no_physical_displays_tip", "Физические дисплеи отсутствуют, нет необходимости использовать режим конфиденциальности."),
        ("Follow remote cursor", "Следовать за удалённым курсором"),
        ("Follow remote window focus", "Следовать за фокусом удалённого окна"),
        ("default_proxy_tip", "Протокол и порт по умолчанию: Socks5 и 1080"),
        ("no_audio_input_device_tip", "Устройство аудиовхода не найдено."),
        ("Incoming", "Входящие"),
        ("Outgoing", "Исходящие"),
        ("Clear Wayland screen selection", "Отменить выбор экрана Wayland"),
        ("clear_Wayland_screen_selection_tip", "После отмены можно заново выбрать экран для демонстрации."),
        ("confirm_clear_Wayland_screen_selection_tip", "Отменить выбор экрана Wayland?"),
        ("android_new_voice_call_tip", "Получен новый запрос на голосовой вызов. Если вы его примите, звук переключится на голосовую связь."),
        ("texture_render_tip", "Использовать визуализацию текстур, чтобы сделать изображения более плавными."),
        ("Use texture rendering", "Визуализация текстур"),
        ("Floating window", "Плавающее окно"),
        ("floating_window_tip", "Помогает поддерживать фоновую службу RustDesk"),
        ("Keep screen on", "Держать экран включённым"),
        ("Never", "Нет"),
        ("During controlled", "При управлении"),
        ("During service is on", "При запущенной службе"),
        ("Capture screen using DirectX", "Захват экрана с помощью DirectX"),
    ].iter().cloned().collect();
}
