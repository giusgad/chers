use crate::defs::{Bitboard, NrOf};

#[derive(Clone, Copy, Debug)]
pub struct Magic {
    pub nr: u64,
    pub shift: usize,
    pub offset: usize,
}

impl Magic {
    pub fn new() -> Self {
        Self {
            nr: 0,
            shift: 0,
            offset: 0,
        }
    }
    pub fn get_index(&self, blockers: Bitboard) -> usize {
        (blockers.wrapping_mul(self.nr) as usize >> self.shift) + self.offset
    }
}

#[rustfmt::skip]
pub const BISHOP_MAGICS: [Magic; NrOf::SQUARES] = [
    Magic { nr: 39570332696314500, shift: 58, offset: 0 },
    Magic { nr: 1191225225912664128, shift: 59, offset: 64 },
    Magic { nr: 4575213914161152, shift: 59, offset: 96 },
    Magic { nr: 361485341036642304, shift: 59, offset: 128 },
    Magic { nr: 145280809988001796, shift: 59, offset: 160 },
    Magic { nr: 1225543165828661248, shift: 59, offset: 192 },
    Magic { nr: 12684953568757153800, shift: 59, offset: 224 },
    Magic { nr: 301759869015296032, shift: 58, offset: 256 },
    Magic { nr: 72202800507208704, shift: 59, offset: 320 },
    Magic { nr: 2382417410037973250, shift: 59, offset: 352 },
    Magic { nr: 2449975806950777600, shift: 59, offset: 384 },
    Magic { nr: 288234791386677252, shift: 59, offset: 416 },
    Magic { nr: 9223376779572412416, shift: 59, offset: 448 },
    Magic { nr: 18034215757283392, shift: 59, offset: 480 },
    Magic { nr: 41678972564553760, shift: 59, offset: 512 },
    Magic { nr: 297290601207170049, shift: 59, offset: 544 },
    Magic { nr: 18015575600661632, shift: 59, offset: 576 },
    Magic { nr: 9250500459070948864, shift: 59, offset: 608 },
    Magic { nr: 292736183727820832, shift: 57, offset: 640 },
    Magic { nr: 37155831401222400, shift: 57, offset: 768 },
    Magic { nr: 14989105530673317952, shift: 57, offset: 896 },
    Magic { nr: 2378182119299481872, shift: 57, offset: 1024 },
    Magic { nr: 4652288801033556004, shift: 59, offset: 1152 },
    Magic { nr: 4791882781221454848, shift: 59, offset: 1184 },
    Magic { nr: 577076565789147680, shift: 59, offset: 1216 },
    Magic { nr: 10736880592034727936, shift: 59, offset: 1248 },
    Magic { nr: 4755840789593456672, shift: 57, offset: 1280 },
    Magic { nr: 72339346241617922, shift: 55, offset: 1408 },
    Magic { nr: 7319587452387392, shift: 55, offset: 1920 },
    Magic { nr: 164399666350302208, shift: 57, offset: 2432 },
    Magic { nr: 10390966524147401216, shift: 59, offset: 2560 },
    Magic { nr: 306808549256990858, shift: 59, offset: 2592 },
    Magic { nr: 1128373826888960, shift: 59, offset: 2624 },
    Magic { nr: 2594500065269662218, shift: 59, offset: 2656 },
    Magic { nr: 1126209145801824, shift: 57, offset: 2688 },
    Magic { nr: 4505800809119872, shift: 55, offset: 2816 },
    Magic { nr: 5405449902345949440, shift: 55, offset: 3328 },
    Magic { nr: 2332873405218439280, shift: 57, offset: 3840 },
    Magic { nr: 2453354658705573888, shift: 59, offset: 3968 },
    Magic { nr: 146934374812893252, shift: 59, offset: 4000 },
    Magic { nr: 2254067824989200, shift: 59, offset: 4032 },
    Magic { nr: 6341147448901128192, shift: 59, offset: 4064 },
    Magic { nr: 1166775765715521537, shift: 57, offset: 4096 },
    Magic { nr: 288236983220699648, shift: 57, offset: 4224 },
    Magic { nr: 5841239105073070336, shift: 57, offset: 4352 },
    Magic { nr: 162147195953348933, shift: 57, offset: 4480 },
    Magic { nr: 9158940482970816, shift: 59, offset: 4608 },
    Magic { nr: 3175394031173899, shift: 59, offset: 4640 },
    Magic { nr: 2315140651406336257, shift: 59, offset: 4672 },
    Magic { nr: 289431051580617032, shift: 59, offset: 4704 },
    Magic { nr: 2203337163780, shift: 59, offset: 4736 },
    Magic { nr: 144117427968672256, shift: 59, offset: 4768 },
    Magic { nr: 4612354796511429632, shift: 59, offset: 4800 },
    Magic { nr: 115036538306560, shift: 59, offset: 4832 },
    Magic { nr: 6766431433918472, shift: 59, offset: 4864 },
    Magic { nr: 289383016726571072, shift: 59, offset: 4896 },
    Magic { nr: 292752678249832460, shift: 58, offset: 4928 },
    Magic { nr: 2882321904667485192, shift: 59, offset: 4992 },
    Magic { nr: 72063117383960068, shift: 59, offset: 5024 },
    Magic { nr: 144966487136831488, shift: 59, offset: 5056 },
    Magic { nr: 183240760681842945, shift: 59, offset: 5088 },
    Magic { nr: 1188954735643222144, shift: 59, offset: 5120 },
    Magic { nr: 181278819087549196, shift: 59, offset: 5152 },
    Magic { nr: 54052279585770000, shift: 58, offset: 5184 },
];

