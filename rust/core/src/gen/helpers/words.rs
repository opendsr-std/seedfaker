pub fn words_for_locale(code: &str) -> &'static [&'static str] {
    match code {
        "de" | "de-at" => &[
            "Arbeit",
            "Bericht",
            "Daten",
            "Fehler",
            "Kunde",
            "Lizenz",
            "Meldung",
            "Nutzer",
            "Passwort",
            "Projekt",
            "Sicherheit",
            "Speicher",
            "System",
            "Verbindung",
            "Verwaltung",
            "Zugang",
            "Anfrage",
            "Antwort",
            "Aufgabe",
            "Dienst",
            "Einstellung",
            "Freigabe",
            "Konfiguration",
            "Protokoll",
            "Schnittstelle",
            "Vorgang",
            "Warnung",
            "Zeitplan",
            "Zugriff",
            "Ablauf",
        ],
        "fr" | "fr-be" | "fr-ca" => &[
            "acc\u{e8}s",
            "compte",
            "connexion",
            "donn\u{e9}es",
            "erreur",
            "fichier",
            "licence",
            "message",
            "mise \u{e0} jour",
            "mot de passe",
            "projet",
            "rapport",
            "requ\u{ea}te",
            "r\u{e9}seau",
            "sauvegarde",
            "s\u{e9}curit\u{e9}",
            "service",
            "syst\u{e8}me",
            "t\u{e2}che",
            "utilisateur",
            "validation",
            "configuration",
            "d\u{e9}ploiement",
            "notification",
            "param\u{e8}tre",
            "processus",
            "r\u{e9}ponse",
            "stockage",
            "traitement",
            "v\u{e9}rification",
        ],
        "es" | "ar" | "mx" | "cl" | "co" | "pe" | "uy" | "ve" | "ec" => &[
            "acceso",
            "archivo",
            "conexi\u{f3}n",
            "contrase\u{f1}a",
            "cuenta",
            "datos",
            "error",
            "informe",
            "licencia",
            "mensaje",
            "notificaci\u{f3}n",
            "proceso",
            "proyecto",
            "red",
            "respaldo",
            "seguridad",
            "servicio",
            "sistema",
            "tarea",
            "usuario",
            "validaci\u{f3}n",
            "actualizaci\u{f3}n",
            "configuraci\u{f3}n",
            "consulta",
            "despliegue",
            "inicio",
            "permiso",
            "registro",
            "reporte",
            "solicitud",
        ],
        "it" => &[
            "accesso",
            "account",
            "aggiornamento",
            "archivio",
            "configurazione",
            "connessione",
            "dati",
            "errore",
            "file",
            "licenza",
            "messaggio",
            "password",
            "processo",
            "progetto",
            "rapporto",
            "rete",
            "richiesta",
            "risposta",
            "servizio",
            "sicurezza",
            "sistema",
            "utente",
            "backup",
            "notifica",
            "permesso",
            "registro",
            "salvataggio",
            "validazione",
            "verifica",
            "attivit\u{e0}",
        ],
        "pt" | "pt-br" => &[
            "acesso",
            "arquivo",
            "conex\u{e3}o",
            "conta",
            "dados",
            "erro",
            "licen\u{e7}a",
            "mensagem",
            "notifica\u{e7}\u{e3}o",
            "processo",
            "projeto",
            "rede",
            "relat\u{f3}rio",
            "seguran\u{e7}a",
            "senha",
            "servi\u{e7}o",
            "sistema",
            "tarefa",
            "usu\u{e1}rio",
            "valida\u{e7}\u{e3}o",
            "atualiza\u{e7}\u{e3}o",
            "backup",
            "configura\u{e7}\u{e3}o",
            "consulta",
            "implanta\u{e7}\u{e3}o",
            "permiss\u{e3}o",
            "registro",
            "requisi\u{e7}\u{e3}o",
            "resposta",
            "verifica\u{e7}\u{e3}o",
        ],
        "ja" => &[
            "\u{30a2}\u{30ab}\u{30a6}\u{30f3}\u{30c8}",
            "\u{30a2}\u{30af}\u{30bb}\u{30b9}",
            "\u{30a8}\u{30e9}\u{30fc}",
            "\u{30b5}\u{30fc}\u{30d0}\u{30fc}",
            "\u{30b7}\u{30b9}\u{30c6}\u{30e0}",
            "\u{30bb}\u{30ad}\u{30e5}\u{30ea}\u{30c6}\u{30a3}",
            "\u{30c7}\u{30fc}\u{30bf}",
            "\u{30cd}\u{30c3}\u{30c8}\u{30ef}\u{30fc}\u{30af}",
            "\u{30d0}\u{30c3}\u{30af}\u{30a2}\u{30c3}\u{30d7}",
            "\u{30d1}\u{30b9}\u{30ef}\u{30fc}\u{30c9}",
            "\u{30d5}\u{30a1}\u{30a4}\u{30eb}",
            "\u{30d7}\u{30ed}\u{30b8}\u{30a7}\u{30af}\u{30c8}",
            "\u{30d7}\u{30ed}\u{30bb}\u{30b9}",
            "\u{30e1}\u{30c3}\u{30bb}\u{30fc}\u{30b8}",
            "\u{30e6}\u{30fc}\u{30b6}\u{30fc}",
            "\u{30ea}\u{30af}\u{30a8}\u{30b9}\u{30c8}",
            "\u{30ec}\u{30dd}\u{30fc}\u{30c8}",
            "\u{30ed}\u{30b0}",
            "\u{66f4}\u{65b0}",
            "\u{8a2d}\u{5b9a}",
            "\u{901a}\u{77e5}",
            "\u{63a5}\u{7d9a}",
            "\u{8a8d}\u{8a3c}",
            "\u{6a29}\u{9650}",
            "\u{78ba}\u{8a8d}",
            "\u{691c}\u{8a3c}",
            "\u{51e6}\u{7406}",
            "\u{4fdd}\u{5b58}",
            "\u{524a}\u{9664}",
            "\u{767b}\u{9332}",
        ],
        "zh" | "tw" => &[
            "\u{8d26}\u{6237}",
            "\u{8bbf}\u{95ee}",
            "\u{5907}\u{4efd}",
            "\u{914d}\u{7f6e}",
            "\u{8fde}\u{63a5}",
            "\u{6570}\u{636e}",
            "\u{90e8}\u{7f72}",
            "\u{9519}\u{8bef}",
            "\u{6587}\u{4ef6}",
            "\u{8bb8}\u{53ef}",
            "\u{6d88}\u{606f}",
            "\u{7f51}\u{7edc}",
            "\u{901a}\u{77e5}",
            "\u{5bc6}\u{7801}",
            "\u{6743}\u{9650}",
            "\u{8fdb}\u{7a0b}",
            "\u{9879}\u{76ee}",
            "\u{62a5}\u{544a}",
            "\u{8bf7}\u{6c42}",
            "\u{5b89}\u{5168}",
            "\u{670d}\u{52a1}",
            "\u{7cfb}\u{7edf}",
            "\u{4efb}\u{52a1}",
            "\u{7528}\u{6237}",
            "\u{9a8c}\u{8bc1}",
            "\u{66f4}\u{65b0}",
            "\u{65e5}\u{5fd7}",
            "\u{8bbe}\u{7f6e}",
            "\u{5b58}\u{50a8}",
            "\u{6ce8}\u{518c}",
        ],
        // Ukrainian
        "uk" => &[
            "\u{434}\u{43e}\u{441}\u{442}\u{443}\u{43f}", // доступ
            "\u{434}\u{430}\u{43d}\u{456}",               // дані
            "\u{43f}\u{43e}\u{43c}\u{438}\u{43b}\u{43a}\u{430}", // помилка
            "\u{43c}\u{435}\u{440}\u{435}\u{436}\u{430}", // мережа
            "\u{43f}\u{430}\u{440}\u{43e}\u{43b}\u{44c}", // пароль
            "\u{43f}\u{440}\u{43e}\u{446}\u{435}\u{441}", // процес
            "\u{43f}\u{440}\u{43e}\u{435}\u{43a}\u{442}", // проект
            "\u{441}\u{435}\u{440}\u{432}\u{456}\u{441}", // сервіс
            "\u{441}\u{438}\u{441}\u{442}\u{435}\u{43c}\u{430}", // система
            "\u{437}\u{430}\u{43f}\u{438}\u{442}",        // запит
            "\u{431}\u{435}\u{437}\u{43f}\u{435}\u{43a}\u{430}", // безпека
            "\u{444}\u{430}\u{439}\u{43b}",               // файл
            "\u{43a}\u{43e}\u{440}\u{438}\u{441}\u{442}\u{443}\u{432}\u{430}\u{447}", // користувач
            "\u{437}\u{432}\u{456}\u{442}",               // звіт
            "\u{43f}\u{43e}\u{432}\u{456}\u{434}\u{43e}\u{43c}\u{43b}\u{435}\u{43d}\u{43d}\u{44f}", // повідомлення
            "\u{43e}\u{43d}\u{43e}\u{432}\u{43b}\u{435}\u{43d}\u{43d}\u{44f}", // оновлення
            "\u{437}\u{430}\u{432}\u{434}\u{430}\u{43d}\u{43d}\u{44f}",        // завдання
            "\u{43d}\u{430}\u{43b}\u{430}\u{448}\u{442}\u{443}\u{432}\u{430}\u{43d}\u{43d}\u{44f}", // налаштування
        ],
        // Belarusian
        "be" => &[
            "\u{434}\u{43e}\u{441}\u{442}\u{443}\u{43f}", // доступ
            "\u{434}\u{430}\u{434}\u{437}\u{435}\u{43d}\u{44b}\u{44f}", // дадзеныя
            "\u{43f}\u{430}\u{43c}\u{44b}\u{43b}\u{43a}\u{430}", // памылка
            "\u{441}\u{435}\u{442}\u{43a}\u{430}",        // сетка
            "\u{43f}\u{430}\u{440}\u{43e}\u{43b}\u{44c}", // пароль
            "\u{43f}\u{440}\u{430}\u{446}\u{44d}\u{441}", // працэс
            "\u{43f}\u{440}\u{430}\u{435}\u{43a}\u{442}", // праект
            "\u{441}\u{435}\u{440}\u{432}\u{456}\u{441}", // сервіс
            "\u{441}\u{456}\u{441}\u{442}\u{44d}\u{43c}\u{430}", // сістэма
            "\u{437}\u{430}\u{43f}\u{44b}\u{442}",        // запыт
            "\u{431}\u{44f}\u{441}\u{43f}\u{435}\u{43a}\u{430}", // бяспека
            "\u{444}\u{430}\u{439}\u{43b}",               // файл
            "\u{43a}\u{430}\u{440}\u{44b}\u{441}\u{442}\u{430}\u{43b}\u{44c}\u{43d}\u{456}\u{43a}", // карыстальнік
            "\u{441}\u{43f}\u{440}\u{430}\u{432}\u{430}\u{437}\u{434}\u{430}\u{447}\u{430}", // справаздача
            "\u{43f}\u{430}\u{432}\u{435}\u{434}\u{430}\u{43c}\u{43b}\u{435}\u{43d}\u{43d}\u{435}", // паведамленне
            "\u{430}\u{431}\u{43d}\u{430}\u{45e}\u{43b}\u{435}\u{43d}\u{43d}\u{435}", // абнаўленне
            "\u{437}\u{430}\u{434}\u{430}\u{447}\u{430}",                             // задача
            "\u{43d}\u{430}\u{43b}\u{430}\u{434}\u{43a}\u{430}",                      // наладка
        ],
        // Russian
        "ru" => &[
            "\u{434}\u{43e}\u{441}\u{442}\u{443}\u{43f}", // доступ
            "\u{434}\u{430}\u{43d}\u{43d}\u{44b}\u{435}", // данные
            "\u{43e}\u{448}\u{438}\u{431}\u{43a}\u{430}", // ошибка
            "\u{441}\u{435}\u{442}\u{44c}",               // сеть
            "\u{43f}\u{430}\u{440}\u{43e}\u{43b}\u{44c}", // пароль
            "\u{43f}\u{440}\u{43e}\u{446}\u{435}\u{441}\u{441}", // процесс
            "\u{43f}\u{440}\u{43e}\u{435}\u{43a}\u{442}", // проект
            "\u{441}\u{435}\u{440}\u{432}\u{438}\u{441}", // сервис
            "\u{441}\u{438}\u{441}\u{442}\u{435}\u{43c}\u{430}", // система
            "\u{437}\u{430}\u{43f}\u{440}\u{43e}\u{441}", // запрос
            "\u{431}\u{435}\u{437}\u{43e}\u{43f}\u{430}\u{441}\u{43d}\u{43e}\u{441}\u{442}\u{44c}", // безопасность
            "\u{444}\u{430}\u{439}\u{43b}", // файл
            "\u{43f}\u{43e}\u{43b}\u{44c}\u{437}\u{43e}\u{432}\u{430}\u{442}\u{435}\u{43b}\u{44c}", // пользователь
            "\u{43e}\u{442}\u{447}\u{451}\u{442}", // отчёт
            "\u{441}\u{43e}\u{43e}\u{431}\u{449}\u{435}\u{43d}\u{438}\u{435}", // сообщение
            "\u{43e}\u{431}\u{43d}\u{43e}\u{432}\u{43b}\u{435}\u{43d}\u{438}\u{435}", // обновление
            "\u{437}\u{430}\u{434}\u{430}\u{447}\u{430}", // задача
            "\u{43d}\u{430}\u{441}\u{442}\u{440}\u{43e}\u{439}\u{43a}\u{430}", // настройка
        ],
        // Serbian (Cyrillic)
        "sr" => &[
            "pristup",
            "podaci",
            "gre\u{161}ka",
            "mre\u{17e}a",
            "lozinka",
            "proces",
            "projekat",
            "sistem",
            "korisnik",
            "zadatak",
            "bezbednost",
            "usluga",
            "datoteka",
            "izve\u{161}taj",
            "poruka",
            "a\u{17e}uriranje",
        ],
        // Bulgarian (Cyrillic)
        "bg" => &[
            "\u{434}\u{43e}\u{441}\u{442}\u{44a}\u{43f}", // достъп
            "\u{434}\u{430}\u{43d}\u{43d}\u{438}",        // данни
            "\u{433}\u{440}\u{435}\u{448}\u{43a}\u{430}", // грешка
            "\u{43c}\u{440}\u{435}\u{436}\u{430}",        // мрежа
            "\u{43f}\u{430}\u{440}\u{43e}\u{43b}\u{430}", // парола
            "\u{43f}\u{440}\u{43e}\u{446}\u{435}\u{441}", // процес
            "\u{43f}\u{440}\u{43e}\u{435}\u{43a}\u{442}", // проект
            "\u{441}\u{438}\u{441}\u{442}\u{435}\u{43c}\u{430}", // система
            "\u{43f}\u{43e}\u{442}\u{440}\u{435}\u{431}\u{438}\u{442}\u{435}\u{43b}", // потребител
            "\u{437}\u{430}\u{434}\u{430}\u{447}\u{430}", // задача
            "\u{441}\u{438}\u{433}\u{443}\u{440}\u{43d}\u{43e}\u{441}\u{442}", // сигурност
            "\u{443}\u{441}\u{43b}\u{443}\u{433}\u{430}", // услуга
            "\u{444}\u{430}\u{439}\u{43b}",               // файл
            "\u{434}\u{43e}\u{43a}\u{43b}\u{430}\u{434}", // доклад
            "\u{441}\u{44a}\u{43e}\u{431}\u{449}\u{435}\u{43d}\u{438}\u{435}", // съобщение
            "\u{430}\u{43a}\u{442}\u{443}\u{430}\u{43b}\u{438}\u{437}\u{430}\u{446}\u{438}\u{44f}", // актуализация
        ],
        // Croatian (Latin)
        "hr" => &[
            "pristup",
            "podaci",
            "pogre\u{161}ka",
            "mre\u{17e}a",
            "lozinka",
            "proces",
            "projekt",
            "sustav",
            "korisnik",
            "zadatak",
            "sigurnost",
            "usluga",
            "datoteka",
            "izvje\u{161}taj",
            "poruka",
            "a\u{17e}uriranje",
        ],
        // Slovenian (Latin)
        "sl" => &[
            "dostop",
            "podatki",
            "napaka",
            "omre\u{17e}je",
            "geslo",
            "proces",
            "projekt",
            "sistem",
            "uporabnik",
            "naloga",
            "varnost",
            "storitev",
            "datoteka",
            "poro\u{10d}ilo",
            "sporo\u{10d}ilo",
            "posodobitev",
        ],
        // Korean
        "ko" => &[
            "\u{c811}\u{adc0}",                 // 접근 (access)
            "\u{b370}\u{c774}\u{d130}",         // 데이터 (data)
            "\u{c624}\u{b958}",                 // 오류 (error)
            "\u{be44}\u{bc00}\u{bc88}\u{d638}", // 비밀번호 (password)
            "\u{c2dc}\u{c2a4}\u{d15c}",         // 시스템 (system)
            "\u{c0ac}\u{c6a9}\u{c790}",         // 사용자 (user)
            "\u{d30c}\u{c77c}",                 // 파일 (file)
            "\u{d504}\u{b85c}\u{c81d}\u{d2b8}", // 프로젝트 (project)
            "\u{c11c}\u{be44}\u{c2a4}",         // 서비스 (service)
            "\u{bcf4}\u{c548}",                 // 보안 (security)
            "\u{b124}\u{d2b8}\u{c6cc}\u{d06c}", // 네트워크 (network)
            "\u{bcf4}\u{ace0}\u{c11c}",         // 보고서 (report)
            "\u{d504}\u{b85c}\u{c138}\u{c2a4}", // 프로세스 (process)
            "\u{c791}\u{c5c5}",                 // 작업 (task)
            "\u{c124}\u{c815}",                 // 설정 (settings)
            "\u{ba54}\u{c2dc}\u{c9c0}",         // 메시지 (message)
        ],
        // Hindi (Devanagari)
        "hi" => &[
            "\u{92a}\u{939}\u{941}\u{901}\u{91a}",        // पहुँच (access)
            "\u{921}\u{947}\u{91f}\u{93e}",               // डेटा (data)
            "\u{924}\u{94d}\u{930}\u{941}\u{91f}\u{93f}", // त्रुटि (error)
            "\u{92a}\u{93e}\u{938}\u{935}\u{930}\u{94d}\u{921}", // पासवर्ड (password)
            "\u{92a}\u{94d}\u{930}\u{923}\u{93e}\u{932}\u{940}", // प्रणाली (system)
            "\u{909}\u{92a}\u{92f}\u{94b}\u{917}\u{915}\u{930}\u{94d}\u{924}\u{93e}", // उपयोगकर्ता (user)
            "\u{92b}\u{93c}\u{93e}\u{907}\u{932}",                                    // फ़ाइल (file)
            "\u{92a}\u{930}\u{93f}\u{92f}\u{94b}\u{91c}\u{928}\u{93e}", // परियोजना (project)
            "\u{938}\u{947}\u{935}\u{93e}",                             // सेवा (service)
            "\u{938}\u{941}\u{930}\u{915}\u{94d}\u{937}\u{93e}",        // सुरक्षा (security)
            "\u{928}\u{947}\u{91f}\u{935}\u{930}\u{94d}\u{915}",        // नेटवर्क (network)
            "\u{930}\u{93f}\u{92a}\u{94b}\u{930}\u{94d}\u{91f}",        // रिपोर्ट (report)
            "\u{92a}\u{94d}\u{930}\u{915}\u{94d}\u{930}\u{93f}\u{92f}\u{93e}", // प्रक्रिया (process)
            "\u{938}\u{902}\u{926}\u{947}\u{936}",                      // संदेश (message)
            "\u{915}\u{93e}\u{930}\u{94d}\u{92f}",                      // कार्य (task)
            "\u{905}\u{926}\u{94d}\u{92f}\u{924}\u{928}",               // अद्यतन (update)
        ],
        // Hebrew
        "he" => &[
            "\u{5d2}\u{5d9}\u{5e9}\u{5d4}",               // גישה (access)
            "\u{5e0}\u{5ea}\u{5d5}\u{5e0}\u{5d9}\u{5dd}", // נתונים (data)
            "\u{5e9}\u{5d2}\u{5d9}\u{5d0}\u{5d4}",        // שגיאה (error)
            "\u{5e1}\u{5d9}\u{5e1}\u{5de}\u{5d4}",        // סיסמה (password)
            "\u{5de}\u{5e2}\u{5e8}\u{5db}\u{5ea}",        // מערכת (system)
            "\u{5de}\u{5e9}\u{5ea}\u{5de}\u{5e9}",        // משתמש (user)
            "\u{5e7}\u{5d5}\u{5d1}\u{5e5}",               // קובץ (file)
            "\u{5e4}\u{5e8}\u{5d5}\u{5d9}\u{5e7}\u{5d8}", // פרויקט (project)
            "\u{5e9}\u{5d9}\u{5e8}\u{5d5}\u{5ea}",        // שירות (service)
            "\u{5d0}\u{5d1}\u{5d8}\u{5d7}\u{5d4}",        // אבטחה (security)
            "\u{5e8}\u{5e9}\u{5ea}",                      // רשת (network)
            "\u{5d3}\u{5d5}\u{5d7}",                      // דוח (report)
            "\u{5ea}\u{5d4}\u{5dc}\u{5d9}\u{5da}",        // תהליך (process)
            "\u{5d4}\u{5d5}\u{5d3}\u{5e2}\u{5d4}",        // הודעה (message)
            "\u{5de}\u{5e9}\u{5d9}\u{5de}\u{5d4}",        // משימה (task)
            "\u{5e2}\u{5d3}\u{5db}\u{5d5}\u{5df}",        // עדכון (update)
        ],
        // Arabic
        "ar-sa" | "ar-ae" | "eg" => &[
            "\u{648}\u{635}\u{648}\u{644}",               // وصول (access)
            "\u{628}\u{64a}\u{627}\u{646}\u{627}\u{62a}", // بيانات (data)
            "\u{62e}\u{637}\u{623}",                      // خطأ (error)
            "\u{643}\u{644}\u{645}\u{629} \u{645}\u{631}\u{648}\u{631}", // كلمة مرور (password)
            "\u{646}\u{638}\u{627}\u{645}",               // نظام (system)
            "\u{645}\u{633}\u{62a}\u{62e}\u{62f}\u{645}", // مستخدم (user)
            "\u{645}\u{644}\u{641}",                      // ملف (file)
            "\u{645}\u{634}\u{631}\u{648}\u{639}",        // مشروع (project)
            "\u{62e}\u{62f}\u{645}\u{629}",               // خدمة (service)
            "\u{623}\u{645}\u{627}\u{646}",               // أمان (security)
            "\u{634}\u{628}\u{643}\u{629}",               // شبكة (network)
            "\u{62a}\u{642}\u{631}\u{64a}\u{631}",        // تقرير (report)
            "\u{639}\u{645}\u{644}\u{64a}\u{629}",        // عملية (process)
            "\u{631}\u{633}\u{627}\u{644}\u{629}",        // رسالة (message)
            "\u{645}\u{647}\u{645}\u{629}",               // مهمة (task)
            "\u{62a}\u{62d}\u{62f}\u{64a}\u{62b}",        // تحديث (update)
        ],
        "tr" => &[
            "eri\u{15f}im",
            "hesap",
            "hata",
            "mesaj",
            "parola",
            "proje",
            "rapor",
            "sistem",
            "kullan\u{131}c\u{131}",
            "dosya",
            "g\u{fc}venlik",
            "hizmet",
        ],
        "pl" => &[
            "dost\u{119}p",
            "konto",
            "dane",
            "has\u{142}o",
            "proces",
            "projekt",
            "raport",
            "system",
            "u\u{17c}ytkownik",
            "zadanie",
            "plik",
            "us\u{142}uga",
        ],
        _ => &[
            "lorem",
            "ipsum",
            "dolor",
            "sit",
            "amet",
            "consectetur",
            "adipiscing",
            "elit",
            "sed",
            "do",
            "eiusmod",
            "tempor",
            "incididunt",
            "labore",
            "dolore",
            "magna",
            "aliqua",
            "enim",
            "minim",
            "veniam",
            "quis",
            "nostrud",
            "exercitation",
            "ullamco",
            "laboris",
            "nisi",
            "aliquip",
            "commodo",
            "consequat",
            "duis",
        ],
    }
}
