//! Test vectors for the secp521r1 group.

use hex_literal::hex;

/// Repeated addition of the generator.
///
/// These are the first 20 test vectors for P-521 from: <http://point-at-infinity.org/ecc/nisttv>
pub const ADD_TEST_VECTORS: &[([u8; 66], [u8; 66])] = &[
    (
        hex!("00C6858E06B70404E9CD9E3ECB662395B4429C648139053FB521F828AF606B4D3DBAA14B5E77EFE75928FE1DC127A2FFA8DE3348B3C1856A429BF97E7E31C2E5BD66"),
        hex!("011839296A789A3BC0045C8A5FB42C7D1BD998F54449579B446817AFBD17273E662C97EE72995EF42640C550B9013FAD0761353C7086A272C24088BE94769FD16650"),
    ),
    (
        hex!("00433C219024277E7E682FCB288148C282747403279B1CCC06352C6E5505D769BE97B3B204DA6EF55507AA104A3A35C5AF41CF2FA364D60FD967F43E3933BA6D783D"),
        hex!("00F4BB8CC7F86DB26700A7F3ECEEEED3F0B5C6B5107C4DA97740AB21A29906C42DBBB3E377DE9F251F6B93937FA99A3248F4EAFCBE95EDC0F4F71BE356D661F41B02"),
    ),
    (
        hex!("01A73D352443DE29195DD91D6A64B5959479B52A6E5B123D9AB9E5AD7A112D7A8DD1AD3F164A3A4832051DA6BD16B59FE21BAEB490862C32EA05A5919D2EDE37AD7D"),
        hex!("013E9B03B97DFA62DDD9979F86C6CAB814F2F1557FA82A9D0317D2F8AB1FA355CEEC2E2DD4CF8DC575B02D5ACED1DEC3C70CF105C9BC93A590425F588CA1EE86C0E5"),
    ),
    (
        hex!("0035B5DF64AE2AC204C354B483487C9070CDC61C891C5FF39AFC06C5D55541D3CEAC8659E24AFE3D0750E8B88E9F078AF066A1D5025B08E5A5E2FBC87412871902F3"),
        hex!("0082096F84261279D2B673E0178EB0B4ABB65521AEF6E6E32E1B5AE63FE2F19907F279F283E54BA385405224F750A95B85EEBB7FAEF04699D1D9E21F47FC346E4D0D"),
    ),
    (
        hex!("00652BF3C52927A432C73DBC3391C04EB0BF7A596EFDB53F0D24CF03DAB8F177ACE4383C0C6D5E3014237112FEAF137E79A329D7E1E6D8931738D5AB5096EC8F3078"),
        hex!("015BE6EF1BDD6601D6EC8A2B73114A8112911CD8FE8E872E0051EDD817C9A0347087BB6897C9072CF374311540211CF5FF79D1F007257354F7F8173CC3E8DEB090CB"),
    ),
    (
        hex!("01EE4569D6CDB59219532EFF34F94480D195623D30977FD71CF3981506ADE4AB01525FBCCA16153F7394E0727A239531BE8C2F66E95657F380AE23731BEDF79206B9"),
        hex!("01DE0255AD0CC64F586AE2DD270546E3B1112AABBB73DA5A808E7240A926201A8A96CAB72D0E56648C9DF96C984DE274F2203DC7B8B55CA0DADE1EACCD7858D44F17"),
    ),
    (
        hex!("0056D5D1D99D5B7F6346EEB65FDA0B073A0C5F22E0E8F5483228F018D2C2F7114C5D8C308D0ABFC698D8C9A6DF30DCE3BBC46F953F50FDC2619A01CEAD882816ECD4"),
        hex!("003D2D1B7D9BAAA2A110D1D8317A39D68478B5C582D02824F0DD71DBD98A26CBDE556BD0F293CDEC9E2B9523A34591CE1A5F9E76712A5DDEFC7B5C6B8BC90525251B"),
    ),
    (
        hex!("000822C40FB6301F7262A8348396B010E25BD4E29D8A9B003E0A8B8A3B05F826298F5BFEA5B8579F49F08B598C1BC8D79E1AB56289B5A6F4040586F9EA54AA78CE68"),
        hex!("016331911D5542FC482048FDAB6E78853B9A44F8EDE9E2C0715B5083DE610677A8F189E9C0AA5911B4BFF0BA0DF065C578699F3BA940094713538AD642F11F17801C"),
    ),
    (
        hex!("01585389E359E1E21826A2F5BF157156D488ED34541B988746992C4AB145B8C6B6657429E1396134DA35F3C556DF725A318F4F50BABD85CD28661F45627967CBE207"),
        hex!("002A2E618C9A8AEDF39F0B55557A27AE938E3088A654EE1CEBB6C825BA263DDB446E0D69E5756057AC840FF56ECF4ABFD87D736C2AE928880F343AA0EA86B9AD2A4E"),
    ),
    (
        hex!("0190EB8F22BDA61F281DFCFE7BB6721EC4CD901D879AC09AC7C34A9246B11ADA8910A2C7C178FCC263299DAA4DA9842093F37C2E411F1A8E819A87FF09A04F2F3320"),
        hex!("01EB5D96B8491614BA9DBAEAB3B0CA2BA760C2EEB2144251B20BA97FD78A62EF62D2BF5349D44D9864BB536F6163DC57EBEFF3689639739FAA172954BC98135EC759"),
    ),
    (
        hex!("008A75841259FDEDFF546F1A39573B4315CFED5DC7ED7C17849543EF2C54F2991652F3DBC5332663DA1BD19B1AEBE3191085015C024FA4C9A902ECC0E02DDA0CDB9A"),
        hex!("0096FB303FCBBA2129849D0CA877054FB2293ADD566210BD0493ED2E95D4E0B9B82B1BC8A90E8B42A4AB3892331914A95336DCAC80E3F4819B5D58874F92CE48C808"),
    ),
    (
        hex!("01C0D9DCEC93F8221C5DE4FAE9749C7FDE1E81874157958457B6107CF7A5967713A644E90B7C3FB81B31477FEE9A60E938013774C75C530928B17BE69571BF842D8C"),
        hex!("014048B5946A4927C0FE3CE1D103A682CA4763FE65AB71494DA45E404ABF6A17C097D6D18843D86FCDB6CC10A6F951B9B630884BA72224F5AE6C79E7B1A3281B17F0"),
    ),
    (
        hex!("007E3E98F984C396AD9CD7865D2B4924861A93F736CDE1B4C2384EEDD2BEAF5B866132C45908E03C996A3550A5E79AB88EE94BEC3B00AB38EFF81887848D32FBCDA7"),
        hex!("0108EE58EB6D781FEDA91A1926DAA3ED5A08CED50A386D5421C69C7A67AE5C1E212AC1BD5D5838BC763F26DFDD351CBFBBC36199EAAF9117E9F7291A01FB022A71C9"),
    ),
    (
        hex!("01875BC7DC551B1B65A9E1B8CCFAAF84DED1958B401494116A2FD4FB0BABE0B3199974FC06C8B897222D79DF3E4B7BC744AA6767F6B812EFBF5D2C9E682DD3432D74"),
        hex!("005CA4923575DACB5BD2D66290BBABB4BDFB8470122B8E51826A0847CE9B86D7ED62D07781B1B4F3584C11E89BF1D133DC0D5B690F53A87C84BE41669F852700D54A"),
    ),
    (
        hex!("006B6AD89ABCB92465F041558FC546D4300FB8FBCC30B40A0852D697B532DF128E11B91CCE27DBD00FFE7875BD1C8FC0331D9B8D96981E3F92BDE9AFE337BCB8DB55"),
        hex!("01B468DA271571391D6A7CE64D2333EDBF63DF0496A9BAD20CBA4B62106997485ED57E9062C899470A802148E2232C96C99246FD90CC446ABDD956343480A1475465"),
    ),
    (
        hex!("01D17D10D8A89C8AD05DDA97DA26AC743B0B2A87F66192FD3F3DD632F8D20B188A52943FF18861CA00A0E5965DA7985630DF0DBF5C8007DCDC533A6C508F81A8402F"),
        hex!("007A37343C582D77001FC714B18D3D3E69721335E4C3B800D50EC7CA30C94B6B82C1C182E1398DB547AA0B3075AC9D9988529E3004D28D18633352E272F89BC73ABE"),
    ),
    (
        hex!("01B00DDB707F130EDA13A0B874645923906A99EE9E269FA2B3B4D66524F269250858760A69E674FE0287DF4E799B5681380FF8C3042AF0D1A41076F817A853110AE0"),
        hex!("0085683F1D7DB16576DBC111D4E4AEDDD106B799534CF69910A98D68AC2B22A1323DF9DA564EF6DD0BF0D2F6757F16ADF420E6905594C2B755F535B9CB7C70E64647"),
    ),
    (
        hex!("01BC33425E72A12779EACB2EDCC5B63D1281F7E86DBC7BF99A7ABD0CFE367DE4666D6EDBB8525BFFE5222F0702C3096DEC0884CE572F5A15C423FDF44D01DD99C61D"),
        hex!("010D06E999885B63535DE3E74D33D9E63D024FB07CE0D196F2552C8E4A00AC84C044234AEB201F7A9133915D1B4B45209B9DA79FE15B19F84FD135D841E2D8F9A86A"),
    ),
    (
        hex!("00998DCCE486419C3487C0F948C2D5A1A07245B77E0755DF547EFFF0ACDB3790E7F1FA3B3096362669679232557D7A45970DFECF431E725BBDE478FF0B2418D6A19B"),
        hex!("0137D5DA0626A021ED5CC3942497535B245D67D28AEE2B7BCF4ACC50EEE36545772773AD963FF2EB8CF9B0EC39991631C377F5A4D89EA9FBFE44A9091A695BFD0575"),
    ),
    (
        hex!("018BDD7F1B889598A4653DEEAE39CC6F8CC2BD767C2AB0D93FB12E968FBED342B51709506339CB1049CB11DD48B9BDB3CD5CAD792E43B74E16D8E2603BFB11B0344F"),
        hex!("00C5AADBE63F68CA5B6B6908296959BF0AF89EE7F52B410B9444546C550952D311204DA3BDDDC6D4EAE7EDFAEC1030DA8EF837CCB22EEE9CFC94DD3287FED0990F94"),
    )
];

