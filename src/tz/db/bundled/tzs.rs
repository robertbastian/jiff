use crate as jiff;
use crate::tz::TimeZone;

pub(super) static TZS: &[TimeZone] =
    &[jiff::tz::TimeZone::__internal_from_tzif(
        &jiff::shared::TzifFixed {
            name: Some("Europe/Zurich"),
            version: 50u8,
            checksum: 1541010761u32,
            designations: "LMT\0BMT\0CEST\0CET\0",
            posix_tz: Some(jiff::shared::PosixTimeZone {
                std_abbrev: "CET",
                std_offset: 3600i32,
                dst: Some(jiff::shared::PosixDst {
                    abbrev: "CEST",
                    offset: 7200i32,
                    rule: Some(jiff::shared::PosixRule {
                        start: jiff::shared::PosixDayTime {
                            date: jiff::shared::PosixDay::WeekdayOfMonth {
                                month: 3i8,
                                week: 5i8,
                                weekday: 0i8,
                            },
                            time: 7200i32,
                        },
                        end: jiff::shared::PosixDayTime {
                            date: jiff::shared::PosixDay::WeekdayOfMonth {
                                month: 10i8,
                                week: 5i8,
                                weekday: 0i8,
                            },
                            time: 10800i32,
                        },
                    }),
                }),
            }),
        }
        .to_jiff(
            &[
                jiff::shared::TzifLocalTimeType {
                    offset: 2048i32,
                    is_dst: false,
                    designation: (0u8, 3u8),
                    indicator: jiff::shared::TzifIndicator::LocalWall,
                }
                .to_jiff(),
                jiff::shared::TzifLocalTimeType {
                    offset: 1786i32,
                    is_dst: false,
                    designation: (4u8, 7u8),
                    indicator: jiff::shared::TzifIndicator::LocalWall,
                }
                .to_jiff(),
                jiff::shared::TzifLocalTimeType {
                    offset: 7200i32,
                    is_dst: true,
                    designation: (8u8, 12u8),
                    indicator: jiff::shared::TzifIndicator::LocalWall,
                }
                .to_jiff(),
                jiff::shared::TzifLocalTimeType {
                    offset: 3600i32,
                    is_dst: false,
                    designation: (13u8, 16u8),
                    indicator: jiff::shared::TzifIndicator::LocalWall,
                }
                .to_jiff(),
                jiff::shared::TzifLocalTimeType {
                    offset: 7200i32,
                    is_dst: true,
                    designation: (8u8, 12u8),
                    indicator: jiff::shared::TzifIndicator::UTStandard,
                }
                .to_jiff(),
                jiff::shared::TzifLocalTimeType {
                    offset: 3600i32,
                    is_dst: false,
                    designation: (13u8, 16u8),
                    indicator: jiff::shared::TzifIndicator::UTStandard,
                }
                .to_jiff(),
            ],
            &[
                jiff::shared::TzifTransition {
                    timestamp: -377705023201i64,
                    type_index: 0u8,
                }
                .to_jiff(2048i32, 2048i32),
                jiff::shared::TzifTransition {
                    timestamp: -3675198848i64,
                    type_index: 1u8,
                }
                .to_jiff(2048i32, 1786i32),
                jiff::shared::TzifTransition {
                    timestamp: -2385246586i64,
                    type_index: 3u8,
                }
                .to_jiff(1786i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: -904435200i64,
                    type_index: 2u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: -891129600i64,
                    type_index: 3u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: -872985600i64,
                    type_index: 2u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: -859680000i64,
                    type_index: 3u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 354675600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 370400400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 386125200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 401850000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 417574800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 433299600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 449024400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 465354000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 481078800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 496803600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 512528400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 528253200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 543978000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 559702800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 575427600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 591152400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 606877200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 622602000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 638326800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 654656400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 670381200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 686106000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 701830800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 717555600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 733280400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 749005200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 764730000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 780454800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 796179600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 811904400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 828234000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 846378000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 859683600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 877827600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 891133200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 909277200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 922582800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 941331600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 954032400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 972781200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 985482000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1004230800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1017536400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1035680400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1048986000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1067130000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1080435600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1099184400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1111885200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1130634000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1143334800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1162083600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1174784400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1193533200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1206838800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1224982800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1238288400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1256432400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1269738000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1288486800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1301187600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1319936400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1332637200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1351386000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1364691600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1382835600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1396141200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1414285200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1427590800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1445734800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1459040400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1477789200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1490490000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1509238800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1521939600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1540688400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1553994000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1572138000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1585443600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1603587600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1616893200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1635642000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1648342800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1667091600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1679792400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1698541200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1711846800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1729990800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1743296400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1761440400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1774746000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1792890000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1806195600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1824944400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1837645200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1856394000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1869094800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1887843600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1901149200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1919293200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1932598800i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1950742800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1964048400i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 1982797200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 1995498000i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 2014246800i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 2026947600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 2045696400i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 2058397200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 2077146000i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 2090451600i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 2108595600i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
                jiff::shared::TzifTransition {
                    timestamp: 2121901200i64,
                    type_index: 4u8,
                }
                .to_jiff(3600i32, 7200i32),
                jiff::shared::TzifTransition {
                    timestamp: 2140045200i64,
                    type_index: 5u8,
                }
                .to_jiff(7200i32, 3600i32),
            ],
        ),
    )];
