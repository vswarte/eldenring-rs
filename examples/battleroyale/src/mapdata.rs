use std::sync::LazyLock;

use game::{
    cs::MapId,
    position::{BlockPoint, ChunkPosition4},
};

pub const MAP_CONFIG: LazyLock<Vec<MapConfiguration>> = LazyLock::new(|| {
    vec![MapConfiguration {
        quickmatch_map_id: 0,
        player_spawn_points: vec![
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-103.65, 44.02, 190.74),
                orientation: 0.0,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-63.80, 48.86, 134.71),
                orientation: 0.0,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-21.00, 87.23, 244.24),
                orientation: 0.0,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-60.93, 73.26, 206.98),
                orientation: 0.0,
            },
        ],
        item_spawn_points: vec![
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-38.31659, 76.69234, 180.39774),
                orientation: -3.094547,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-37.178814, 67.61757, 216.68727),
                orientation: 1.9493979,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-32.646408, 73.26856, 196.88576),
                orientation: 0.38517046,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-38.778732, 73.26856, 190.12466),
                orientation: -2.6141443,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-13.559275, 69.46372, 204.53665),
                orientation: -0.081388,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(4.582514, 67.32002, 200.15695),
                orientation: -0.040528443,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(3.325323, 62.353355, 187.61911),
                orientation: -0.08153486,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(19.98627, 62.353355, 198.42915),
                orientation: -0.19068956,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(45.767193, 58.835075, 191.92606),
                orientation: 2.2384288,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(59.718597, 57.36333, 200.3674),
                orientation: 3.0074272,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(72.942696, 57.560356, 212.16129),
                orientation: 2.9575138,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(76.87945, 57.36335, 232.46727),
                orientation: 2.120494,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(68.35415, 57.22838, 242.03749),
                orientation: 0.7939468,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(61.336807, 57.363346, 250.55478),
                orientation: 0.57132745,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(51.541042, 57.36335, 254.06879),
                orientation: -0.7105155,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(34.80216, 57.241013, 245.27887),
                orientation: -0.76200724,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(15.175749, 59.853798, 224.25876),
                orientation: -0.20024243,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(24.29547, 66.61205, 203.63159),
                orientation: 3.0555758,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(51.31929, 58.77026, 229.23563),
                orientation: 2.2843351,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(51.229668, 58.76875, 224.68481),
                orientation: 0.7710471,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(55.695255, 58.77051, 224.54608),
                orientation: -0.8727942,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-15.949009, 66.53897, 151.87738),
                orientation: 1.9993172,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-25.411839, 64.27217, 144.9866),
                orientation: 0.4355904,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-19.762978, 59.385494, 150.73082),
                orientation: -0.5388999,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-27.476912, 57.49138, 146.22635),
                orientation: 2.0498328,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-37.52786, 56.99025, 151.41205),
                orientation: 1.9716451,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-49.397827, 61.727425, 160.99548),
                orientation: -2.7145042,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-46.88455, 57.328102, 161.3485),
                orientation: -2.0332177,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-50.631626, 57.51834, 163.12067),
                orientation: 2.6290522,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-45.772488, 48.865, 159.76984),
                orientation: -2.0029404,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-22.827997, 48.865005, 154.70932),
                orientation: 2.2959864,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-41.85961, 45.946465, 178.25877),
                orientation: 2.0529585,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-47.784462, 45.946465, 173.67235),
                orientation: 2.341693,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-50.693, 45.946465, 188.59079),
                orientation: 0.3929776,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-63.627018, 45.946465, 188.33206),
                orientation: -1.1494373,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-60.27642, 46.903126, 172.50526),
                orientation: -2.8081677,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-60.652744, 46.88784, 168.156),
                orientation: -0.07870436,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-71.9265, 47.008713, 167.17407),
                orientation: -2.0804467,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-48.043835, 48.865005, 169.80305),
                orientation: 1.2908518,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-70.51637, 91.49872, 249.81018),
                orientation: -3.0214996,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-78.71672, 92.94957, 274.76978),
                orientation: 0.6787755,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-103.38568, 95.63673, 253.99663),
                orientation: 2.4736762,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-99.77348, 110.720276, 280.32504),
                orientation: 0.118503265,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-51.99202, 118.17397, 281.36557),
                orientation: 0.32019168,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-79.81348, 118.20866, 292.57306),
                orientation: -1.4358041,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-92.95022, 110.70308, 273.28595),
                orientation: -2.237702,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-91.713394, 110.70416, 280.72214),
                orientation: -0.6576123,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-111.69753, 93.18045, 279.2752),
                orientation: -1.4946914,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-79.44243, 92.56491, 268.6256),
                orientation: 2.4903088,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-43.16407, 87.289955, 251.07683),
                orientation: 2.2569327,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-72.8277, 93.740974, 277.09955),
                orientation: -1.4638162,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-32.58322, 87.289955, 250.11568),
                orientation: -2.5142446,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-8.77541, 83.3024, 209.78624),
                orientation: 2.058339,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-44.000565, 70.88488, 186.69656),
                orientation: -1.1203865,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-45.34166, 68.54164, 176.93436),
                orientation: -1.9838061,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-29.683687, 66.68925, 164.32083),
                orientation: -1.1658707,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-35.51379, 62.918987, 149.27628),
                orientation: 0.31773132,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-54.708817, 56.741272, 153.99284),
                orientation: 1.0658506,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-39.48952, 48.865, 156.44925),
                orientation: 2.5460472,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-85.38492, 49.138493, 162.08215),
                orientation: 1.8395779,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-85.54056, 49.215546, 142.09113),
                orientation: -2.7380624,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-97.06249, 26.961826, 152.60858),
                orientation: 0.15634084,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-80.698044, 27.005777, 144.67903),
                orientation: 0.27667022,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-73.04982, 26.444477, 136.6904),
                orientation: 2.5597987,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-115.833664, 43.915237, 204.24072),
                orientation: 0.45573884,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-123.08621, 43.915237, 188.552),
                orientation: -2.7164295,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-127.63259, 43.915237, 193.28413),
                orientation: -1.9081933,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-119.39305, 29.514996, 119.57115),
                orientation: -1.7791888,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-103.97896, 29.643764, 122.19893),
                orientation: 1.4096353,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-99.084496, 20.766296, 80.5783),
                orientation: -2.132061,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-95.93078, 20.939365, 87.28386),
                orientation: -0.8557381,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-77.485344, 20.939365, 95.393486),
                orientation: 1.7994848,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-78.75835, 13.757237, 78.75317),
                orientation: -0.75863075,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-76.94045, 12.638696, 99.810684),
                orientation: 0.5223246,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-99.2683, 12.638696, 77.0735),
                orientation: -2.1308923,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-86.53215, 12.638696, 67.06495),
                orientation: -0.92714345,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-64.12042, 13.798362, 64.477325),
                orientation: 2.2086372,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-21.539501, 6.84878, 39.606228),
                orientation: 0.87223387,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-39.041977, 6.8406014, 26.75164),
                orientation: -0.792616,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-26.311394, 6.84878, 39.396168),
                orientation: -0.76407146,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-36.524597, 6.84878, 19.339172),
                orientation: -2.336394,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-24.81176, 5.1149845, 25.161121),
                orientation: 2.3951051,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-55.23001, 48.923077, 144.07431),
                orientation: -2.7236052,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(5.829872, 53.875008, 119.46925),
                orientation: 2.0175202,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-5.738411, 54.927544, 135.48813),
                orientation: -1.2227653,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-1.054501, 57.393944, 143.11456),
                orientation: 0.29803413,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-6.3121786, 54.184998, 149.56902),
                orientation: -0.34551477,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-8.236016, 61.308807, 140.53638),
                orientation: -2.7136855,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-38.82264, 59.43837, 108.512726),
                orientation: -2.7038977,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-59.1987, 56.414494, 121.16169),
                orientation: -2.0002017,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-52.951435, 56.414494, 118.37593),
                orientation: 2.870933,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-34.92987, 53.527637, 130.2971),
                orientation: -2.6102276,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-57.51651, 48.865, 116.96189),
                orientation: -2.7676926,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-22.473898, 67.76521, 209.0747),
                orientation: 1.1587276,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-36.420284, 73.85125, 209.03487),
                orientation: 1.9332718,
            },
            SpawnPoint {
                map: MapId::from_parts(20, 00, 00, 00),
                position: BlockPoint::from_xyz(-63.940086, 67.258545, 204.59904),
                orientation: -2.0369835,
            },
        ],
        event_flag_overrides: vec![
            // Defeat divine beast to clear out the arena
            (9140, true),

            // First elevator 
            (20000540, true), // Open door
            (20008540, true), // Remove action region
            (20005012, true),

            // Well Depths entrance
            (20000562, true), // Open door
            (20008562, true), // Remove action region
            (20005007, true),

            // Storeroom entrance
            (20000564, true), // Open door
            (20008564, true), // Remove action region
            (20005008, true),

            // Storeroom backdoor
            (20000566, true), // Open door
            (20008566, true), // Remove action region
            (20005009, true),

            // Well Depths ladder door
            (20000560, true), // Open door
            (20008560, true), // Remove action region
            (20005010, true),
        ],
    }]
});

pub struct MapConfiguration {
    /// Quickmatch map ID to map against
    pub quickmatch_map_id: u32,
    /// Spawn points for this map
    pub player_spawn_points: Vec<SpawnPoint>,
    /// Spawn points for this map
    pub item_spawn_points: Vec<SpawnPoint>,
    /// Event flags that need to be set while loading this map.
    pub event_flag_overrides: Vec<(u32, bool)>,
}

#[derive(Clone, Debug)]
pub struct SpawnPoint {
    /// Map ID to load into
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: BlockPoint,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
}