#[rustfmt::skip]
pub const ROOK_MAGICS: [Magic; NrOf::SQUARES] = [
    Magic { nr: 36029555626000392, shift: 52, offset: 0 },
    Magic { nr: 18014708015702016, shift: 53, offset: 4096 },
    Magic { nr: 72092780976934976, shift: 53, offset: 6144 },
    Magic { nr: 2918341363564019840, shift: 53, offset: 8192 },
    Magic { nr: 9907927976886938624, shift: 53, offset: 10240 },
    Magic { nr: 144116356374462488, shift: 53, offset: 12288 },
    Magic { nr: 36100265283160576, shift: 53, offset: 14336 },
    Magic { nr: 72064745158493218, shift: 52, offset: 16384 },
    Magic { nr: 9148083845824512, shift: 53, offset: 20480 },
    Magic { nr: 651051759644508554, shift: 54, offset: 22528 },
    Magic { nr: 19422050955301120, shift: 54, offset: 23552 },
    Magic { nr: 612771127646757124, shift: 54, offset: 24576 },
    Magic { nr: 4035788362107356200, shift: 54, offset: 25600 },
    Magic { nr: 422229653324288, shift: 54, offset: 26624 },
    Magic { nr: 5066653803028992, shift: 54, offset: 27648 },
    Magic { nr: 871587404971036928, shift: 53, offset: 28672 },
    Magic { nr: 36033745366564864, shift: 53, offset: 30720 },
    Magic { nr: 11400287386607616, shift: 54, offset: 32768 },
    Magic { nr: 11529358532604862466, shift: 54, offset: 33792 },
    Magic { nr: 142387024250880, shift: 54, offset: 34816 },
    Magic { nr: 141287311284224, shift: 54, offset: 35840 },
    Magic { nr: 1441856118123922432, shift: 54, offset: 36864 },
    Magic { nr: 1152943494856708114, shift: 54, offset: 37888 },
    Magic { nr: 2199568597252, shift: 53, offset: 38912 },
    Magic { nr: 288722959508473131, shift: 53, offset: 40960 },
    Magic { nr: 9024809694674976, shift: 54, offset: 43008 },
    Magic { nr: 18295946504871940, shift: 54, offset: 44032 },
    Magic { nr: 2341908094413967360, shift: 54, offset: 45056 },
    Magic { nr: 708087644162048, shift: 54, offset: 46080 },
    Magic { nr: 4616194018257142272, shift: 54, offset: 47104 },
    Magic { nr: 61080074241310852, shift: 54, offset: 48128 },
    Magic { nr: 74310261436093445, shift: 53, offset: 49152 },
    Magic { nr: 4791846633673327168, shift: 53, offset: 51200 },
    Magic { nr: 146402173339570176, shift: 54, offset: 53248 },
    Magic { nr: 239117528243113984, shift: 54, offset: 54272 },
    Magic { nr: 169466541379586, shift: 54, offset: 55296 },
    Magic { nr: 2308235598063732736, shift: 54, offset: 56320 },
    Magic { nr: 19157907916529728, shift: 54, offset: 57344 },
    Magic { nr: 5215205766989875216, shift: 54, offset: 58368 },
    Magic { nr: 1157716766906916993, shift: 53, offset: 59392 },
    Magic { nr: 585614333708763136, shift: 53, offset: 61440 },
    Magic { nr: 18298073121947716, shift: 54, offset: 63488 },
    Magic { nr: 27021816811815008, shift: 54, offset: 64512 },
    Magic { nr: 11530622558659739658, shift: 54, offset: 65536 },
    Magic { nr: 10141899566678032, shift: 54, offset: 66560 },
    Magic { nr: 145241156836524544, shift: 54, offset: 67584 },
    Magic { nr: 83334219685953537, shift: 54, offset: 68608 },
    Magic { nr: 282041941884931, shift: 53, offset: 69632 },
    Magic { nr: 72479952536144384, shift: 53, offset: 71680 },
    Magic { nr: 90213155258503936, shift: 54, offset: 73728 },
    Magic { nr: 17592722948480, shift: 54, offset: 74752 },
    Magic { nr: 563272354906624, shift: 54, offset: 75776 },
    Magic { nr: 1152974590403477760, shift: 54, offset: 76800 },
    Magic { nr: 37156896553074816, shift: 54, offset: 77824 },
    Magic { nr: 9225641433286706176, shift: 54, offset: 78848 },
    Magic { nr: 16141007796110918144, shift: 53, offset: 79872 },
    Magic { nr: 4647715916032778369, shift: 52, offset: 81920 },
    Magic { nr: 2359904347225063681, shift: 53, offset: 86016 },
    Magic { nr: 3458835995097563665, shift: 53, offset: 88064 },
    Magic { nr: 290763788515545089, shift: 53, offset: 90112 },
    Magic { nr: 333547881762000901, shift: 53, offset: 92160 },
    Magic { nr: 2434758892279366662, shift: 53, offset: 94208 },
    Magic { nr: 8833171718404, shift: 53, offset: 96256 },
    Magic { nr: 2310913959005265986, shift: 52, offset: 98304 },
];