/// Scalar multiplication with the generator.
///
/// These are the test vectors for P-521 from <http://point-at-infinity.org/ecc/nisttv>
/// that are not part of [`ADD_TEST_VECTORS`].
pub const MUL_TEST_VECTORS: &[([u8; 66], [u8; 66], [u8; 66])] = &[
    (
        hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018ebbb95eed0e13"),
        hex!("01650048FBD63E8C30B305BF36BD7643B91448EF2206E8A0CA84A140789A99B0423A0A2533EA079CA7E049843E69E5FA2C25A163819110CEC1A30ACBBB3A422A40D8"),
        hex!("010C9C64A0E0DB6052DBC5646687D06DECE5E9E0703153EFE9CB816FE025E85354D3C5F869D6DB3F4C0C01B5F97919A5E72CEEBE03042E5AA99112691CFFC2724828"),
    ),
    (
        hex!("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000159d893d4cdd747246cdca43590e13"),
        hex!("017E1370D39C9C63925DAEEAC571E21CAAF60BD169191BAEE8352E0F54674443B29786243564ABB705F6FC0FE5FC5D3F98086B67CA0BE7AC8A9DEC421D9F1BC6B37F"),
        hex!("01CD559605EAD19FBD99E83600A6A81A0489E6F20306EE0789AE00CE16A6EFEA2F42F7534186CF1C60DF230BD9BCF8CB95E5028AD9820B2B1C0E15597EE54C4614A6"),
    ),
    (
        hex!("0083ff83fffffc03fff80007fffc000f8003ffe00007ffe0fffc000f8000000007ffffff00ffff000ffffff001fffc000000001c0000400000003803ffffffcfffff"),
        hex!("00B45CB84651C9D4F08858B867F82D816E84E94FE4CAE3DA5F65E420B08398D0C5BF019253A6C26D20671BDEF0B8E6C1D348A4B0734687F73AC6A4CBB2E085C68B3F"),
        hex!("01C84942BBF538903062170A4BA8B3410D385719BA2037D29CA5248BFCBC8478220FEC79244DCD45D31885A1764DEE479CE20B12CEAB62F9001C7AA4282CE4BE7F56"),
    ),
    (
        hex!("000001ffffe03ffff1ffff801fffffffff8000001fffff800ffffff8001fffc7ffff820000040007ffffe000001f800007fffffffc00001c007c0007000070000007"),
        hex!("01CCEF4CDA108CEBE6568820B54A3CA3A3997E4EF0EDA6C350E7ED3DBB1861EDD80181C650CEBE5440FEBA880F9C8A7A86F8B82659794F6F5B88E501E5DD84E65D7E"),
        hex!("01026565F8B195D03C3F6139C3A63EAA1C29F7090AB2A8F75027939EC05109035F1B38E6C508E0C14CE53AB7E2DA33AA28140EDBF3964862FB157119517454E60F07"),
    ),
    (
        hex!("00007fffffe0003f00000007f00007ffff80000000001ffc000000fff030001f0000fffff0000038000000000002003f007ffffff0000000000000ffe00000000000"),
        hex!("00C1002DC2884EEDADB3F9B468BBEBD55980799852C506D37271FFCD006919DB3A96DF8FE91EF6ED4B9081B1809E8F2C2B28AF5FCBF524147C73CB0B913D6FAB0995"),
        hex!("01614E8A62C8293DD2AA6EF27D30974A4FD185019FA8EF4F982DA48698CECF706581F69EE9ED67A9C231EC9D0934D0F674646153273BCBB345E923B1EC1386A1A4AD"),
    ),
    (
        hex!("00001fffc1000003fe0003ffffffe0001ffffffffffffff00001fffff83ffffffffffffe003ffffffffffff7ffffc03ff807fffe0001fffff800fff800001ffff000"),
        hex!("010ED3E085ECDE1E66874286B5D5642B9D37853A026A0A025C7B84936E2ECEEC5F342E14C80C79CCF814D5AD085C5303F2823251F2B9276F88C9D7A43E387EBD87AC"),
        hex!("01BE399A7666B29E79BBF3D277531A97CE05CAC0B49BECE4781E7AEE0D6E80FEE883C76E9F08453DC1ADE4E49300F3D56FEE6A1510DA1B1F12EEAA39A05AA0508119"),
    ),
    (
        hex!("000000000fff80fffffffe03fffc07fffc800070000fc0007ffc00000000000fffe1fbff81ff9fffff81fff81fc000000000ff801fffc0f81f01fff8001fc005ffff"),
        hex!("013070A29B059D317AF37089E40FCB135868F52290EFF3E9F3E32CDADCA18EA234D8589C665A4B8E3D0714DE004A419DEA7091A3BBA97263C438FE9413AA598FD4A5"),
        hex!("00238A27FD9E5E7324C8B538EF2E334B71AC2611A95F42F4F2544D8C4A65D2A32A8BAFA15EFD4FC2BD8AB2B0C51F65B680879589F4D5FE8A84CEB17A2E8D3587F011"),
    ),
    (
        hex!("000fffffc01ffffff01fffffe000000fc0ffffff00063ffdffbffff87ffffffffe03fffffffff0000000000ff8001f8000000008007ff800003ffff00000fffc01ff"),
        hex!("01A3D88799878EC74E66FF1AD8C7DFA9A9B4445A17F0810FF8189DD27AE3B6C580D352476DBDAEB08D7DA0DE3866F7C7FDBEBB8418E19710F1F7AFA88C22280B1404"),
        hex!("00B39703D2053EC7B8812BDFEBFD81B4CB76F245FE535A1F1E46801C35DE03C15063A99A203981529C146132863CA0E68544D0F0A638D8A2859D82B4DD266F27C3AE"),
    ),
    (
        hex!("000000003ffe001ffffffc7ffe00000000fffbff00000007ffe00ffffff803ffffff3ffffc003f8000000007fe03ff8000fff8007ffffffffc0003ffe0001fc0000f"),
        hex!("01D16B4365DEFE6FD356DC1F31727AF2A32C7E86C5AE87ED2950A08BC8653F203C7F7860E80F95AA27C93EA76E8CD094127B15ED42CC5F96DC0A0F9A1C1E31D0D526"),
        hex!("006E3710A0F9366E0BB8A14FFE8EBC2722EECF4A123EC9BA98DCCCA335D6FAFD289DC69FD90903C9AC982FEB46DF93F03A7C8C9549D32C1C386D17F37340E63822A8"),
    ),
    (
        hex!("00007f0000003ffc00000001fff007fff008000000ff0000000fffc03fffffff800000030fff80fe00000000c00001ffff8001ffffffffe0000000000003fffffff3"),
        hex!("01B1220F67C985E9FC9C588C0C86BB16E6FE4CC11E168A98D701AE4670724B3D030ED9965FADF4207C7A1BE9BE0F40DEF2BBFFF0C7EABCB5B42526CE1D3CAA468F52"),
        hex!("006CDAD2860F6D2C37159A5A866D11605F2E7D87430DCFE6E6816AB6423CD9003CA6F2527B9C2A2483C541D456C963D18A0D2A46E158CB2A44C0BF42D562881FB748"),
    ),
    (
        hex!("00f07f80ffffff00003ff8003ff87fffff007fe07e0000003ffffff80007fe0000000000000003fc00000000007ffc07ff807f7f1fffef07fffff8000000000003ff"),
        hex!("00F25E545213C8C074BE38A0612EA9B66336B14A874372548D9716392DFA31CD0D13E94F86CD48B8D43B80B5299144E01245C873B39F6AC6C4FB397746AF034AD67C"),
        hex!("01733ABB21147CC27E35F41FAF40290AFD1EEB221D983FFABBD88E5DC8776450A409EACDC1BCA2B9F517289C68645BB96781808FEAE42573C2BB289F16E2AECECE17"),
    ),
    (
        hex!("000000000003fff7ffffffffffffffe007ffffffe3fffffffffc01ffe0001fe01fffffff0000000000ffffffc0000000007ffffff03ff8000000000000c000000000"),
        hex!("0172CD22CBE0634B6BFEE24BB1D350F384A945ED618ECAD48AADC6C1BC0DCC107F0FFE9FE14DC929F90153F390C25BE5D3A73A56F9ACCB0C72C768753869732D0DC4"),
        hex!("00D249CFB570DA4CC48FB5426A928B43D7922F787373B6182408FBC71706E7527E8414C79167F3C999FF58DE352D238F1FE7168C658D338F72696F2F889A97DE23C5"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863f5"),
        hex!("018BDD7F1B889598A4653DEEAE39CC6F8CC2BD767C2AB0D93FB12E968FBED342B51709506339CB1049CB11DD48B9BDB3CD5CAD792E43B74E16D8E2603BFB11B0344F"),
        hex!("013A552419C09735A49496F7D696A640F50761180AD4BEF46BBBAB93AAF6AD2CEEDFB25C4222392B1518120513EFCF257107C8334DD11163036B22CD78012F66F06B"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863f6"),
        hex!("00998DCCE486419C3487C0F948C2D5A1A07245B77E0755DF547EFFF0ACDB3790E7F1FA3B3096362669679232557D7A45970DFECF431E725BBDE478FF0B2418D6A19B"),
        hex!("00C82A25F9D95FDE12A33C6BDB68ACA4DBA2982D7511D48430B533AF111C9ABA88D88C5269C00D1473064F13C666E9CE3C880A5B2761560401BB56F6E596A402FA8A"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863f7"),
        hex!("01BC33425E72A12779EACB2EDCC5B63D1281F7E86DBC7BF99A7ABD0CFE367DE4666D6EDBB8525BFFE5222F0702C3096DEC0884CE572F5A15C423FDF44D01DD99C61D"),
        hex!("00F2F9166677A49CACA21C18B2CC2619C2FDB04F831F2E690DAAD371B5FF537B3FBBDCB514DFE0856ECC6EA2E4B4BADF646258601EA4E607B02ECA27BE1D27065795"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863f8"),
        hex!("01B00DDB707F130EDA13A0B874645923906A99EE9E269FA2B3B4D66524F269250858760A69E674FE0287DF4E799B5681380FF8C3042AF0D1A41076F817A853110AE0"),
        hex!("017A97C0E2824E9A89243EEE2B1B51222EF94866ACB30966EF56729753D4DD5ECDC20625A9B10922F40F2D098A80E9520BDF196FAA6B3D48AA0ACA4634838F19B9B8"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863f9"),
        hex!("01D17D10D8A89C8AD05DDA97DA26AC743B0B2A87F66192FD3F3DD632F8D20B188A52943FF18861CA00A0E5965DA7985630DF0DBF5C8007DCDC533A6C508F81A8402F"),
        hex!("0185C8CBC3A7D288FFE038EB4E72C2C1968DECCA1B3C47FF2AF13835CF36B4947D3E3E7D1EC6724AB855F4CF8A53626677AD61CFFB2D72E79CCCAD1D8D076438C541"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863fa"),
        hex!("006B6AD89ABCB92465F041558FC546D4300FB8FBCC30B40A0852D697B532DF128E11B91CCE27DBD00FFE7875BD1C8FC0331D9B8D96981E3F92BDE9AFE337BCB8DB55"),
        hex!("004B9725D8EA8EC6E2958319B2DCCC12409C20FB6956452DF345B49DEF9668B7A12A816F9D3766B8F57FDEB71DDCD369366DB9026F33BB954226A9CBCB7F5EB8AB9A"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863fb"),
        hex!("01875BC7DC551B1B65A9E1B8CCFAAF84DED1958B401494116A2FD4FB0BABE0B3199974FC06C8B897222D79DF3E4B7BC744AA6767F6B812EFBF5D2C9E682DD3432D74"),
        hex!("01A35B6DCA8A2534A42D299D6F44544B42047B8FEDD471AE7D95F7B831647928129D2F887E4E4B0CA7B3EE17640E2ECC23F2A496F0AC57837B41BE99607AD8FF2AB5"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863fc"),
        hex!("007E3E98F984C396AD9CD7865D2B4924861A93F736CDE1B4C2384EEDD2BEAF5B866132C45908E03C996A3550A5E79AB88EE94BEC3B00AB38EFF81887848D32FBCDA7"),
        hex!("00F711A7149287E01256E5E6D9255C12A5F7312AF5C792ABDE3963859851A3E1DED53E42A2A7C74389C0D92022CAE340443C9E6615506EE81608D6E5FE04FDD58E36"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863fd"),
        hex!("01C0D9DCEC93F8221C5DE4FAE9749C7FDE1E81874157958457B6107CF7A5967713A644E90B7C3FB81B31477FEE9A60E938013774C75C530928B17BE69571BF842D8C"),
        hex!("00BFB74A6B95B6D83F01C31E2EFC597D35B89C019A548EB6B25BA1BFB54095E83F68292E77BC2790324933EF5906AE4649CF77B458DDDB0A519386184E5CD7E4E80F"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863fe"),
        hex!("008A75841259FDEDFF546F1A39573B4315CFED5DC7ED7C17849543EF2C54F2991652F3DBC5332663DA1BD19B1AEBE3191085015C024FA4C9A902ECC0E02DDA0CDB9A"),
        hex!("016904CFC03445DED67B62F35788FAB04DD6C522A99DEF42FB6C12D16A2B1F4647D4E43756F174BD5B54C76DCCE6EB56ACC923537F1C0B7E64A2A778B06D31B737F7"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e913863ff"),
        hex!("0190EB8F22BDA61F281DFCFE7BB6721EC4CD901D879AC09AC7C34A9246B11ADA8910A2C7C178FCC263299DAA4DA9842093F37C2E411F1A8E819A87FF09A04F2F3320"),
        hex!("0014A26947B6E9EB456245154C4F35D4589F3D114DEBBDAE4DF4568028759D109D2D40ACB62BB2679B44AC909E9C23A814100C9769C68C6055E8D6AB4367ECA138A6"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386400"),
        hex!("01585389E359E1E21826A2F5BF157156D488ED34541B988746992C4AB145B8C6B6657429E1396134DA35F3C556DF725A318F4F50BABD85CD28661F45627967CBE207"),
        hex!("01D5D19E736575120C60F4AAAA85D8516C71CF7759AB11E3144937DA45D9C224BB91F2961A8A9FA8537BF00A9130B54027828C93D516D777F0CBC55F15794652D5B1"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386401"),
        hex!("000822C40FB6301F7262A8348396B010E25BD4E29D8A9B003E0A8B8A3B05F826298F5BFEA5B8579F49F08B598C1BC8D79E1AB56289B5A6F4040586F9EA54AA78CE68"),
        hex!("009CCE6EE2AABD03B7DFB7025491877AC465BB0712161D3F8EA4AF7C219EF988570E76163F55A6EE4B400F45F20F9A3A879660C456BFF6B8ECAC7529BD0EE0E87FE3"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386402"),
        hex!("0056D5D1D99D5B7F6346EEB65FDA0B073A0C5F22E0E8F5483228F018D2C2F7114C5D8C308D0ABFC698D8C9A6DF30DCE3BBC46F953F50FDC2619A01CEAD882816ECD4"),
        hex!("01C2D2E48264555D5EEF2E27CE85C6297B874A3A7D2FD7DB0F228E242675D93421AA942F0D6C321361D46ADC5CBA6E31E5A061898ED5A2210384A3947436FADADAE4"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386403"),
        hex!("01EE4569D6CDB59219532EFF34F94480D195623D30977FD71CF3981506ADE4AB01525FBCCA16153F7394E0727A239531BE8C2F66E95657F380AE23731BEDF79206B9"),
        hex!("0021FDAA52F339B0A7951D22D8FAB91C4EEED554448C25A57F718DBF56D9DFE575693548D2F1A99B7362069367B21D8B0DDFC238474AA35F2521E1533287A72BB0E8"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386404"),
        hex!("00652BF3C52927A432C73DBC3391C04EB0BF7A596EFDB53F0D24CF03DAB8F177ACE4383C0C6D5E3014237112FEAF137E79A329D7E1E6D8931738D5AB5096EC8F3078"),
        hex!("00A41910E42299FE291375D48CEEB57EED6EE327017178D1FFAE1227E8365FCB8F7844976836F8D30C8BCEEABFDEE30A00862E0FF8DA8CAB0807E8C33C17214F6F34"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386405"),
        hex!("0035B5DF64AE2AC204C354B483487C9070CDC61C891C5FF39AFC06C5D55541D3CEAC8659E24AFE3D0750E8B88E9F078AF066A1D5025B08E5A5E2FBC87412871902F3"),
        hex!("017DF6907BD9ED862D498C1FE8714F4B5449AADE5109191CD1E4A519C01D0E66F80D860D7C1AB45C7ABFADDB08AF56A47A114480510FB9662E261DE0B803CB91B2F2"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386406"),
        hex!("01A73D352443DE29195DD91D6A64B5959479B52A6E5B123D9AB9E5AD7A112D7A8DD1AD3F164A3A4832051DA6BD16B59FE21BAEB490862C32EA05A5919D2EDE37AD7D"),
        hex!("00C164FC4682059D2226686079393547EB0D0EAA8057D562FCE82D0754E05CAA3113D1D22B30723A8A4FD2A5312E213C38F30EFA36436C5A6FBDA0A7735E11793F1A"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386407"),
        hex!("00433C219024277E7E682FCB288148C282747403279B1CCC06352C6E5505D769BE97B3B204DA6EF55507AA104A3A35C5AF41CF2FA364D60FD967F43E3933BA6D783D"),
        hex!("010B44733807924D98FF580C1311112C0F4A394AEF83B25688BF54DE5D66F93BD2444C1C882160DAE0946C6C805665CDB70B1503416A123F0B08E41CA9299E0BE4FD"),
    ),
    (
        hex!("01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386408"),
        hex!("00C6858E06B70404E9CD9E3ECB662395B4429C648139053FB521F828AF606B4D3DBAA14B5E77EFE75928FE1DC127A2FFA8DE3348B3C1856A429BF97E7E31C2E5BD66"),
        hex!("00E7C6D6958765C43FFBA375A04BD382E426670ABBB6A864BB97E85042E8D8C199D368118D66A10BD9BF3AAF46FEC052F89ECAC38F795D8D3DBF77416B89602E99AF"),
    )
];
