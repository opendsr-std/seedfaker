use super::{City, Locale, NameOrder};

pub static LOCALE: Locale = Locale {
    code: "bd",
    name_order: NameOrder::Western,    first_names: &[
        "Mohammad", "Abdul", "Muhammad", "Md", "Abu", "Syed", "Sheikh", "Kazi", "Rahim", "Karim",
        "Rahman", "Hossain", "Islam", "Ahmed", "Arif", "Fahim", "Tanvir", "Rafi", "Shakib", "Sohel",
        "Rubel", "Sumon", "Mizan", "Rony", "Akter", "Begum", "Khatun", "Sultana", "Fatima", "Nasrin",
        "Hasina", "Rashida", "Hamida", "Taslima", "Shirin", "Tania", "Ruma", "Nusrat", "Tasnim",
        "Laboni", "Nazia", "Farzana", "Sharmin", "Rehana", "Moushumi", "Shanta", "Parvin", "Salma",
        "Afsana", "Lutfun", "Rasel", "Mamun", "Masud", "Habib",
        "Imran", "Jubayer", "Kawsar", "Liton", "Monir", "Nazmul", "Obaidul", "Palash",
        "Rafiq", "Sajid", "Taher", "Uzzal", "Wahid", "Yasin", "Zahid", "Ashraf",
        "Billal", "Delwar", "Enamul", "Feroz", "Golam", "Helal", "Iqbal", "Jahangir",
        "Kamrul", "Lutfar", "Motiur", "Nazrul", "Obayed", "Parvez", "Quamrul", "Rofiqul",
        "Selim", "Toufiq", "Ujjal", "Wazed",
        "Ayesha", "Bilkis", "Chandni", "Dilara", "Esrat", "Farida", "Gulshan", "Humayra",
        "Iffat", "Jasmine", "Kamrun", "Lubna", "Mahbuba", "Nahar", "Oishee", "Poly",
        "Rabia", "Sabina", "Tahmina", "Umme", "Wahida", "Yasmin", "Zakia", "Afrin",
        "Bithi", "Chameli", "Dalia", "Eti", "Flora", "Gulnaz", "Halima", "Ishrat",
        "Jorina", "Kohinoor", "Laizu", "Morsheda", "Naima", "Roksana", "Shathi", "Tara",
        "Urmi", "Vabna", "Zannat", "Asma", "Bushra", "Daliya", "Eliza", "Firoza",
        "Habiba", "Ismat", "Jui", "Konika", "Lima", "Mitu", "Nargis",
        "Owahida", "Parina", "Razia", "Sayeda", "Tohura",
    ],
    first_names_common: 0,
    last_names: &[
        "Rahman", "Hossain", "Islam", "Ahmed", "Akter", "Begum", "Khatun", "Khan", "Ali", "Uddin",
        "Mia", "Sarker", "Chowdhury", "Das", "Alam", "Sultana", "Bhuiyan", "Hassan", "Karim",
        "Siddique", "Haque", "Talukder", "Mondal", "Roy", "Ghosh", "Biswas", "Barua", "Sen", "Saha",
        "Nath",
        "Dey", "Majumder", "Mazumder", "Paul", "Poddar", "Basak", "Mitra", "Bose",
        "Chakraborty", "Bhattacharya", "Mukherjee", "Banerjee", "Ganguly", "Dasgupta",
        "Sikder", "Prodhan", "Mollah", "Fakir", "Howlader", "Pramanik", "Halder",
        "Kundu", "Pal", "Gazi", "Sheikh", "Bepari", "Matubbar", "Hawladar",
        "Mridha", "Shikder", "Laskar", "Munshi", "Akhand", "Patwary", "Choudhury",
        "Mazumdar", "Dutta", "Sutradhar", "Karmaker", "Jalal", "Hamid", "Azad",
        "Kabir", "Rashid", "Hussain", "Mahmud", "Mostafa", "Nasir", "Hanif",
        "Latif", "Mannan", "Quddus", "Sattar", "Wahab", "Jabbar", "Razzaq",
        "Samad", "Gaffar", "Bari", "Bashar", "Hafiz", "Malek", "Matin",
        "Mojid", "Momin", "Noor", "Qadir", "Rahim", "Rauf", "Shakur",
        "Wadud", "Zahir", "Huq", "Sobhan", "Taher", "Alim", "Awal",
        "Baset", "Hakim", "Hai", "Khaleq", "Kuddus", "Mukit", "Majid",
        "Motaleb", "Rasheed", "Rouf", "Salam", "Samid", "Shahid", "Shamsul",
        "Tawab", "Wali", "Zaman", "Zia",
        "Abedin", "Chisti", "Faruk", "Gani", "Huda", "Jalil",
        "Kashem", "Liaquat", "Mobarak", "Nasiruddin", "Osmani",
        "Pasha", "Rafique", "Sadeque", "Taleb", "Ullah",
        "Wazir", "Yakub", "Zinnat", "Billah", "Dulal",
        "Ekram", "Firoz", "Golap", "Hayat",
    ],
    last_names_common: 0,
    domains: &[
        "grameenphone.com",
        "robi.com.bd",
        "banglalink.net",
        "dutchbangla.com",
        "brack.net",
        "brac.net",
        "prothomalo.com",
        "thedailystar.net",
        "waltonbd.com",
        "bkash.com",
        "gmail.com",
        "outlook.com",
        "nagad.com.bd",
        "daraz.com.bd",
        "chaldal.com",
        "shohoz.com",
        "ajkerdeal.com",
        "evaly.com.bd",
        "sheba.xyz",
        "pathao.com",
        "gpstar.com",
        "teletalk.com.bd",
        "banglatribune.com",
        "bd-pratidin.com",
        "kalerkantho.com",
        "jugantor.com",
        "bdnews24.com",
        "somewhereinblog.net",
        "squaregroup.com",
        "beximco.com",
        "islamibank.com.bd",
        "bracbank.com",
        "citybank.com.bd",
        "easternbank.com",
        "uttarabank.com.bd",
        "dbbl.com.bd",
        "bracnet.net",
        "aktel.com.bd",
        "symphony.com.bd",
    ],
    domains_common: 0,
    companies: &[
        "Grameenphone",
        "BRAC Bank",
        "Dutch-Bangla Bank",
        "Walton",
        "Square Pharmaceuticals",
        "Beximco",
        "PRAN-RFL",
        "bKash",
        "Robi Axiata",
        "Banglalink",
        "City Bank",
        "Pathao",
        "Nagad",
        "ACI Limited",
        "Renata Limited",
        "Incepta Pharmaceuticals",
        "Eastern Bank",
        "Uttara Bank",
        "Islami Bank Bangladesh",
        "Prime Bank",
        "Mutual Trust Bank",
        "IFIC Bank",
        "Singer Bangladesh",
        "Akij Group",
        "Meghna Group",
        "Bashundhara Group",
        "Summit Group",
        "Orion Group",
        "Hameem Group",
        "Ha-Meem Group",
        "Rahimafrooz",
        "Bengal Group",
        "Standard Chartered Bangladesh",
        "Teletalk",
        "Symphony Mobile",
        "Daraz Bangladesh",
        "Shohoz",
        "Chaldal",
        "GPH Ispat",
    ],
    cities: &[
        City { name: "Dhaka", region: "Dhaka", postal: "1000", lat: 23.81, lon: 90.41, tz: "Asia/Dhaka" },
        City { name: "Chittagong", region: "Chittagong", postal: "4000", lat: 22.34, lon: 91.78, tz: "Asia/Dhaka" },
        City { name: "Khulna", region: "Khulna", postal: "9000", lat: 22.84, lon: 89.54, tz: "Asia/Dhaka" },
        City { name: "Rajshahi", region: "Rajshahi", postal: "6000", lat: 24.37, lon: 88.6, tz: "Asia/Dhaka" },
        City { name: "Sylhet", region: "Sylhet", postal: "3100", lat: 24.9, lon: 91.87, tz: "Asia/Dhaka" },
        City { name: "Rangpur", region: "Rangpur", postal: "5400", lat: 25.74, lon: 89.25, tz: "Asia/Dhaka" },
        City { name: "Comilla", region: "Comilla", postal: "3500", lat: 23.46, lon: 91.18, tz: "Asia/Dhaka" },
        City { name: "Gazipur", region: "Dhaka", postal: "1700", lat: 24.0, lon: 90.42, tz: "Asia/Dhaka" },
        City { name: "Narayanganj", region: "Dhaka", postal: "1400", lat: 23.62, lon: 90.5, tz: "Asia/Dhaka" },
        City { name: "Mymensingh", region: "Mymensingh", postal: "2200", lat: 24.75, lon: 90.41, tz: "Asia/Dhaka" },
        City { name: "Barisal", region: "Barisal", postal: "8200", lat: 22.7, lon: 90.37, tz: "Asia/Dhaka" },
        City { name: "Bogra", region: "Rajshahi", postal: "5800", lat: 24.85, lon: 89.37, tz: "Asia/Dhaka" },
        City { name: "Jessore", region: "Khulna", postal: "7400", lat: 23.17, lon: 89.21, tz: "Asia/Dhaka" },
        City { name: "Dinajpur", region: "Rangpur", postal: "5200", lat: 25.63, lon: 88.64, tz: "Asia/Dhaka" },
        City { name: "Cox's Bazar", region: "Chittagong", postal: "4700", lat: 21.43, lon: 92.01, tz: "Asia/Dhaka" },
        City { name: "Brahmanbaria", region: "Chittagong", postal: "3400", lat: 23.96, lon: 91.11, tz: "Asia/Dhaka" },
        City { name: "Tangail", region: "Dhaka", postal: "1900", lat: 24.25, lon: 89.92, tz: "Asia/Dhaka" },
        City { name: "Narsingdi", region: "Dhaka", postal: "1600", lat: 23.92, lon: 90.72, tz: "Asia/Dhaka" },
        City { name: "Savar", region: "Dhaka", postal: "1340", lat: 23.86, lon: 90.27, tz: "Asia/Dhaka" },
        City { name: "Tongi", region: "Dhaka", postal: "1710", lat: 23.89, lon: 90.4, tz: "Asia/Dhaka" },
        City { name: "Nawabganj", region: "Rajshahi", postal: "6300", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Kushtia", region: "Khulna", postal: "7000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Pabna", region: "Rajshahi", postal: "6600", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Faridpur", region: "Dhaka", postal: "1800", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Noakhali", region: "Chittagong", postal: "3800", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Chandpur", region: "Chittagong", postal: "3600", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Habiganj", region: "Sylhet", postal: "3300", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Sirajganj", region: "Rajshahi", postal: "6700", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
        City { name: "Jamalpur", region: "Mymensingh", postal: "2000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" },
    ],
    streets: &[
        "Gulshan Avenue",
        "Banani Road",
        "Dhanmondi Road",
        "Mirpur Road",
        "Satmasjid Road",
        "Mohakhali Road",
        "Farmgate Road",
        "Motijheel Road",
        "Purana Paltan Lane",
        "Elephant Road",
        "Bashundhara Road",
        "Pragati Sarani",
        "Kemal Ataturk Avenue",
        "Rokeya Sarani",
        "Bir Uttam Ziaur Rahman Road",
        "Shaheed Tajuddin Ahmed Sarani",
        "New Eskaton Road",
        "Old Elephant Road",
        "Green Road",
        "Zigatola Road",
        "Shyamoli Road",
        "Lalmatia Road",
        "Uttara Sector Road",
        "Begum Rokeya Sarani",
        "Airport Road",
        "Tongi Diversion Road",
        "Dhaka Mymensingh Highway",
        "Dhaka Aricha Highway",
        "Bir Uttam Rafiqul Islam Road",
        "Kazi Nazrul Islam Avenue",
        "Bangabandhu Avenue",
        "Topkhana Road",
        "Johnson Road",
        "Nawabpur Road",
        "Islampur Road",
        "Wari Road",
        "Shahbagh Road",
        "Ramna Road",
        "Kakrail Road",
    ],
    native_first_names: Some(&[
        "\u{09ae}\u{09cb}\u{09b9}\u{09be}\u{09ae}\u{09cd}\u{09ae}\u{09a6}", // মোহাম্মদ
        "\u{0986}\u{09ac}\u{09cd}\u{09a6}\u{09c1}\u{09b2}", // আব্দুল
        "\u{09ae}\u{09c1}\u{09b9}\u{09be}\u{09ae}\u{09cd}\u{09ae}\u{09a6}", // মুহাম্মদ
        "\u{09ae}\u{09cb}\u{0983}", // মোঃ
        "\u{0986}\u{09ac}\u{09c1}", // আবু
        "\u{09b8}\u{09c8}\u{09af}\u{09bc}\u{09a6}", // সৈয়দ
        "\u{09b6}\u{09c7}\u{0996}", // শেখ
        "\u{0995}\u{09be}\u{099c}\u{09c0}", // কাজী
        "\u{09b0}\u{09b9}\u{09bf}\u{09ae}", // রহিম
        "\u{0995}\u{09b0}\u{09bf}\u{09ae}", // করিম
        "\u{09b0}\u{09b9}\u{09ae}\u{09be}\u{09a8}", // রহমান
        "\u{09b9}\u{09cb}\u{09b8}\u{09be}\u{0987}\u{09a8}", // হোসাইন
        "\u{0987}\u{09b8}\u{09b2}\u{09be}\u{09ae}", // ইসলাম
        "\u{0986}\u{09b9}\u{09ae}\u{09c7}\u{09a6}", // আহমেদ
        "\u{0986}\u{0995}\u{09cd}\u{09a4}\u{09be}\u{09b0}", // আক্তার
        "\u{09ac}\u{09c7}\u{0997}\u{09ae}", // বেগম
        "\u{0996}\u{09be}\u{09a4}\u{09c1}\u{09a8}", // খাতুন
        "\u{09b8}\u{09c1}\u{09b2}\u{09a4}\u{09be}\u{09a8}\u{09be}", // সুলতানা
        "\u{09ab}\u{09be}\u{09a4}\u{09bf}\u{09ae}\u{09be}", // ফাতিমা
        "\u{09a8}\u{09be}\u{09b8}\u{09b0}\u{09bf}\u{09a8}", // নাসরিন
    ]),
    native_last_names: Some(&[
        "\u{09b0}\u{09b9}\u{09ae}\u{09be}\u{09a8}", // রহমান
        "\u{09b9}\u{09cb}\u{09b8}\u{09be}\u{0987}\u{09a8}", // হোসাইন
        "\u{0987}\u{09b8}\u{09b2}\u{09be}\u{09ae}", // ইসলাম
        "\u{0986}\u{09b9}\u{09ae}\u{09c7}\u{09a6}", // আহমেদ
        "\u{0986}\u{0995}\u{09cd}\u{09a4}\u{09be}\u{09b0}", // আক্তার
        "\u{09ac}\u{09c7}\u{0997}\u{09ae}", // বেগম
        "\u{0996}\u{09be}\u{09a4}\u{09c1}\u{09a8}", // খাতুন
        "\u{0996}\u{09be}\u{09a8}", // খান
        "\u{0986}\u{09b2}\u{09c0}", // আলী
        "\u{0989}\u{09a6}\u{09cd}\u{09a6}\u{09bf}\u{09a8}", // উদ্দিন
        "\u{09ae}\u{09bf}\u{09af}\u{09bc}\u{09be}", // মিয়া
        "\u{09b8}\u{09b0}\u{0995}\u{09be}\u{09b0}", // সরকার
        "\u{099a}\u{09cc}\u{09a7}\u{09c1}\u{09b0}\u{09c0}", // চৌধুরী
        "\u{09a6}\u{09be}\u{09b8}", // দাস
        "\u{0986}\u{09b2}\u{09ae}", // আলম
        "\u{09b8}\u{09c1}\u{09b2}\u{09a4}\u{09be}\u{09a8}\u{09be}", // সুলতানা
        "\u{09ad}\u{09c2}\u{0981}\u{0987}\u{09af}\u{09bc}\u{09be}", // ভূঁইয়া
        "\u{09b9}\u{09be}\u{09b8}\u{09be}\u{09a8}", // হাসান
        "\u{0995}\u{09b0}\u{09bf}\u{09ae}", // করিম
        "\u{09b8}\u{09bf}\u{09a6}\u{09cd}\u{09a6}\u{09bf}\u{0995}\u{09c0}", // সিদ্দিকী
    ]),
    native_cities: Some(&[
        City { name: "\u{09a2}\u{09be}\u{0995}\u{09be}", region: "Dhaka", postal: "1000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // ঢাকা
        City { name: "\u{099a}\u{099f}\u{09cd}\u{099f}\u{0997}\u{09cd}\u{09b0}\u{09be}\u{09ae}", region: "Chittagong", postal: "4000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // চট্টগ্রাম
        City { name: "\u{0996}\u{09c1}\u{09b2}\u{09a8}\u{09be}", region: "Khulna", postal: "9000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // খুলনা
        City { name: "\u{09b0}\u{09be}\u{099c}\u{09b6}\u{09be}\u{09b9}\u{09c0}", region: "Rajshahi", postal: "6000", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // রাজশাহী
        City { name: "\u{09b8}\u{09bf}\u{09b2}\u{09c7}\u{099f}", region: "Sylhet", postal: "3100", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // সিলেট
        City { name: "\u{09b0}\u{0982}\u{09aa}\u{09c1}\u{09b0}", region: "Rangpur", postal: "5400", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // রংপুর
        City { name: "\u{0995}\u{09c1}\u{09ae}\u{09bf}\u{09b2}\u{09cd}\u{09b2}\u{09be}", region: "Comilla", postal: "3500", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // কুমিল্লা
        City { name: "\u{0997}\u{09be}\u{099c}\u{09c0}\u{09aa}\u{09c1}\u{09b0}", region: "Dhaka", postal: "1700", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // গাজীপুর
        City { name: "\u{09a8}\u{09be}\u{09b0}\u{09be}\u{09af}\u{09bc}\u{09a3}\u{0997}\u{099e}\u{09cd}\u{099c}", region: "Dhaka", postal: "1400", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // নারায়ণগঞ্জ
        City { name: "\u{09ae}\u{09af}\u{09bc}\u{09ae}\u{09a8}\u{09b8}\u{09bf}\u{0982}\u{09b9}", region: "Mymensingh", postal: "2200", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // ময়মনসিংহ
        City { name: "\u{09ac}\u{09b0}\u{09bf}\u{09b6}\u{09be}\u{09b2}", region: "Barisal", postal: "8200", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // বরিশাল
        City { name: "\u{09ac}\u{0997}\u{09c1}\u{09dc}\u{09be}", region: "Rajshahi", postal: "5800", lat: 24.0, lon: 90.0, tz: "Asia/Dhaka" }, // বগুড়া
    ]),
    native_streets: Some(&[
        "\u{0997}\u{09c1}\u{09b2}\u{09b6}\u{09be}\u{09a8} \u{098f}\u{09ad}\u{09bf}\u{09a8}\u{09bf}\u{0989}", // গুলশান এভিনিউ
        "\u{09ac}\u{09a8}\u{09be}\u{09a8}\u{09c0} \u{09b0}\u{09cb}\u{09a1}", // বনানী রোড
        "\u{09a7}\u{09be}\u{09a8}\u{09ae}\u{09a8}\u{09cd}\u{09a6}\u{09bf} \u{09b0}\u{09cb}\u{09a1}", // ধানমন্ডি রোড
        "\u{09ae}\u{09bf}\u{09b0}\u{09aa}\u{09c1}\u{09b0} \u{09b0}\u{09cb}\u{09a1}", // মিরপুর রোড
        "\u{09b8}\u{09be}\u{09a4}\u{09ae}\u{09b8}\u{099c}\u{09bf}\u{09a6} \u{09b0}\u{09cb}\u{09a1}", // সাতমসজিদ রোড
    ]),
};
