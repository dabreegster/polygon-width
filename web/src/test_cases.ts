export let testCases = {
  // Manually drawn
  "just a rectangle":
    "POLYGON((-0.1008378 51.4893734,-0.099282 51.4898419,-0.0992257 51.4897652,-0.1007816 51.4893016,-0.1008378 51.4893734))",
  "simple L":
    "POLYGON((-0.1021076 51.4896734,-0.1015438 51.489379,-0.10134 51.4895191,-0.101303 51.4894982,-0.1015399 51.4893283,-0.102142 51.4896457,-0.1021076 51.4896734))",
  "many width changes":
    "POLYGON((-0.1018838 51.4895115,-0.1017193 51.4894192,-0.1016939 51.4894361,-0.1016541 51.4894124,-0.1016219 51.4894303,-0.1015788 51.489405,-0.1016067 51.4893771,-0.1014222 51.4892885,-0.1015086 51.4892326,-0.1016694 51.4893138,-0.1016423 51.4893317,-0.1016804 51.4893581,-0.1017074 51.4893381,-0.1018361 51.4894213,-0.1018818 51.4894061,-0.1019284 51.4894366,-0.1019038 51.4894609,-0.1019443 51.4894764,-0.1018838 51.4895115))",
  "very wide and uneven":
    "POLYGON((13.3562208 52.5297463,13.3560897 52.5299508,13.3569956 52.5304672,13.3562493 52.5304083,13.3558676 52.5300582,13.3555314 52.5296423,13.3562094 52.5294482,13.3568987 52.5294309,13.3577818 52.5295522,13.3583401 52.52993,13.3587674 52.5301692,13.3584882 52.5303771,13.3584939 52.5307341,13.3582318 52.5308485,13.3577989 52.5304568,13.3574285 52.5298918,13.3562208 52.5297463))",

  // from OSM area:highway
  "curvy L":
    "POLYGON((-0.1243813 51.50106430000000302,-0.12479800000000001 51.50107640000000231,-0.124885 51.5010789000000031,-0.12497220000000001 51.50108149999999796,-0.1253871 51.50109350000000319,-0.12541060000000001 51.50109570000000048,-0.12542239999999999 51.50110159999999837,-0.12542890000000001 51.50111030000000056,-0.12543509999999999 51.50112359999999967,-0.12545809999999999 51.50110420000000033,-0.12545709999999999 51.50107719999999745,-0.12545609999999999 51.5010667999999967,-0.12544810000000001 51.5010594000000026,-0.1254353 51.50105560000000082,-0.1246506 51.50103409999999826,-0.12443170000000001 51.50102569999999957,-0.12438639999999999 51.50102669999999705,-0.1243496 51.50103200000000214,-0.12431739999999999 51.50104309999999685,-0.12429270000000001 51.50105359999999877,-0.12426570000000001 51.50106999999999857,-0.1242398 51.50109479999999706,-0.1242162 51.50114039999999704,-0.1241848 51.50121070000000145,-0.1242273 51.50124189999999658,-0.124194 51.50146910000000133,-0.1241453 51.50149749999999926,-0.1241608 51.50152549999999962,-0.1242166 51.50152560000000079,-0.12429419999999999 51.50152539999999846,-0.1243064 51.50136369999999886,-0.1243146 51.50125529999999685,-0.12432699999999999 51.50109170000000347,-0.124332 51.50107959999999707,-0.12434290000000001 51.50107030000000208,-0.12435839999999999 51.50106569999999806,-0.1243813 51.50106430000000302))",
  "long T":
    "POLYGON((-0.1260973 51.5058446000000032,-0.12657669999999999 51.50572009999999779,-0.12660589999999999 51.50571370000000115,-0.1266294 51.50571430000000106,-0.12665290000000001 51.50571970000000022,-0.12668209999999999 51.50573920000000072,-0.12663720000000001 51.50564150000000296,-0.12659709999999999 51.50555210000000272,-0.12659200000000001 51.50557280000000304,-0.1265868 51.50558379999999659,-0.12657009999999999 51.50560089999999747,-0.12652959999999999 51.50562939999999656,-0.12645680000000001 51.50565790000000277,-0.12637019999999999 51.50568280000000243,-0.124559 51.50615839999999679,-0.1245126 51.50616699999999781,-0.1244765 51.50617210000000057,-0.1244473 51.506171700000003,-0.1244131 51.506165799999998,-0.124389 51.50615549999999843,-0.1243693 51.50613859999999988,-0.1243541 51.50611800000000073,-0.1243522 51.50609990000000238,-0.12437719999999999 51.50603350000000091,-0.1244817 51.50577839999999696,-0.1247093 51.50522269999999736,-0.1247944 51.50521030000000167,-0.1247582 51.5052012000000019,-0.12474109999999999 51.5051910000000035,-0.1247341 51.50517750000000206,-0.12473919999999999 51.50515980000000127,-0.1247856 51.50510289999999713,-0.1247409 51.5050965000000005,-0.12468310000000001 51.50508820000000298,-0.1246582 51.50514559999999875,-0.12468319999999999 51.50514960000000286,-0.12442640000000001 51.50576829999999973,-0.1243972 51.50583859999999703,-0.1243731 51.50583389999999895,-0.1242501 51.50613150000000218,-0.12421840000000001 51.50619970000000336,-0.12420639999999999 51.50622260000000097,-0.1241867 51.50624040000000292,-0.1241601 51.50625649999999922,-0.12413730000000001 51.50626439999999917,-0.1241252 51.50626760000000104,-0.1240834 51.50625930000000352,-0.1236097 51.5063847999999993,-0.1236232 51.50640489999999971,-0.1236227 51.50641130000000345,-0.1236408 51.50643889999999914,-0.1236559 51.50646179999999674,-0.12366489999999999 51.50646600000000319,-0.1236685 51.50646929999999912,-0.1236752 51.50647740000000141,-0.12368460000000001 51.5064801999999986,-0.1236993 51.50647850000000005,-0.12549199999999999 51.50601300000000293,-0.1255077 51.50601460000000031,-0.12551889999999999 51.50602099999999695,-0.12552430000000001 51.50603050000000138,-0.12559310000000001 51.5060128000000006,-0.12560540000000001 51.50600959999999873,-0.1256023 51.5060001000000014,-0.12560499999999999 51.50598759999999743,-0.125613 51.50597719999999669,-0.1256225 51.50597049999999655,-0.12562889999999999 51.50596790000000169,-0.1259643 51.50588599999999673,-0.12598429999999999 51.50586229999999688,-0.12606410000000001 51.50584039999999675,-0.1260973 51.5058446000000032))",
  "complex loop":
    "POLYGON((13.37066169999999943 52.52428640000000115,13.37065879999999929 52.52477549999999695,13.3706517999999992 52.52570510000000326,13.37065049999999999 52.52577130000000238,13.37064699999999995 52.52577709999999911,13.37064080000000033 52.52578079999999972,13.37063110000000066 52.52578309999999817,13.37061910000000076 52.52578390000000041,13.3703328999999993 52.52578319999999934,13.37032950000000042 52.52577449999999715,13.37032969999999921 52.52576140000000038,13.37043710000000019 52.52576200000000028,13.3704376000000007 52.52572750000000212,13.37005229999999933 52.52572550000000007,13.37005169999999943 52.52576239999999785,13.37022559999999949 52.52576330000000127,13.3702255000000001 52.52577399999999841,13.37021969999999982 52.525782999999997,13.36828670000000052 52.52577120000000122,13.36826390000000053 52.52576830000000285,13.36824660000000087 52.52576100000000281,13.36823719999999938 52.52574839999999767,13.36823590000000017 52.52573370000000352,13.36823730000000054 52.525621000000001,13.36825750000000035 52.52562110000000217,13.3682692000000003 52.52562050000000227,13.36827730000000081 52.52561670000000049,13.36828260000000057 52.52561039999999792,13.3682824999999994 52.52560220000000157,13.36828670000000052 52.52521670000000142,13.3682856000000001 52.52521089999999759,13.36828019999999917 52.52520609999999834,13.36827489999999941 52.52520340000000232,13.36826650000000072 52.52520220000000251,13.36823919999999966 52.525201899999999,13.36824350000000017 52.52425300000000163,13.36824359999999956 52.52423960000000136,13.36824440000000003 52.524230799999998,13.36824930000000045 52.52422440000000137,13.36826720000000002 52.52421819999999997,13.36830529999999939 52.52421760000000006,13.37063009999999963 52.52422640000000342,13.37065029999999943 52.52422680000000099,13.37066000000000088 52.524230799999998,13.37066489999999952 52.52423819999999921,13.37066169999999943 52.52428640000000115),(13.37062449999999991 52.52428900000000311,13.37045250000000074 52.52429289999999895,13.37029519999999927 52.52429469999999867,13.37028849999999913 52.52429300000000012,13.37023789999999934 52.52436529999999948,13.36991229999999931 52.52436569999999705,13.36991070000000015 52.52436689999999686,13.36973790000000051 52.52436600000000055,13.36972029999999911 52.52436610000000172,13.36970320000000001 52.52436610000000172,13.3696868000000002 52.52436610000000172,13.36966920000000059 52.52436610000000172,13.36965179999999975 52.52436620000000289,13.36963420000000013 52.52436620000000289,13.36961709999999925 52.52436620000000289,13.36958470000000077 52.52436629999999695,13.3695877000000003 52.52436209999999761,13.36958100000000016 52.52436029999999789,13.36962899999999976 52.52429240000000021,13.36929310000000015 52.52429200000000264,13.36928799999999917 52.52429959999999909,13.36927140000000058 52.52432389999999884,13.36912519999999915 52.52432379999999768,13.36908039999999964 52.52432379999999768,13.36900409999999972 52.52432389999999884,13.36892819999999915 52.52432400000000001,13.36835690000000021 52.52432089999999931,13.36834799999999923 52.52562429999999694,13.36834559999999961 52.52565030000000235,13.36829110000000043 52.5257284999999996,13.36859150000000085 52.52573180000000264,13.36860719999999958 52.52571009999999774,13.36869699999999916 52.52558520000000186,13.36934220000000018 52.52568819999999761,13.36931719999999935 52.52572609999999997,13.36941140000000061 52.52572529999999773,13.3695567000000004 52.52572419999999909,13.36961980000000061 52.52572500000000133,13.36983350000000037 52.52570740000000171,13.3699198999999993 52.52570730000000054,13.36998610000000021 52.52570759999999694,13.37055490000000013 52.52570630000000307,13.37056229999999957 52.5247286000000031,13.37056409999999929 52.52468470000000167,13.37057730000000078 52.52435299999999785,13.37062449999999991 52.52428900000000311))",
};
