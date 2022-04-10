extern crate test;
use test::Bencher;

use crate::define;

    #[bench]
    fn bench_dsl_1_transition(b: &mut Bencher) {
        b.iter(|| {
            define!("11" - "12" -> "13",);
        });
    }

    #[bench]
    fn bench_dsl_10_transitions(b: &mut Bencher) {
        b.iter(|| {
            define!(
                "11" - "12" -> "13",
                "21" - "22" -> "23",
                "31" - "32" -> "33",
                "41" - "42" -> "43",
                "51" - "52" -> "53",
                "61" - "62" -> "63",
                "71" - "72" -> "73",
                "81" - "82" -> "83",
                "91" - "92" -> "93",
                "101" - "102" -> "103",
            );
        });
    }

    #[bench]
    fn bench_dsl_100_transitions(b: &mut Bencher) {
        b.iter(|| {
            define!(
                "11" - "12" -> "13",
                "21" - "22" -> "23",
                "31" - "32" -> "33",
                "41" - "42" -> "43",
                "51" - "52" -> "53",
                "61" - "62" -> "63",
                "71" - "72" -> "73",
                "81" - "82" -> "83",
                "91" - "92" -> "93",
                "101" - "102" -> "103",
                "111" - "112" -> "113",
                "121" - "122" -> "123",
                "131" - "132" -> "133",
                "141" - "142" -> "143",
                "151" - "152" -> "153",
                "161" - "162" -> "163",
                "171" - "172" -> "173",
                "181" - "182" -> "183",
                "191" - "192" -> "193",
                "201" - "202" -> "203",
                "211" - "212" -> "213",
                "221" - "222" -> "223",
                "231" - "232" -> "233",
                "241" - "242" -> "243",
                "251" - "252" -> "253",
                "261" - "262" -> "263",
                "271" - "272" -> "273",
                "281" - "282" -> "283",
                "291" - "292" -> "293",
                "301" - "302" -> "303",
                "311" - "312" -> "313",
                "321" - "322" -> "323",
                "331" - "332" -> "333",
                "341" - "342" -> "343",
                "351" - "352" -> "353",
                "361" - "362" -> "363",
                "371" - "372" -> "373",
                "381" - "382" -> "383",
                "391" - "392" -> "393",
                "401" - "402" -> "403",
                "411" - "412" -> "413",
                "421" - "422" -> "423",
                "431" - "432" -> "433",
                "441" - "442" -> "443",
                "451" - "452" -> "453",
                "461" - "462" -> "463",
                "471" - "472" -> "473",
                "481" - "482" -> "483",
                "491" - "492" -> "493",
                "501" - "502" -> "503",
                "511" - "512" -> "513",
                "521" - "522" -> "523",
                "531" - "532" -> "533",
                "541" - "542" -> "543",
                "551" - "552" -> "553",
                "561" - "562" -> "563",
                "571" - "572" -> "573",
                "581" - "582" -> "583",
                "591" - "592" -> "593",
                "601" - "602" -> "603",
                "611" - "612" -> "613",
                "621" - "622" -> "623",
                "631" - "632" -> "633",
                "641" - "642" -> "643",
                "651" - "652" -> "653",
                "661" - "662" -> "663",
                "671" - "672" -> "673",
                "681" - "682" -> "683",
                "691" - "692" -> "693",
                "701" - "702" -> "703",
                "711" - "712" -> "713",
                "721" - "722" -> "723",
                "731" - "732" -> "733",
                "741" - "742" -> "743",
                "751" - "752" -> "753",
                "761" - "762" -> "763",
                "771" - "772" -> "773",
                "781" - "782" -> "783",
                "791" - "792" -> "793",
                "801" - "802" -> "803",
                "811" - "812" -> "813",
                "821" - "822" -> "823",
                "831" - "832" -> "833",
                "841" - "842" -> "843",
                "851" - "852" -> "853",
                "861" - "862" -> "863",
                "871" - "872" -> "873",
                "881" - "882" -> "883",
                "891" - "892" -> "893",
                "901" - "902" -> "903",
                "911" - "912" -> "913",
                "921" - "922" -> "923",
                "931" - "932" -> "933",
                "941" - "942" -> "943",
                "951" - "952" -> "953",
                "961" - "962" -> "963",
                "971" - "972" -> "973",
                "981" - "982" -> "983",
                "991" - "992" -> "993",
                "1001" - "1002" -> "1003"
            );
        });
    }