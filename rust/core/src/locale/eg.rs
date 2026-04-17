use super::{City, Locale, NameOrder};

pub static LOCALE: Locale = Locale {
    code: "eg",
    name_order: NameOrder::PatronymicMiddle,    first_names: &[
        "Mohamed", "Ahmed", "Mahmoud", "Ali", "Hassan", "Omar", "Mostafa", "Ibrahim", "Youssef",
        "Khaled", "Amr", "Tamer", "Hossam", "Karim", "Sherif", "Wael", "Tarek", "Sameh", "Ayman",
        "Hesham", "Adel", "Nabil", "Samir", "Essam", "Ashraf", "Fatima", "Nour", "Mariam", "Sara",
        "Hana", "Yasmin", "Dina", "Aya", "Rania", "Mona", "Heba", "Eman", "Abeer", "Ghada", "Noha",
        "Amira", "Samar", "Laila", "Soha", "Dalal", "Nagwa", "Nermin", "Rehab", "Salma", "Dalia",
        "Mai", "Omnia", "Yara",
        // additional first names
        "Abdallah", "Hazem", "Walid", "Ramy", "Bassem", "Ehab", "Magdy", "Osama", "Emad", "Gamal",
        "Ragab", "Sayed", "Mazen", "Akram", "Hatem", "Nader", "Medhat", "Mohsen", "Alaa", "Reda",
        "Abdel-Aziz", "Badr", "Fady", "George", "Mina", "Bishoy", "Hany", "Ayman", "Atef", "Fouad",
        "Ziad", "Hamdi", "Rafik", "Shady", "Yasser", "Saeed", "Lotfy", "Sobhy", "Wagdy", "Zakaria",
        "Habiba", "Farida", "Malak", "Nada", "Shahd", "Rowan", "Lamia", "Wessam", "Naglaa", "Rawda",
        "Doaa", "Hanaa", "Asmaa", "Sawsan", "Nahla", "Samia", "Aida", "Afaf", "Suzy", "Mervat",
        "Ingy", "Hadeer", "Nesma", "Shaimaa", "Esraa", "Riham", "Basma", "Reem", "Lubna", "Nashwa",
        "Marwa", "Manar", "Hala", "Maisa", "Menna", "Nourhan", "Toqa", "Raghdaa", "Rabab", "Karima",
        "Israa", "Faten", "Sahar", "Howayda", "Hayam", "Ola", "Wafaa", "Tahani", "Maysoon", "Gehan",
        "Abir", "Neveen", "Sherihan", "Lamiaa", "Zahra", "Sandos", "Toka",
        "Bahaa", "Diaa", "Emad", "Fathi", "Gamal", "Haytham", "Ihab",
        "Kamal", "Lutfi", "Medhat", "Nabil", "Ramadan", "Tamer",
    ],
    first_names_common: 0,
    last_names: &[
        "Ibrahim", "Mahmoud", "Hassan", "Ali", "Ahmed", "Mohamed", "Abdel-Rahman", "Abdel-Fattah",
        "El-Sayed", "Farouk", "Nasser", "Saeed", "Osman", "Khalil", "Salem", "Youssef", "Ismail",
        "Helmy", "Ragab", "Gamal", "Shafik", "Moustafa", "Habib", "Mansour", "El-Masry", "Soliman",
        "Hamed", "Shaker", "Barakat", "El-Din", "Tawfik", "Fouad",
        // additional last names
        "Abdel-Nour", "Amin", "Anwar", "Ashour", "Attia", "Awad", "Badawy", "Bakr", "Dawood",
        "Diab", "El-Banna", "El-Gazzar", "El-Hadidy", "El-Hawary", "El-Husseiny", "El-Kady",
        "El-Maghraby", "El-Nabawy", "El-Naggar", "El-Rashidy", "El-Sawy", "El-Shamy", "El-Sherif",
        "El-Taher", "El-Zayat", "Fahmy", "Fawzy", "Gaber", "Ghaly", "Gomaa", "Hamdy", "Haroun",
        "Hegazy", "Hussein", "Kamel", "Labib", "Lotfy", "Maged", "Mahfouz", "Metwally", "Morsi",
        "Mourad", "Naguib", "Nassar", "Radwan", "Ragheb", "Rashed", "Rizk", "Sabry", "Sadek",
        "Shehata", "Sobhy", "Suleiman", "Tantawy", "Wahba", "Yousry", "Zaki", "Zein",
        "El-Adl", "El-Assal", "El-Deeb", "El-Demerdash", "El-Fiky", "El-Garhy", "El-Gendy",
        "El-Guindy", "El-Halawany", "El-Hennawy", "El-Husseini", "El-Kholi", "El-Leithy",
        "El-Mahdi", "El-Mansy", "El-Meligy", "El-Menshawy", "El-Miniawi", "El-Missiry",
        "El-Moghazy", "El-Mohammady", "El-Morsy", "El-Nady", "El-Nahas", "El-Nokaly",
        "El-Omda", "El-Saadany", "El-Saba", "El-Safty", "El-Sawi", "El-Shafei", "El-Sharkawy",
        "El-Sharqawy", "El-Shimy", "El-Shinnawi", "El-Tawil", "El-Toukhy", "El-Waziry",
        "Abdel-Aziz", "Abdel-Gawad", "Abdel-Halim", "Abdel-Hamid", "Abdel-Karim",
        "Abdel-Latif", "Abdel-Meguid", "Abdel-Moneim", "Abdel-Naby", "Abdel-Salam",
        "Abdel-Wahab", "Aboul-Fotouh", "Bastawisy", "Darwish", "Ezzat", "Fathi", "Fikry",
        "Ghannam", "Hashem", "Hendy", "Kandil", "Karam", "Magdi", "Makram", "Mikhail",
        "Ramzy", "Raslan", "Refaat", "Sami", "Seif",
    ],
    last_names_common: 0,
    domains: &[
        "vodafone.com.eg",
        "orange.com.eg",
        "etisalat.eg",
        "cib.com.eg",
        "nbe.com.eg",
        "banquemisr.com",
        "fawry.com",
        "jumia.com.eg",
        "souq.com",
        "elwatannews.com",
        "gmail.com",
        "outlook.com",
        // additional domains
        "ahram.org.eg",
        "masrawy.com",
        "youm7.com",
        "cairo24.com",
        "filgoal.com",
        "wuzzuf.net",
        "otlob.com",
        "alexbank.com",
        "aaib.com",
        "qnb.com.eg",
        "fab.com.eg",
        "elaraby-group.com",
        "edita.com.eg",
        "efg-hermes.com",
        "orascom.com",
        "elsewedy.com",
        "swvl.com",
        "vezeeta.com",
        "elmenus.com",
        "btech.com",
        "raya.com.eg",
        "link.net",
        "tedata.net",
        "we.com.eg",
        "nilesat.com.eg",
        "egypt.gov.eg",
        "yahoo.com",
    ],
    domains_common: 0,
    companies: &[
        "Commercial International Bank",
        "National Bank of Egypt",
        "Banque Misr",
        "Vodafone Egypt",
        "Orange Egypt",
        "Etisalat Misr",
        "Orascom",
        "EFG Hermes",
        "Fawry",
        "Elsewedy Electric",
        "Telecom Egypt",
        "Talaat Moustafa Group",
        // additional companies
        "QNB Al Ahli",
        "Arab African International Bank",
        "Alexandria Bank",
        "Banque du Caire",
        "HSBC Egypt",
        "Edita Food Industries",
        "El Araby Group",
        "Raya Holding",
        "Juhayna",
        "Oriental Weavers",
        "EgyptAir",
        "SWVL",
        "Vezeeta",
        "Instabug",
        "Paymob",
        "Swvl",
        "Carbon",
        "Si-Ware Systems",
        "Sarmady",
        "Palm Hills Developments",
        "Emaar Misr",
        "Pioneers Holding",
        "Ezz Steel",
        "Abu Qir Fertilizers",
        "Misr Spinning and Weaving",
        "Misr Insurance",
        "Egyptian Refining",
        "Heliopolis Housing",
    ],
    cities: &[
        City { name: "Cairo", region: "Cairo", postal: "11511", lat: 30.04, lon: 31.24, tz: "Africa/Cairo" },
        City { name: "Alexandria", region: "Alexandria", postal: "21500", lat: 31.2, lon: 29.92, tz: "Africa/Cairo" },
        City { name: "Giza", region: "Giza", postal: "12511", lat: 30.01, lon: 31.21, tz: "Africa/Cairo" },
        City { name: "Shubra El-Kheima", region: "Qalyubia", postal: "13711", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Port Said", region: "Port Said", postal: "42511", lat: 31.26, lon: 32.3, tz: "Africa/Cairo" },
        City { name: "Suez", region: "Suez", postal: "43511", lat: 29.97, lon: 32.55, tz: "Africa/Cairo" },
        City { name: "Luxor", region: "Luxor", postal: "85511", lat: 25.69, lon: 32.64, tz: "Africa/Cairo" },
        City { name: "Aswan", region: "Aswan", postal: "81511", lat: 24.09, lon: 32.9, tz: "Africa/Cairo" },
        City { name: "Mansoura", region: "Dakahlia", postal: "35511", lat: 31.04, lon: 31.38, tz: "Africa/Cairo" },
        City { name: "Tanta", region: "Gharbia", postal: "31511", lat: 30.79, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Ismailia", region: "Ismailia", postal: "41511", lat: 30.6, lon: 32.27, tz: "Africa/Cairo" },
        City { name: "Zagazig", region: "Sharqia", postal: "44511", lat: 30.59, lon: 31.5, tz: "Africa/Cairo" },
        // additional cities
        City { name: "Asyut", region: "Asyut", postal: "71511", lat: 27.18, lon: 31.18, tz: "Africa/Cairo" },
        City { name: "Faiyum", region: "Faiyum", postal: "63511", lat: 29.31, lon: 30.84, tz: "Africa/Cairo" },
        City { name: "Beni Suef", region: "Beni Suef", postal: "62511", lat: 29.07, lon: 31.1, tz: "Africa/Cairo" },
        City { name: "Minya", region: "Minya", postal: "61511", lat: 28.1, lon: 30.75, tz: "Africa/Cairo" },
        City { name: "Sohag", region: "Sohag", postal: "82511", lat: 26.56, lon: 31.69, tz: "Africa/Cairo" },
        City { name: "Qena", region: "Qena", postal: "83511", lat: 26.16, lon: 32.73, tz: "Africa/Cairo" },
        City { name: "Damietta", region: "Damietta", postal: "34511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Kafr El Sheikh", region: "Kafr El Sheikh", postal: "33511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Banha", region: "Qalyubia", postal: "13511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Hurghada", region: "Red Sea", postal: "84511", lat: 27.26, lon: 33.81, tz: "Africa/Cairo" },
        City { name: "Sharm El Sheikh", region: "South Sinai", postal: "46619", lat: 27.92, lon: 34.33, tz: "Africa/Cairo" },
        City { name: "Marsa Matruh", region: "Matruh", postal: "51511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "El Mahalla El Kubra", region: "Gharbia", postal: "31951", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Shibin El Kom", region: "Monufia", postal: "32511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "Damanhour", region: "Beheira", postal: "22511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "El Arish", region: "North Sinai", postal: "45611", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
        City { name: "6th of October City", region: "Giza", postal: "12573", lat: 29.94, lon: 30.92, tz: "Africa/Cairo" },
        City { name: "New Cairo", region: "Cairo", postal: "11835", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" },
    ],
    streets: &[
        "Tahrir Square",
        "Talaat Harb Street",
        "Kasr El Nil Street",
        "26th of July Street",
        "Corniche El Nil",
        "Salah Salem Road",
        "El Haram Street",
        "Gameat El Dowal Street",
        "Makram Ebeid Street",
        "Mostafa El Nahas Street",
        "Abbas El Akkad Street",
        "Ramses Street",
        // additional streets
        "Mohamed Farid Street",
        "Emad El Din Street",
        "Port Said Street",
        "El Galaa Street",
        "El Thawra Street",
        "Ahmed Orabi Street",
        "El Tayaran Street",
        "Youssef Abbas Street",
        "El Merghany Street",
        "El Nozha Street",
        "El Hegaz Street",
        "Shehab Street",
        "Lebanon Street",
        "Syria Street",
        "El Batal Ahmed Abdel-Aziz Street",
        "Abdel-Khalek Sarwat Street",
        "Nubar Street",
        "Mohamed Ali Street",
        "El Muizz Street",
        "El Azhar Street",
        "Suez Canal Street",
        "El Nasr Road",
        "Autostrad Road",
        "Ring Road",
        "El Wahat Road",
        "Sphinx Square",
        "Faisal Street",
        "El Ahram Street",
        "Saad Zaghloul Street",
        "El Gomhoreya Street",
    ],
    native_first_names: Some(&[
        "\u{0645}\u{062d}\u{0645}\u{062f}", // محمد
        "\u{0623}\u{062d}\u{0645}\u{062f}", // أحمد
        "\u{0645}\u{062d}\u{0645}\u{0648}\u{062f}", // محمود
        "\u{0639}\u{0644}\u{064a}", // علي
        "\u{062d}\u{0633}\u{0646}", // حسن
        "\u{0639}\u{0645}\u{0631}", // عمر
        "\u{0645}\u{0635}\u{0637}\u{0641}\u{0649}", // مصطفى
        "\u{0625}\u{0628}\u{0631}\u{0627}\u{0647}\u{064a}\u{0645}", // إبراهيم
        "\u{064a}\u{0648}\u{0633}\u{0641}", // يوسف
        "\u{062e}\u{0627}\u{0644}\u{062f}", // خالد
        "\u{0641}\u{0627}\u{0637}\u{0645}\u{0629}", // فاطمة
        "\u{0646}\u{0648}\u{0631}", // نور
        "\u{0645}\u{0631}\u{064a}\u{0645}", // مريم
        "\u{0633}\u{0627}\u{0631}\u{0629}", // سارة
        "\u{0647}\u{0646}\u{0627}", // هنا
        "\u{064a}\u{0627}\u{0633}\u{0645}\u{064a}\u{0646}", // ياسمين
        "\u{062f}\u{064a}\u{0646}\u{0627}", // دينا
        "\u{0622}\u{064a}\u{0629}", // آية
        "\u{0631}\u{0627}\u{0646}\u{064a}\u{0627}", // رانيا
        "\u{0645}\u{0646}\u{0649}", // منى
        "\u{0639}\u{0645}\u{0631}\u{0648}", // عمرو
        "\u{062a}\u{0627}\u{0645}\u{0631}", // تامر
        "\u{062d}\u{0633}\u{0627}\u{0645}", // حسام
        "\u{0643}\u{0631}\u{064a}\u{0645}", // كريم
        "\u{0634}\u{0631}\u{064a}\u{0641}", // شريف
    ]),
    native_last_names: Some(&[
        "\u{0625}\u{0628}\u{0631}\u{0627}\u{0647}\u{064a}\u{0645}", // إبراهيم
        "\u{0645}\u{062d}\u{0645}\u{0648}\u{062f}", // محمود
        "\u{062d}\u{0633}\u{0646}", // حسن
        "\u{0639}\u{0644}\u{064a}", // علي
        "\u{0623}\u{062d}\u{0645}\u{062f}", // أحمد
        "\u{0645}\u{062d}\u{0645}\u{062f}", // محمد
        "\u{0639}\u{0628}\u{062f}\u{0627}\u{0644}\u{0631}\u{062d}\u{0645}\u{0646}", // عبدالرحمن
        "\u{0639}\u{0628}\u{062f}\u{0627}\u{0644}\u{0641}\u{062a}\u{0627}\u{062d}", // عبدالفتاح
        "\u{0627}\u{0644}\u{0633}\u{064a}\u{062f}", // السيد
        "\u{0641}\u{0627}\u{0631}\u{0648}\u{0642}", // فاروق
        "\u{0646}\u{0627}\u{0635}\u{0631}", // ناصر
        "\u{0633}\u{0639}\u{064a}\u{062f}", // سعيد
        "\u{0639}\u{062b}\u{0645}\u{0627}\u{0646}", // عثمان
        "\u{062e}\u{0644}\u{064a}\u{0644}", // خليل
        "\u{0633}\u{0627}\u{0644}\u{0645}", // سالم
        "\u{064a}\u{0648}\u{0633}\u{0641}", // يوسف
        "\u{0625}\u{0633}\u{0645}\u{0627}\u{0639}\u{064a}\u{0644}", // إسماعيل
        "\u{062d}\u{0644}\u{0645}\u{064a}", // حلمي
        "\u{0631}\u{062c}\u{0628}", // رجب
        "\u{062c}\u{0645}\u{0627}\u{0644}", // جمال
    ]),
    native_cities: Some(&[
        City { name: "\u{0627}\u{0644}\u{0642}\u{0627}\u{0647}\u{0631}\u{0629}", region: "Cairo", postal: "11511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // القاهرة
        City { name: "\u{0627}\u{0644}\u{0625}\u{0633}\u{0643}\u{0646}\u{062f}\u{0631}\u{064a}\u{0629}", region: "Alexandria", postal: "21500", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // الإسكندرية
        City { name: "\u{0627}\u{0644}\u{062c}\u{064a}\u{0632}\u{0629}", region: "Giza", postal: "12511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // الجيزة
        City { name: "\u{0634}\u{0628}\u{0631}\u{0627} \u{0627}\u{0644}\u{062e}\u{064a}\u{0645}\u{0629}", region: "Qalyubia", postal: "13711", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // شبرا الخيمة
        City { name: "\u{0628}\u{0648}\u{0631}\u{0633}\u{0639}\u{064a}\u{062f}", region: "Port Said", postal: "42511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // بورسعيد
        City { name: "\u{0627}\u{0644}\u{0633}\u{0648}\u{064a}\u{0633}", region: "Suez", postal: "43511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // السويس
        City { name: "\u{0627}\u{0644}\u{0623}\u{0642}\u{0635}\u{0631}", region: "Luxor", postal: "85511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // الأقصر
        City { name: "\u{0623}\u{0633}\u{0648}\u{0627}\u{0646}", region: "Aswan", postal: "81511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // أسوان
        City { name: "\u{0627}\u{0644}\u{0645}\u{0646}\u{0635}\u{0648}\u{0631}\u{0629}", region: "Dakahlia", postal: "35511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // المنصورة
        City { name: "\u{0637}\u{0646}\u{0637}\u{0627}", region: "Gharbia", postal: "31511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // طنطا
        City { name: "\u{0627}\u{0644}\u{0625}\u{0633}\u{0645}\u{0627}\u{0639}\u{064a}\u{0644}\u{064a}\u{0629}", region: "Ismailia", postal: "41511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // الإسماعيلية
        City { name: "\u{0627}\u{0644}\u{0632}\u{0642}\u{0627}\u{0632}\u{064a}\u{0642}", region: "Sharqia", postal: "44511", lat: 30.0, lon: 31.0, tz: "Africa/Cairo" }, // الزقازيق
    ]),
    native_streets: Some(&[
        "\u{0645}\u{064a}\u{062f}\u{0627}\u{0646} \u{0627}\u{0644}\u{062a}\u{062d}\u{0631}\u{064a}\u{0631}", // ميدان التحرير
        "\u{0634}\u{0627}\u{0631}\u{0639} \u{0637}\u{0644}\u{0639}\u{062a} \u{062d}\u{0631}\u{0628}", // شارع طلعت حرب
        "\u{0634}\u{0627}\u{0631}\u{0639} \u{0642}\u{0635}\u{0631} \u{0627}\u{0644}\u{0646}\u{064a}\u{0644}", // شارع قصر النيل
        "\u{0634}\u{0627}\u{0631}\u{0639} \u{0662}\u{0666} \u{064a}\u{0648}\u{0644}\u{064a}\u{0648}", // شارع ٢٦ يوليو
        "\u{0643}\u{0648}\u{0631}\u{0646}\u{064a}\u{0634} \u{0627}\u{0644}\u{0646}\u{064a}\u{0644}", // كورنيش النيل
    ]),
};
