use super::{City, Locale, NameOrder};

pub static LOCALE: Locale = Locale {
    code: "pk",
    name_order: NameOrder::Western,    first_names: &[
        "Muhammad", "Ahmed", "Ali", "Hassan", "Usman", "Bilal", "Umar", "Hamza", "Asad", "Faisal",
        "Imran", "Kamran", "Kashif", "Naveed", "Rehan", "Waqas", "Shoaib", "Taha", "Owais", "Yasir",
        "Rizwan", "Arslan", "Adnan", "Junaid", "Amir", "Shahid", "Tariq", "Nadeem", "Asif", "Farhan",
        "Zubair", "Zahid", "Saad", "Sameer", "Zainab", "Fatima", "Ayesha", "Sana", "Hira", "Maryam",
        "Nadia", "Saima", "Rabia", "Amna", "Anam", "Sara", "Mehreen", "Huma", "Saba", "Noor",
        "Kiran", "Bushra", "Iqra", "Maham", "Arooj",
        // additional first names
        "Irfan", "Salman", "Aamir", "Raza", "Waseem", "Naeem", "Akbar", "Khalid", "Mubashir",
        "Shehzad", "Zeeshan", "Haroon", "Danish", "Fahad", "Ghulam", "Habib", "Javed", "Liaquat",
        "Masood", "Nasir", "Pervaiz", "Qamar", "Rafiq", "Sajid", "Tahir", "Waheed", "Yaqoob",
        "Aftab", "Bashir", "Daud", "Ejaz", "Fayyaz", "Gulzar", "Hanif", "Ijaz", "Jamil", "Mudassar",
        "Noman", "Obaid", "Qasim", "Saqib", "Talha", "Umair", "Waleed", "Zafar", "Babar", "Ehsan",
        "Shafiq", "Atif", "Azhar", "Furqan", "Hasnain", "Muneeb", "Raheel", "Saif", "Usama",
        "Sidra", "Sumera", "Samina", "Fariha", "Lubna", "Tahira", "Naila", "Zubaida", "Rukhsana",
        "Shabana", "Nasreen", "Parveen", "Asma", "Uzma", "Kanwal", "Haleema", "Tayyaba", "Iram",
        "Saira", "Farah", "Ambreen", "Fouzia", "Ghazala", "Jaweria", "Madeeha", "Nimra", "Riffat",
        "Shaista", "Tanveer", "Wardah", "Zara", "Alina", "Hafsa", "Kinza", "Laiba", "Momina",
        "Nawal", "Ramsha", "Sadia", "Tooba", "Yumna", "Areeba", "Dur-e-Nayab",
    ],
    first_names_common: 0,
    last_names: &[
        "Khan", "Ahmed", "Ali", "Hussain", "Malik", "Sheikh", "Butt", "Chaudhry", "Qureshi",
        "Siddiqui", "Akhtar", "Aslam", "Iqbal", "Javed", "Raza", "Mehmood", "Shah", "Abbasi",
        "Bhatti", "Hashmi", "Mirza", "Niazi", "Rizvi", "Saeed", "Anwar", "Baig", "Dar", "Gilani",
        "Haider", "Haq", "Mushtaq", "Naqvi", "Noor", "Rehman", "Sharif", "Zaidi", "Awan", "Baloch",
        "Afridi", "Yousuf",
        // additional last names
        "Arain", "Bajwa", "Chohan", "Durrani", "Farooqi", "Ghauri", "Gondal", "Hayat", "Jamali",
        "Junejo", "Kazi", "Khattak", "Leghari", "Lodhi", "Makhdoom", "Marwat", "Mengal", "Memon",
        "Mughal", "Paracha", "Pirzada", "Rajput", "Rana", "Sethi", "Soomro", "Suleman", "Syed",
        "Tahir", "Tareen", "Tiwana", "Usmani", "Wains", "Wazir", "Yousafzai", "Zaman",
        "Ansari", "Bukhari", "Chishti", "Farooqui", "Gardezi", "Hameed", "Kazmi", "Khawaja",
        "Lakhani", "Moosani", "Nadeem", "Pasha", "Qazi", "Sarwar", "Sipra", "Tanveer",
        "Warriach", "Asghar", "Bangash", "Dawood", "Ehsan", "Fazal", "Ghaffar", "Hanif",
        "Israr", "Jalil", "Karim", "Latif", "Masih", "Nauman", "Qadir", "Rafiq",
        "Sabir", "Talib", "Umar", "Virk", "Warraich", "Yaqub", "Zubairi",
        "Bhutto", "Daultana", "Girgani", "Hotiana", "Jatoi", "Khar", "Langah",
        "Mazari", "Noon", "Orakzai", "Palijo", "Qaisrani", "Rind", "Samoo",
        "Talpur", "Umrani", "Wassan", "Zardari",
        "Bokhari", "Chandio", "Domki", "Gabol", "Halepota", "Khoso", "Lashari",
        "Magsi", "Notezai", "Phulpoto", "Rashdi", "Solangi", "Tunio",
        "Arain", "Balti", "Durrani", "Gandapur", "Jadoon",
        "Khattak", "Lodhi", "Mengal", "Niazi", "Paracha",
    ],
    last_names_common: 0,
    domains: &[
        "hbl.com",
        "ubl.com.pk",
        "mcb.com.pk",
        "jazz.com.pk",
        "telenor.com.pk",
        "zong.com.pk",
        "daraz.pk",
        "careem.com",
        "foodpanda.pk",
        "geo.tv",
        "gmail.com",
        "outlook.com",
        // additional domains
        "ptcl.com.pk",
        "nayatel.com",
        "stormfiber.com",
        "dawn.com",
        "arynews.tv",
        "brecorder.com",
        "express.pk",
        "thenews.com.pk",
        "ufone.com",
        "scbpk.com",
        "nbp.com.pk",
        "alfalahbank.com",
        "meezanbank.com",
        "askaribank.com.pk",
        "bfrn.com.pk",
        "jsbl.com",
        "silkbank.com.pk",
        "jazzcash.com.pk",
        "easypaisa.com.pk",
        "zameen.com",
        "olx.com.pk",
        "rozee.pk",
        "kaymu.pk",
        "priceoye.pk",
        "telemart.pk",
        "pakwheels.com",
        "yahoo.com",
    ],
    domains_common: 0,
    companies: &[
        "Habib Bank",
        "UBL",
        "MCB Bank",
        "Jazz",
        "Telenor Pakistan",
        "PTCL",
        "Engro",
        "Lucky Cement",
        "Fauji Fertilizer",
        "Oil and Gas Development",
        "Pakistan State Oil",
        "Nestle Pakistan",
        // additional companies
        "National Bank of Pakistan",
        "Allied Bank",
        "Askari Bank",
        "Bank Alfalah",
        "Meezan Bank",
        "Faysal Bank",
        "Bank Al Habib",
        "Sui Northern Gas",
        "Sui Southern Gas",
        "Hub Power Company",
        "K-Electric",
        "Pakistan Tobacco",
        "Unilever Pakistan",
        "Indus Motor",
        "Pak Suzuki",
        "Honda Atlas",
        "Packages Limited",
        "Interloop",
        "Sapphire Textile",
        "Gul Ahmed Textile",
        "Nishat Mills",
        "Kohinoor Textile",
        "Attock Petroleum",
        "Mari Petroleum",
        "Pakistan Petroleum",
        "Dawood Hercules",
        "ICI Pakistan",
    ],
    cities: &[
        City { name: "Karachi", region: "Sindh", postal: "74000", lat: 24.86, lon: 67.01, tz: "Asia/Karachi" },
        City { name: "Lahore", region: "Punjab", postal: "54000", lat: 31.55, lon: 74.35, tz: "Asia/Karachi" },
        City { name: "Islamabad", region: "ICT", postal: "44000", lat: 33.69, lon: 73.04, tz: "Asia/Karachi" },
        City { name: "Rawalpindi", region: "Punjab", postal: "46000", lat: 33.6, lon: 73.05, tz: "Asia/Karachi" },
        City { name: "Faisalabad", region: "Punjab", postal: "38000", lat: 31.42, lon: 73.08, tz: "Asia/Karachi" },
        City { name: "Multan", region: "Punjab", postal: "60000", lat: 30.2, lon: 71.46, tz: "Asia/Karachi" },
        City { name: "Peshawar", region: "KPK", postal: "25000", lat: 34.01, lon: 71.58, tz: "Asia/Karachi" },
        City { name: "Quetta", region: "Balochistan", postal: "87300", lat: 30.18, lon: 66.99, tz: "Asia/Karachi" },
        City { name: "Sialkot", region: "Punjab", postal: "51310", lat: 32.5, lon: 74.53, tz: "Asia/Karachi" },
        City { name: "Gujranwala", region: "Punjab", postal: "52250", lat: 32.16, lon: 74.19, tz: "Asia/Karachi" },
        City { name: "Hyderabad", region: "Sindh", postal: "71000", lat: 25.4, lon: 68.37, tz: "Asia/Karachi" },
        City { name: "Abbottabad", region: "KPK", postal: "22010", lat: 34.15, lon: 73.21, tz: "Asia/Karachi" },
        // additional cities
        City { name: "Bahawalpur", region: "Punjab", postal: "63100", lat: 29.39, lon: 71.68, tz: "Asia/Karachi" },
        City { name: "Sargodha", region: "Punjab", postal: "40100", lat: 32.08, lon: 72.67, tz: "Asia/Karachi" },
        City { name: "Sukkur", region: "Sindh", postal: "65200", lat: 27.7, lon: 68.86, tz: "Asia/Karachi" },
        City { name: "Larkana", region: "Sindh", postal: "77150", lat: 27.56, lon: 68.21, tz: "Asia/Karachi" },
        City { name: "Mardan", region: "KPK", postal: "23200", lat: 34.2, lon: 72.05, tz: "Asia/Karachi" },
        City { name: "Mingora", region: "KPK", postal: "19130", lat: 34.78, lon: 72.36, tz: "Asia/Karachi" },
        City { name: "Rahim Yar Khan", region: "Punjab", postal: "64200", lat: 28.42, lon: 70.3, tz: "Asia/Karachi" },
        City { name: "Sahiwal", region: "Punjab", postal: "57000", lat: 30.67, lon: 73.11, tz: "Asia/Karachi" },
        City { name: "Okara", region: "Punjab", postal: "56300", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" },
        City { name: "Wah Cantt", region: "Punjab", postal: "47040", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" },
        City { name: "Dera Ghazi Khan", region: "Punjab", postal: "32200", lat: 30.06, lon: 70.64, tz: "Asia/Karachi" },
        City { name: "Mirpur", region: "AJK", postal: "10250", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" },
        City { name: "Nawabshah", region: "Sindh", postal: "67450", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" },
        City { name: "Kasur", region: "Punjab", postal: "55050", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" },
        City { name: "Sheikhupura", region: "Punjab", postal: "39350", lat: 31.71, lon: 73.99, tz: "Asia/Karachi" },
        City { name: "Jhang", region: "Punjab", postal: "35200", lat: 31.28, lon: 72.32, tz: "Asia/Karachi" },
    ],
    streets: &[
        "Mall Road",
        "Shahrah-e-Faisal",
        "M.A. Jinnah Road",
        "I.I. Chundrigar Road",
        "Tariq Road",
        "Zamzama Boulevard",
        "Jinnah Avenue",
        "Constitution Avenue",
        "Murree Road",
        "GT Road",
        "Clifton Road",
        "University Road",
        // additional streets
        "Shahrah-e-Quaid-e-Azam",
        "Davis Road",
        "Egerton Road",
        "Ferozpur Road",
        "Garden Road",
        "Jail Road",
        "Kashmir Road",
        "Lawrence Road",
        "Main Boulevard",
        "Multan Road",
        "Nazimabad Road",
        "Rashid Minhas Road",
        "Shaheed-e-Millat Road",
        "Shahra-e-Liaquat",
        "Sir Syed Road",
        "Stadium Road",
        "Tipu Sultan Road",
        "Wahdat Road",
        "Zaibunnisa Street",
        "Abdullah Haroon Road",
        "Aga Khan Road",
        "Ataturk Avenue",
        "Blue Area",
        "Chaklala Road",
        "Chandni Chowk",
        "Circular Road",
        "Empress Road",
        "Fatima Jinnah Road",
        "Khayaban-e-Iqbal",
        "Khayaban-e-Jinnah",
    ],
    native_first_names: Some(&[
        "\u{0645}\u{062d}\u{0645}\u{062f}", // محمد
        "\u{0627}\u{062d}\u{0645}\u{062f}", // احمد
        "\u{0639}\u{0644}\u{06cc}", // علی
        "\u{062d}\u{0633}\u{0646}", // حسن
        "\u{0639}\u{062b}\u{0645}\u{0627}\u{0646}", // عثمان
        "\u{0628}\u{0644}\u{0627}\u{0644}", // بلال
        "\u{0639}\u{0645}\u{0631}", // عمر
        "\u{062d}\u{0645}\u{0632}\u{06c1}", // حمزہ
        "\u{0632}\u{06cc}\u{0646}\u{0628}", // زینب
        "\u{0641}\u{0627}\u{0637}\u{0645}\u{06c1}", // فاطمہ
        "\u{0639}\u{0627}\u{0626}\u{0634}\u{06c1}", // عائشہ
        "\u{062b}\u{0646}\u{0627}", // ثنا
        "\u{062d}\u{0631}\u{0627}", // حرا
        "\u{0645}\u{0631}\u{06cc}\u{0645}", // مریم
        "\u{0627}\u{0633}\u{062f}", // اسد
        "\u{0641}\u{06cc}\u{0635}\u{0644}", // فیصل
        "\u{0639}\u{0645}\u{0631}\u{0627}\u{0646}", // عمران
        "\u{06a9}\u{0627}\u{0645}\u{0631}\u{0627}\u{0646}", // کامران
        "\u{06a9}\u{0627}\u{0634}\u{0641}", // کاشف
        "\u{0646}\u{0648}\u{06cc}\u{062f}", // نوید
        "\u{0646}\u{0627}\u{062f}\u{06cc}\u{06c1}", // نادیہ
        "\u{0635}\u{0627}\u{0626}\u{0645}\u{06c1}", // صائمہ
        "\u{0631}\u{0627}\u{0628}\u{0639}\u{06c1}", // رابعہ
        "\u{0622}\u{0645}\u{0646}\u{06c1}", // آمنہ
        "\u{0627}\u{0646}\u{0639}\u{0645}", // انعم
        "\u{0633}\u{0627}\u{0631}\u{06c1}", // سارہ
    ]),
    native_last_names: Some(&[
        "\u{062e}\u{0627}\u{0646}", // خان
        "\u{0627}\u{062d}\u{0645}\u{062f}", // احمد
        "\u{0639}\u{0644}\u{06cc}", // علی
        "\u{062d}\u{0633}\u{06cc}\u{0646}", // حسین
        "\u{0645}\u{0644}\u{06a9}", // ملک
        "\u{0634}\u{06cc}\u{062e}", // شیخ
        "\u{0628}\u{0679}", // بٹ
        "\u{0686}\u{0648}\u{062f}\u{06be}\u{0631}\u{06cc}", // چودھری
        "\u{0642}\u{0631}\u{06cc}\u{0634}\u{06cc}", // قریشی
        "\u{0635}\u{062f}\u{06cc}\u{0642}\u{06cc}", // صدیقی
        "\u{0627}\u{062e}\u{062a}\u{0631}", // اختر
        "\u{0627}\u{0633}\u{0644}\u{0645}", // اسلم
        "\u{0627}\u{0642}\u{0628}\u{0627}\u{0644}", // اقبال
        "\u{062c}\u{0627}\u{0648}\u{06cc}\u{062f}", // جاوید
        "\u{0631}\u{0636}\u{0627}", // رضا
        "\u{0645}\u{062d}\u{0645}\u{0648}\u{062f}", // محمود
        "\u{0634}\u{0627}\u{06c1}", // شاہ
        "\u{0639}\u{0628}\u{0627}\u{0633}\u{06cc}", // عباسی
        "\u{0628}\u{06be}\u{0679}\u{06cc}", // بھٹی
        "\u{06c1}\u{0627}\u{0634}\u{0645}\u{06cc}", // ہاشمی
    ]),
    native_cities: Some(&[
        City { name: "\u{06a9}\u{0631}\u{0627}\u{0686}\u{06cc}", region: "Sindh", postal: "74000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // کراچی
        City { name: "\u{0644}\u{0627}\u{06c1}\u{0648}\u{0631}", region: "Punjab", postal: "54000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // لاہور
        City { name: "\u{0627}\u{0633}\u{0644}\u{0627}\u{0645} \u{0622}\u{0628}\u{0627}\u{062f}", region: "ICT", postal: "44000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // اسلام آباد
        City { name: "\u{0631}\u{0627}\u{0648}\u{0644}\u{067e}\u{0646}\u{0688}\u{06cc}", region: "Punjab", postal: "46000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // راولپنڈی
        City { name: "\u{0641}\u{06cc}\u{0635}\u{0644} \u{0622}\u{0628}\u{0627}\u{062f}", region: "Punjab", postal: "38000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // فیصل آباد
        City { name: "\u{0645}\u{0644}\u{062a}\u{0627}\u{0646}", region: "Punjab", postal: "60000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // ملتان
        City { name: "\u{067e}\u{0634}\u{0627}\u{0648}\u{0631}", region: "KPK", postal: "25000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // پشاور
        City { name: "\u{06a9}\u{0648}\u{0626}\u{0679}\u{06c1}", region: "Balochistan", postal: "87300", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // کوئٹہ
        City { name: "\u{0633}\u{06cc}\u{0627}\u{0644}\u{06a9}\u{0648}\u{0679}", region: "Punjab", postal: "51310", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // سیالکوٹ
        City { name: "\u{06af}\u{0648}\u{062c}\u{0631}\u{0627}\u{0646}\u{0648}\u{0627}\u{0644}\u{06c1}", region: "Punjab", postal: "52250", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // گوجرانوالہ
        City { name: "\u{062d}\u{06cc}\u{062f}\u{0631}\u{0622}\u{0628}\u{0627}\u{062f}", region: "Sindh", postal: "71000", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // حیدرآباد
        City { name: "\u{0627}\u{06cc}\u{0628}\u{0679} \u{0622}\u{0628}\u{0627}\u{062f}", region: "KPK", postal: "22010", lat: 30.0, lon: 70.0, tz: "Asia/Karachi" }, // ایبٹ آباد
    ]),
    native_streets: Some(&[
        "\u{0645}\u{0627}\u{0644} \u{0631}\u{0648}\u{0688}", // مال روڈ
        "\u{0634}\u{0627}\u{06c1}\u{0631}\u{0627}\u{06c1} \u{0641}\u{06cc}\u{0635}\u{0644}", // شاہراہ فیصل
        "\u{0627}\u{06cc}\u{0645} \u{0627}\u{06cc} \u{062c}\u{0646}\u{0627}\u{062d} \u{0631}\u{0648}\u{0688}", // ایم اے جناح روڈ
        "\u{0622}\u{0626}\u{06cc} \u{0622}\u{0626}\u{06cc} \u{0686}\u{0646}\u{062f}\u{0631}\u{06cc}\u{06af}\u{0631} \u{0631}\u{0648}\u{0688}", // آئی آئی چندریگر روڈ
        "\u{0637}\u{0627}\u{0631}\u{0642} \u{0631}\u{0648}\u{0688}", // طارق روڈ
    ]),
};
