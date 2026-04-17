# Field Reference

200+ fields across 17 groups. For syntax, ranges, and ordering — see [fields](fields.md).

## Contents

- [Universal modifiers](#universal-modifiers)
- [daily](#daily) (5)
- [core](#core) (15)
- [text](#text) (10)
- [time](#time) (7)
- [person](#person) (19)
- [contact](#contact) (12)
- [location](#location) (11)
- [finance](#finance) (21)
- [auth](#auth) (49)
- [gov-id](#gov-id) (48)
- [internet](#internet) (47)
- [blockchain](#blockchain) (6)
- [organization](#organization) (9)
- [healthcare](#healthcare) (8)
- [dev](#dev) (5)
- [ops](#ops) (8)
- [device](#device) (5)

## Universal modifiers

All fields support these modifiers:

| Modifier | Description | Example |
|----------|-------------|---------|
| `:upper` | Uppercase output | `ISABELA DESAI` |
| `:lower` | Lowercase output | `isabela desai` |
| `:capitalize` | Capitalize first character | `Isabela desai` |

Combine with field-specific modifiers: `mac:plain:upper`, `amount:usd:lower`.

## daily

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `emoji` |  | 🦗              Single Unicode emoji character | 🍻 |
| `bz` |  | Our enter...    Corporate buzzword or jargon phrase (synergy, leverage) | How we democratize context-aware benchmarks in production |
| `dice` |  | 2               Six-sided die roll result, 1 through 6 | 3 |
| `excuse` |  | The third...    Humorous developer excuse for being late or missing a deadline | It's not this ticket, it's a known issue |
| `mball` |  | Ask Reddi...    Magic 8-Ball fortune response (Yes definitely, Ask again later) | Your future self would say not a chance |
## core

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `integer` | range, asc/desc | 2600            Random whole number in configurable range | 71 |
| `float` | range, asc/desc | 2669.34         Decimal number with 2-digit precision | 2864.14 |
| `boolean` |  | false           true or false with equal probability | true |
| `digit` |  | 3               Single decimal digit [0-9] | 3 |
| `bit` | sign | 0               Binary value, 0 or 1 | 1 |
| `bit:sign` |  | | -1 |
| `trit` |  | 0               Ternary digit (-1, 0, or 1) | 0 |
| `enum` |  | Random pick from values (enum:a,b,c) with weights (enum:a=3,b=1) |  |
| `serial` |  | 0               Zero-based record counter (0, 1, 2, ...) | 0 |
| `color` | hex, rgb, rgba | sea green       Color value: named (maroon), hex (#ff8800), or RGB components (120, 40, 200) | maroon |
| `color:hex` |  | | #09fb2a |
| `color:rgb` |  | | 131, 128, 233 |
| `color:rgba` |  | | 99, 87, 163, 0.68 |
| `uuid` | plain | cd7aa43b-...    Version 4 UUID [xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx] | 8b42b31a-d519-40c1-81a2-a75a1097465b |
| `uuid:plain` |  | | 8d54f5fb531640549488deb7b0d0c3c9 |
## text

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `letter` |  | o               Single ASCII letter [a-zA-Z] | z |
| `trigram` |  | cfp             Pronounceable three-letter syllable (baf, kel, zur) | kkq |
| `digits` | N, range, asc/desc | 137094360       Numeric digit string (digits:4 = 0469, digits:6:100..500 = 000342) | 944395740573540 |
| `letters` | N | eezgyqmkq...    Alphabetic string (letters:8 = kZmPqRtY) | brobtnrbpgb |
| `alnum` | N | NfF3v49UT...    Alphanumeric string (alnum:6 = xK7m2B) | JNgBBgJ3 |
| `base64` | N | ICpgpACps...    Base64-encoded random bytes (base64:16 = aGVsbG8gd29y) | Bmh7FT84eU80ahDYmEg |
| `hex` | byte, N | ca833a4f9       Hexadecimal string (hex:4 = 0f3a, hex:8 = 0f3a7b2e) | db50c2f4c0b |
| `hex:byte` |  | | 47 |
| `word` |  | nostrud         English-like word from vocabulary pool | laboris |
| `message` |  | Ullamco m...    Multi-word natural language sentence | Commodo consectetur do |
## time

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `timestamp` | unix, ms, log, range, asc/desc | 1976-07-2...    ISO 8601 datetime with timezone (2024-03-15T09:30:00Z) | 1977-09-28T21:51:35Z |
| `timestamp:unix` |  | | 1661577082 |
| `timestamp:ms` |  | | 437984489213 |
| `timestamp:log` |  | | 23/Nov/2001:18:17:47 +0000 |
| `date` | us, eu, range, asc/desc | 1915-10-04      Calendar date in locale-specific format (2024-03-15) | 1967-02-26 |
| `date:us` |  | | 03/18/1975 |
| `date:eu` |  | | 11.09.2005 |
## person

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `name` |  | Athena Ho...    Full person name with first and last components | Isabela Desai |
| `first-name` |  | Hana            Given name from locale-aware dictionary | Laura |
| `last-name` |  | Desai           Family name from locale-aware dictionary | Bailey |
| `middle-name` |  | Carlos          Middle name or patronymic from locale-aware dictionary | Mira |
| `birthdate` | us, eu, range | 1978-08-22      Date of birth with weighted age distribution | 1963-08-05 |
| `birthdate:us` |  | | 08/08/1980 |
| `birthdate:eu` |  | | 11.08.1977 |
| `age` | range, asc/desc | 69              Age (18-100), weighted demographic distribution | 18 |
| `gender` |  | Female          Gender label (male, female, non-binary) | Female |
| `username` | xuniq | g_morgan_02     Clean platform handle [a-z0-9_] | michelle_powers02 |
| `username:xuniq` |  | | kingsophia02_oysa2 |
| `login-name` | xuniq | markgupta...    System login identifier, typically firstname.lastname | shaniqua_ball3924 |
| `login-name:xuniq` |  | | elias_colon_vip.nq2ii |
| `social-handle` | xuniq | @franklin...    Social media display name with @ prefix (@cooluser99) | @fionaromero5284 |
| `social-handle:xuniq` |  | | @la5284_n9clx |
| `nickname` | xuniq | blade15         Creative alias unrelated to real name (darkwolf42, swiftcoder) | wanderer_4716 |
| `nickname:xuniq` |  | | sigil540_yt39l |
| `biometric-id` |  | BIO-IRIS-...    Opaque biometric template reference [hex, 32 bytes] | BIO-FP-KQKGANVWLIDRVYFM |
| `student-id` |  | SID-95258...    University or school student identifier [alphanumeric] | SID-493724770 |
## contact

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `email` | xuniq | melodyca7...    Email with locale-aware name and domain | lennoxshah7944@hey.com |
| `email:xuniq` |  | | dariusmartinez43.puwfa@icloud.com |
| `phone` | e164, intl, plain | +1-747-86...    Phone number with country code and locale formatting | (942) 322-8377 |
| `phone:e164` |  | | +15448048956 |
| `phone:intl` |  | | +1 935 449 779 5 |
| `phone:plain` |  | | 3357759469 |
| `address` |  | 575 Monro...    Full mailing address with street, city, state, and postal code | 2506 Monroe Ave, Baltimore, MD 21201 |
| `street-address` |  | 3039 Yong...    Street line with number and street name (742 Evergreen Terrace) | 4520 West Rd |
| `city` |  | Nashville       Real city name from weighted population data | Indianapolis |
| `state` |  | TN              US state or equivalent administrative region name | MO |
| `postal-code` |  | 33600           ZIP or postal code matching locale format | 92100 |
| `country` |  | United St...    Full country name from ISO 3166 list | United States |
## location

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `latitude` |  | 41.9800         Locale-aware geographic latitude, 4 decimal places | 47.9700 |
| `longitude` |  | -93.3000        Locale-aware geographic longitude, 4 decimal places | -89.9600 |
| `country-code` | alpha3, numeric | US              ISO 3166-1 alpha-2 country code (US, DE, JP) | US |
| `country-code:alpha3` |  | | USA |
| `country-code:numeric` |  | | 840 |
| `phone-code` |  | +1              International dialing prefix (+1, +44, +81) | +1 |
| `language-code` |  | en              ISO 639-1 two-letter language code (en, fr, ja) | en |
| `locale-code` | short, underscore | en-US           IETF BCP 47 locale tag (en-US, pt-BR, zh-Hans) | en-US |
| `locale-code:short` |  | | en |
| `locale-code:underscore` |  | | en_US |
| `timezone` |  | America/L...    IANA timezone identifier (America/New_York, Europe/Berlin) | Asia/Dubai |
## finance

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `credit-card` | space, dash, plain | 3730-2605...    Valid-checksum card number with BIN prefix [16 digits] | 4088524207310166 |
| `credit-card:space` |  | | 3465 803194 11819 |
| `credit-card:dash` |  | | 4449-3440-9913-9167 |
| `credit-card:plain` |  | | 4690893169519027 |
| `cvv` |  | 236             Card verification value [3-4 digits] | 5885 |
| `iban` | plain | DE09 4299...    International Bank Account Number with valid check digits | FR72 4076 7299 5650 0018 8903 684 |
| `iban:plain` |  | | GB90200232529803423701 |
| `swift-bic` |  | TPZXUSUN        SWIFT/BIC bank identifier code [8 or 11 chars] | SXOQUSOI |
| `routing-number` |  | 904127126       US ABA routing transit number [9 digits, valid checksum] | 372968343 |
| `bank-account` |  | 83255105748     Bank account number with locale-appropriate length | 4258908965506695 |
| `tax-id` |  | 17-9496603      US Employer Identification Number [NN-NNNNNNN] | 77-3450182 |
| `amount` | dot, comma, plain, usd, eur, gbp, range, asc/desc | $62,330.96      Monetary value with configurable currency and decimal format | $161.72 |
| `amount:dot` |  | | 21.41 |
| `amount:comma` |  | | 2.043,82 |
| `amount:plain` |  | | 10982.37 |
| `amount:usd` |  | | $572.70 |
| `amount:eur` |  | | €145,85 |
| `amount:gbp` |  | | £7,855.48 |
| `currency-code` | crypto | USD             ISO 4217 three-letter currency code (USD, EUR, GBP) | USD |
| `currency-code:crypto` |  | | DOT |
| `currency-symbol` |  | $               Unicode currency glyph ($, EUR, GBP, JPY) | $ |
## auth

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `password` | pin, memorable, mixed, strong, N | iloveyou        Password with configurable length (password:12 for exact) | hot106 |
| `password:pin` |  | | 6905 |
| `password:memorable` |  | | winter-silver-cosmic |
| `password:mixed` |  | | Qu*UPpnMO#Tiu2 |
| `password:strong` |  | | 08xC2_*X&T.+k00iCVrkf |
| `jwt` |  | eyJhbGciO...    JSON Web Token with valid header.payload.signature structure | eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI2MTY3MDkiLCJlbWFpbCI6I |
| `bearer-token` |  | Bearer gQ...    OAuth2 bearer token, opaque [base64url, 32+ bytes] | Bearer tNTcye1HX6vjlM8r7UyQOAC9CQ_I092vvXrYOEhzvd5/AyOtPKh82wgkm/-r |
| `api-key` |  | key-BiIGE...    Generic API key [alphanumeric, 32-48 chars] | api_1c857323d5a7924c980ad6a01e73d984376c3245 |
| `totp-secret` |  | 7AF5DMVYV...    TOTP shared secret [base32, 16+ chars] | JR4M4KTKFWN32J27HO3WYXTPC45NBYQD |
| `oauth-client-secret` |  | cos_Du8lL...    OAuth2 client secret [alphanumeric, 40 chars] | cos_op2ehovq7WlBobm4lAYIhlJiroaHc2MQiLizuk23lhdqRjw6 |
| `aws-access-key` |  | AKIA6ISW9...    AWS access key ID [AKIA...] | AKIA6ZI1S12431KML0CO |
| `aws-secret-key` |  | HvWxXnWuc...    AWS secret access key [base64, 40 chars] | m/YLCfdQrxnkKAEHERPSD7D24LYG9LCeJdw8r/Hf |
| `stripe-key` |  | sk_live_Y...    Stripe API key with environment prefix [sk_live_..., sk_test_...] | sk_test_SPShuGbfXV3w7zLrpPTiym9WD6H5yz |
| `github-pat` |  | ghr_641sH...    GitHub personal access token [ghp_...] | ghu_7CgGs3DcHoJGnDgbR0rMrSazTGdRXcH7Fin4 |
| `gitlab-token` |  | glpat-y1E...    GitLab personal or project token [glpat-...] | glpat-szEAJIILD9jz8IejPCS7 |
| `openai-key` |  | sk-proj-Q...    OpenAI API key [sk-...] | sk-hQbsXSXLo0VprqO8svu8c5n210bSTzB8bqvHwgvrVhaHGZsU |
| `sendgrid-key` |  | SG.kkIHJV...    SendGrid API key [SG....] | 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 |
| `twilio-sid` |  | ACcd2edcd...    Twilio Account SID [AC...] | ACc747598b62c482d85461123c70eca516 |
| `twilio-token` |  | 0681c57a1...    Twilio auth token [hex, 32 chars] | 648b7551f7f567570420c6e4434ba061 |
| `slack-bot-token` |  | xoxb-4432...    Slack bot OAuth token [xoxb-...] | xoxb-285802648253-548930984423-m54Tf1RvmOiQEfuMbBHYargE |
| `slack-user-token` |  | xoxp-6081...    Slack user OAuth token [xoxp-...] | xoxp-449032790583-463193677831-816269870090-f53cdf590eceaf7c36e34ad1f8 |
| `datadog-key` |  | e28e21cab...    Datadog API or application key [hex, 32 chars] | 00d8d67a0806aef5585f8c05d01ff6f1 |
| `sentry-dsn` |  | https://d...    Sentry Data Source Name URL with project ID | https://f06b497a8d7a05a7b6efa49b1238b7c5@o156425.ingest.sentry.io/7001 |
| `vault-token` |  | hvs.uBjpl...    HashiCorp Vault access token [hvs....] | hvs.Tr8QhBJ8SiZpHZ7tMeFSXjQh |
| `npm-token` |  | npm_PhvGP...    npm registry auth token [npm_...] | npm_TXFxuIeSpqRyYqjpIddvW7qJ5jCN0wngynug |
| `vercel-token` |  | vc_prod_8...    Vercel deployment token [alphanumeric, 24 chars] | vc_test_upm2oQZsLp5uXxwsdrQAy9aLEReKQIhu |
| `supabase-key` |  | eyJTylSJO...    Supabase anon or service-role JWT key | eyJ56OKKEs6BQuPwmUmRAt6Kk9DzX447DlmRTFZKkvRVcSSDFvaVbokEwAY6KiA14dlr4w |
| `telegram-token` |  | 339731864...    Telegram Bot API token [NNNNNNNNNN:AAxx...] | 075703721:aLcDhCuNJ4t8P9KGQUAmQ65EsSPb7uwG4Ho |
| `discord-webhook` |  | https://d...    Discord webhook URL with token path | https://discord.com/api/webhooks/508314262355040608/7LnPj8qMifNt16i8QC |
| `gcp-key` |  | AIza7qpfD...    Google Cloud Platform API key [AIza...] | AIza1kWL2smmiMnBnH8c4zsywhys5GF1Lbw2DHR |
| `azure-key` |  | ExnaGeqqS...    Microsoft Azure subscription or service key [base64] | dD3ysZHgcwwjvZR4XoZpRo0wprZd0mDkLV6SsUn/cDDt |
| `cloudflare-token` |  | GWuwn0FEa...    Cloudflare API token [alphanumeric, 40 chars] | CKnsriHg9aMMCojC0I8BRQyCqnbQx64qZlzthbJL |
| `pagerduty-key` |  | u+kXwmprm...    PagerDuty API key [alphanumeric, 20 chars] | u+CspjTaLizWg8vF1hTE4y |
| `newrelic-key` |  | NRAK-3Lc9...    New Relic API key [NRAK-...] | NRAK-4qL4rcxrDssAFY8fAcji4WOv8QaCJdwE |
| `splunk-token` |  | 06bbc1d5-...    Splunk HEC auth token [UUID format] | 3bf3943c-4945-4d9f-aeae-06f585a3d82f |
| `heroku-key` |  | 2ed65908-...    Heroku API key [UUID format] | 9ae72ad4-8fe3-4df7-ba86-2c4c5d9b91d6 |
| `firebase-key` |  | AIzawNEBI...    Firebase Web API key [AIza...] | AIzais3SU4EXWUbWKBk5A9RE9NUXuyg0NDrNonW |
| `ssh-private-key` |  | -----BEGI...    PEM-encoded SSH private key block (RSA or Ed25519) | -----BEGIN OPENSSH PRIVATE KEY----- d6HqiNssW743otEtnX1asmoYP/P91FYYTZ |
| `ssh-public-key` |  | ssh-ed255...    OpenSSH public key line (ssh-rsa ..., ssh-ed25519 ...) | ssh-ed25519 AAAAC3NzaC1lZDI1NTE5RbrPkz3vfALAGnbA+0LruRTH4HUJPzhDDrSya0 |
| `connection-string` |  | postgresq...    Database connection URI with credentials (postgres://user:pass@...) | mongodb://carmen:Hz5YU1tmxVhf4i@db-90.internal.kpmg.com:27017/main |
| `anthropic-key` |  | sk-ant-ap...    Anthropic API key [sk-ant-...] | sk-ant-api03-upUl66tjOIJYXHXmB5DF2pk2bneDeu5yWl9wJhoBQmrrms3t7mvJ3D1Qx |
| `session-id` |  | sess_2bdd...    Opaque session identifier [base64url, 32+ bytes] | sess_be6a090d569e3a092e40fb26321ddff1 |
| `passkey-id` |  | GQdgHlCyH...    WebAuthn passkey credential ID [base64url] | tj_XLdsYyQFowOouE0EY_ojf976--9EHFQ058JBQbN- |
| `facebook-token` |  | EAAGm0PX4...    Facebook/Meta Graph API access token [EAA...] | EAAGm0PX4ZCpsGgudjEQLcqPqqax63hxLmXPnpkw2NuXbjzapiEdbZI0RG11S416C4hxgp |
| `google-token` |  | ya29.a0Q3...    Google OAuth2 access token [ya29....] | ya29.a0QiRFNwvS1R87ti118adiKYUltsW56tL7EzoPgQkUxF2unVQzxafPRycFvNf8mnX |
| `apple-token` |  | eyJraWQiO...    Apple Sign-In JWT identity token | eyJraWQiOiJXNkdqIiwiYWxnIjoiRVMyNTYifQ.eyJpc3MiOiJodHRwczovL2FwcGxlaWQ |
| `refresh-token` |  | rt_VlHm9D...    OAuth2 refresh token [opaque, 64+ chars] | rt_T3c9oYExiLOGBen5kTiQq02kzRv0lFwjwOVJqp1SaF8uxaNM |
| `csrf-token` |  | 9f527fdc8...    Cross-site request forgery protection token [hex, 32 bytes] | 23939a5b267bf06732ba6ecf3a223a37 |
| `basic-auth` |  | Basic bml...    HTTP Basic auth header value [base64(user:pass)] | Basic c29maWE6TEVDWTlHaVllaFUw |
## gov-id

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `ssn` | plain | 662-72-6771     US Social Security Number [NNN-NN-NNNN] | 819-52-1026 |
| `ssn:plain` |  | | 700513539 |
| `passport` | international, internal | 951205946       Passport number with country-appropriate format | 759472470 |
| `passport:international` |  | | YH9045779 |
| `passport:internal` |  | | 9741 206322 |
| `drivers-license` |  | NY-M4495912     US driver's license number, state-format-aware | TX-L3764274 |
| `national-id` |  | 493-80-1436     Generic national identity number for configurable country | 469-91-3582 |
| `cpf` | plain | 649.338.3...    Brazil Cadastro de Pessoa Fisica [NNN.NNN.NNN-NN] | 386.448.970-99 |
| `cpf:plain` |  | | 66803318212 |
| `sin` |  | 481-183-723     Canada Social Insurance Number [NNN-NNN-NNN] | 952-533-110 |
| `tfn` |  | 133 045 911     Australia Tax File Number [NNN NNN NNN] | 180 953 329 |
| `nino` |  | KK 73 65 ...    UK National Insurance Number [AA NNNNNN A] | TE 78 66 43 A |
| `nhs-number` |  | 626 847 9824    UK National Health Service number [NNN NNN NNNN] | 966 551 4373 |
| `nir` |  | 2 33 02 9...    France national ID registration number [13 digits + key] | 2 89 78 09 268 118 |
| `codice-fiscale` |  | DCXIZC02P...    Italy fiscal code [16 alphanumeric chars] | AKRRGY01C11N379X |
| `dni` |  | 58890928A       Spain Documento Nacional de Identidad [8 digits + letter] | 11173605K |
| `nie` |  | Z4657439J       Spain foreigner identification number [X/Y/Z + 7 digits + letter] | Y4545400Q |
| `bsn` |  | 969982889       Netherlands Burgerservicenummer [9 digits] | 688325807 |
| `personnummer` |  | 830601-2384     Sweden personal identity number [YYYYMMDD-NNNN] | 851006-0881 |
| `steuer-id` |  | 22161518448     Germany tax identification number [11 digits] | 66614593352 |
| `cuil` |  | 23-299906...    Argentina labor identification code [NN-NNNNNNNN-N] | 24-94696166-7 |
| `jmbg` |  | 221195481...    Former Yugoslavia unique master citizen number [13 digits] | 1802967823084 |
| `tc-kimlik` |  | 84832021539     Turkey national identity number [11 digits] | 21810328601 |
| `pesel` |  | 71081632916     Poland national identification number [11 digits] | 57102232894 |
| `curp` |  | GSRU88070...    Mexico population registry key [18 alphanumeric chars] | FPMV971125MBNONX4A |
| `rut` |  | 5.785.233-8     Chile unique tax role number [NN.NNN.NNN-D] | 18.888.474-7 |
| `inn` |  | 351070555540    Russia taxpayer identification number [10 or 12 digits] | 9217848587 |
| `ipn` |  | 6689540489      Ukraine individual tax number [10 digits] | 1173233260 |
| `abn` |  | 52 945 55...    Australia Business Number [11 digits] | 51 799 980 814 |
| `cnpj` | plain | 70.001.34...    Brazil corporate taxpayer registry [NN.NNN.NNN/NNNN-NN] | 17.373.050/0775-83 |
| `cnpj:plain` |  | | 23661324350702 |
| `oib` |  | 52643735080     Croatia personal identification number [11 digits] | 04719769046 |
| `amka` |  | 17066428802     Greece social security number [11 digits] | 14016301423 |
| `rodne-cislo` |  | 801024/4824     Czech/Slovakia birth number [NNNNNN/NNNN] | 510407/2118 |
| `szemelyi-szam` |  | 87-858593-5     Hungary personal identification number [N-NNNNNN-A] | 84-208002-1 |
| `hetu` |  | 210392-313A     Finland personal identity code [DDMMYY-NNNC] | 070896A221P |
| `cpr` |  | 050351-4517     Denmark civil registration number [DDMMYY-NNNN] | 161151-0029 |
| `fodselsnummer` |  | 19066293493     Norway national identity number [11 digits] | 20106213987 |
| `pps` |  | 7151032WX       Ireland Personal Public Service number [NNNNNNNAA] | 1722814I |
| `emso` |  | 161198522...    Slovenia unique master citizen number [13 digits] | 1507972732996 |
| `egn` |  | 6307033025      Bulgaria Unified Civil Number [10 digits] | 7903168702 |
| `idnp` |  | 381586909...    Moldova personal identification number [13 digits] | 5017622811859 |
| `health-card` |  | 1576-551-258    Provincial health insurance card number (Canada) | 0991-110-872 |
| `cedula` |  | 02248726        Colombia/Ecuador/Venezuela national ID card number | 64913721 |
| `aadhaar` |  | 8334 5708...    India unique identity number [NNNN NNNN NNNN] | 2061 4470 1093 |
| `pan` |  | LDTKZ4974L      India Permanent Account Number [AAAAA9999A] | ZHYYU4841V |
| `cccd` |  | 340989629019    Vietnam citizen identity card number [12 digits] | 078339708205 |
| `shenfenzheng` |  | 618446199...    China resident identity card number [18 digits] | 214572196601155639 |
## internet

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `ip` |  | 45.131.0.69     IPv4 address in dotted-decimal notation (192.168.1.42) | 91.199.2.251 |
| `ipv6` |  | bb6f:89ed...    Full IPv6 address [8 colon-separated hextets] | 219f:b10b:b4ff:b454:638d:5bc1:5010:855d |
| `mac` | plain, dot | 75-be-be-...    MAC address [AA:BB:CC:DD:EE:FF] | 40-c7-ae-43-35-8b |
| `mac:plain` |  | | 152edf63058a |
| `mac:dot` |  | | a40b.5bfd.58d9 |
| `url` | http, https, ftp, ws, wss, ssh | https://d...    Well-formed URL with scheme, host, and path | https://portal.corp.net/v3/webhooks |
| `url:http` |  | | http://cdn.assets.io/v1/products |
| `url:https` |  | | https://api.example.com/v3/metrics |
| `url:ftp` |  | | ftp://gateway.cloud.dev/archive/iayuyB6F.dat |
| `url:ws` |  | | ws://data.platform.dev/v2/users |
| `url:wss` |  | | wss://app.internal.io/v2/webhooks |
| `url:ssh` |  | | ssh://ci@api.example.com:1644 |
| `auth-url` |  | https://n...    URL with embedded credentials (scheme://user:pass@host/path) | https://ivy:9Q0dhlgbgC6E@morganstanley.com/admin |
| `internal-url` |  | https://g...    Private-network URL targeting RFC 1918 or localhost | https://grafana.corp.internal/d/api-latency-55576 |
| `dns-record` |  | db.prod.i...    DNS resource record line (A, AAAA, CNAME, MX) | worker.dev.internal A 10.144.93.5 |
| `browser-cookie` |  | session=4...    HTTP Set-Cookie header value with name, value, and attributes | session=999f584333e44fc36a506adce6be503a; _ga=GA1.2.810090758.17099772 |
| `user-agent` |  | Mozilla/5...    Browser or bot User-Agent header string | Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chr |
| `mime-type` |  | audio/mpeg      IANA media type (application/json, image/png) | image/png |
| `http-method` |  | POST            HTTP request method (GET, POST, PUT, DELETE) | GET |
| `http-status` |  | 401             HTTP response status code with reason (200 OK, 404 Not Found) | 403 |
| `port` | system, registered, dynamic, unprivileged, service | 8531            TCP/UDP port number with weighted service distribution | 9000 |
| `port:system` |  | | 53 |
| `port:registered` |  | | 18955 |
| `port:dynamic` |  | | 52072 |
| `port:unprivileged` |  | | 15053 |
| `port:service` |  | | 587 |
| `latency` | fast, slow, seconds, asc/desc | 263             Response latency in milliseconds, log-normal distribution (1-30000) | 141 |
| `latency:fast` |  | | 75 |
| `latency:slow` |  | | 3510 |
| `latency:seconds` |  | | 0.434 |
| `image-url` | 1x1, 4x3, 3x2, 16x9, 21x9, 9x16, 3x4, 2x3 | https://p...    Placeholder image URL with configurable aspect ratio | https://picsum.photos/seed/233/600/800 |
| `image-url:1x1` |  | | https://picsum.photos/seed/302/512/512 |
| `image-url:4x3` |  | | https://picsum.photos/seed/236/800/600 |
| `image-url:3x2` |  | | https://picsum.photos/seed/661/900/600 |
| `image-url:16x9` |  | | https://picsum.photos/seed/593/1280/720 |
| `image-url:21x9` |  | | https://picsum.photos/seed/88/1260/540 |
| `image-url:9x16` |  | | https://picsum.photos/seed/605/720/1280 |
| `image-url:3x4` |  | | https://picsum.photos/seed/838/600/800 |
| `image-url:2x3` |  | | https://picsum.photos/seed/184/600/900 |
| `twitter-url` |  | https://x...    X/Twitter profile or post URL (https://x.com/...) | https://x.com/jasperhortonh |
| `linkedin-url` |  | https://l...    LinkedIn profile URL (https://linkedin.com/in/...) | https://linkedin.com/in/tara-mann-c92600 |
| `facebook-url` |  | https://f...    Facebook profile URL (https://facebook.com/...) | https://facebook.com/agentsilviam4 |
| `instagram-url` |  | https://i...    Instagram profile URL (https://instagram.com/...) | https://instagram.com/juliaxx7841 |
| `github-url` |  | https://g...    GitHub user or repo URL (https://github.com/...) | https://github.com/dominic576 |
| `telegram-url` |  | https://t...    Telegram profile or invite URL (https://t.me/...) | https://t.me/luis_lewis_box |
| `youtube-url` |  | https://y...    YouTube video or channel URL (https://youtube.com/...) | https://youtube.com/@princefinn24 |
| `webhook-url` |  | https://h...    Generic webhook endpoint URL with token path | https://hooks.slack.com/services/W4RLPW0N5/KLUO4CAUA/q00YNzY77jN1cFkzT |
## blockchain

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `btc-address` |  | bc1q8xdkc...    Bitcoin address, P2PKH or Bech32 format (1..., bc1...) | bc1qux3et9x0twf96pdjf4mq7pd0w7cd9kxjclzhx9 |
| `eth-address` |  | 0x242cd85...    Ethereum address with EIP-55 checksum [0x + 40 hex chars] | 0xb8e3f19a72af90b1e190fdd4badbacda7a0ec42b |
| `sol-address` |  | oo7cxf1HB...    Solana base58-encoded public key [32-44 chars] | FxKs2QSK2utmHFgoYiqoq8h1J3TC4tgUcKU6s |
| `tx-hash` | btc | 0x7a1a16b...    Blockchain transaction hash [hex, 64 chars] | 0x18617ffb21ba9a5d62b6ddfef4f150b029db862aa772b0d2b230be61ec753775 |
| `tx-hash:btc` |  | | feda3408720897f17179f28b4e05db06ec186e9c775c45707ee40f0418799f6f |
| `pgp-fingerprint` |  | F421 AAEA...    PGP key fingerprint [40 hex chars, space-grouped] | CCD4 E2D0 87D5 9AC4 ABD3 B424 18B3 1756 7798 8D2C |
## organization

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `company-name` |  | Oliver Wyman    Business name with optional suffix (Inc, LLC, GmbH) | Raymond James |
| `ein` |  | 49-1068144      US Employer Identification Number [NN-NNNNNNN] | 08-8787543 |
| `vat-number` |  | GB606 688...    EU Value Added Tax identification number with country prefix | GB128 9189 12 |
| `duns` |  | 60-945-5587     Dun & Bradstreet business identifier [9 digits] | 58-063-3275 |
| `lei` |  | PNQI4Z0NE...    Legal Entity Identifier [20 alphanumeric chars] | AS9JIOPSB7GUYYFJSRWS |
| `job-title` |  | VP Engine...    Professional role title (Senior Engineer, Product Manager) | CFO |
| `ldap-dn` |  | CN=Anvi W...    LDAP distinguished name (cn=John,ou=Users,dc=example,dc=com) | CN=Bennett Parks,OU=Legal,DC=internal,DC=com |
| `employee-id` |  | EMP-3634533     Internal employee identifier [alphanumeric, prefixed] | ID-30634533 |
| `court-case` |  | Case No. ...    US federal court case number (1:24-cv-01234) | Case No. 2021-MC-82347 (CDCA) |
## healthcare

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `mrn` |  | MRN-9225528     Medical Record Number, facility-scoped [alphanumeric] | MR63345015 |
| `npi` |  | 3982324285      US National Provider Identifier [10 digits, Luhn-valid] | 8481379611 |
| `insurance-id` |  | CIGNA-578...    Health insurance member or policy ID [alphanumeric] | BCBS-800775144 |
| `medicare-id` |  | 0J9P7362N9X     US Medicare Beneficiary Identifier [11 alphanumeric chars] | B77CFCCICVP |
| `icd-10` |  | R57.0           ICD-10 diagnosis code (A00.0, J06.9, M54.5) | E49.6 |
| `cpt-code` |  | 46670           CPT medical procedure code [5 digits] | 18249 |
| `ndc` |  | 19290-913-69    US National Drug Code [NNNNN-NNNN-NN] | 31132-608-13 |
| `rx-number` |  | RX-6467867      Pharmacy prescription number [7-12 digits] | RX-8334605 |
## dev

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `project-code` |  | PRJ-ABBU-...    Short project identifier with prefix (PRJ-0042, ACME-117) | PRJ-UWSH-7237 |
| `jira-id` |  | ENG-22098       Jira issue key [PROJECT-NNNN] | ENG-62545 |
| `github-issue` |  | #4584           GitHub issue reference (owner/repo#1234) | #3194 |
| `commit-hash` |  | 5fe8e0872...    Git commit SHA-1 hash [hex, 40 chars] | 6234e07cea62a005733465e95a8d264ffb67cf0e |
| `semver` |  | 3.5.41          Semantic version string (1.4.2, 0.12.0-beta.3) | 5.14.63 |
## ops

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `docker-image` |  | gcr.io/da...    Docker image reference with registry, name, and tag | docker.io/myorg/api:v3.20.57 |
| `slack-channel` |  | #general        Slack channel name with # prefix (#engineering, #alerts) | #data-eng |
| `sentry-issue` |  | FRONTEND-...    Sentry issue identifier [PROJECT-HASH] | API-BC13 |
| `pagerduty-incident` |  | P8676537        PagerDuty incident ID [alphanumeric, 7+ chars] | P5419095 |
| `file-path` |  | /tmp/pxtv...    Unix-style absolute file path (/var/log/app/server.log) | /tmp/vfl2yw/dump.tar.gz |
| `s3-path` |  | s3://asse...    AWS S3 object URI (s3://bucket-name/key/path) | s3://analytics/raw/2024-03-27/export.json |
| `env-var` | multi | SMTP_PASS...    Environment variable assignment (DATABASE_URL=postgres://...) | AWS_SECRET_ACCESS_KEY=qv6LgJaJuvWpOZlkylqbdg2JnbXQx/HF2I3cie9p |
| `env-var:multi` |  | | AWS_SECRET_ACCESS_KEY=j+Nzz+bC4FZ1YED2zq9Vk/hJGOPbx6+knnFnskPe SMTP_PA |
## device

| Field | Modifiers | Description | Example |
|-------|-----------|-------------|---------|
| `vin` |  | WNL14DR87...    Vehicle Identification Number [17 alphanumeric chars] | NNF2LDL6D2UB5LG6X |
| `license-plate` |  | SAM-6251        Vehicle registration plate with locale-appropriate format | AGS-7427 |
| `imei` |  | 221880794...    Mobile device IMEI [15 digits, Luhn-valid] | 506532529167844 |
| `imsi` |  | 311804444...    Mobile subscriber identity [15 digits, MCC+MNC prefix] | 208982969779763 |
| `device-id` |  | 29C73EF8-...    Opaque hardware or app-instance identifier [UUID or hex] | 4B0418CE-13F8-4F32-81C1-FC0F64ECB613 |

[Quick start](quick-start.md) · [Fields](fields.md) · [CLI](cli.md) · [Configs](configs.md) · [Context](context.md) · [Guides](../guides/)
