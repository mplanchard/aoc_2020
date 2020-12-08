struct PairIterator {
    vec: Vec<usize>,
    i_idx: usize,
    j_idx: usize,
}
impl PairIterator {
    fn new(vec: Vec<usize>) -> Self {
        Self {
            vec,
            i_idx: 0,
            j_idx: 0,
        }
    }
}
impl Iterator for PairIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let last_idx = self.vec.len() - 1;
        if self.i_idx < last_idx {
            if self.j_idx < last_idx {
                self.j_idx += 1;
                return Some((self.vec[self.i_idx], self.vec[self.j_idx]));
            }
            self.i_idx += 1;
            self.j_idx = self.i_idx + 1;
            if self.j_idx <= last_idx {
                return Some((self.vec[self.i_idx], self.vec[self.j_idx]));
            }
        }
        None
    }
}

struct TriadIterator {
    vec: Vec<usize>,
    i_idx: usize,
    j_idx: usize,
    k_idx: usize,
}
impl TriadIterator {
    fn new(vec: Vec<usize>) -> Self {
        Self {
            vec,
            i_idx: 0,
            j_idx: 1,
            k_idx: 1,
        }
    }
}
impl Iterator for TriadIterator {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let last_idx = self.vec.len() - 1;
        if self.i_idx < last_idx {
            if self.j_idx < last_idx {
                if self.k_idx < last_idx {
                    self.k_idx += 1;
                    return Some((
                        self.vec[self.i_idx],
                        self.vec[self.j_idx],
                        self.vec[self.k_idx],
                    ));
                }
                self.j_idx += 1;
                self.k_idx = self.j_idx + 1;
                if self.k_idx <= last_idx {
                    return Some((
                        self.vec[self.i_idx],
                        self.vec[self.j_idx],
                        self.vec[self.k_idx],
                    ));
                }
            }
            self.i_idx += 1;
            self.j_idx = self.i_idx + 1;
            self.k_idx = self.i_idx + 2;
            if self.k_idx <= last_idx {
                return Some((
                    self.vec[self.i_idx],
                    self.vec[self.j_idx],
                    self.vec[self.k_idx],
                ));
            }
        }
        None
    }
}

trait Pairs {
    /// Return a lazy iterator over unique pairs from the vec
    fn pairs(self) -> PairIterator;
}
impl Pairs for Vec<usize> {
    /// Return a lazy iterator over unique pairs from the vec
    fn pairs(self) -> PairIterator {
        PairIterator::new(self)
    }
}

trait Triads {
    /// Return a lazy iterator over unique triads from the vec
    fn triads(self) -> TriadIterator;
}

impl Triads for Vec<usize> {
    /// Return a lazy iterator over unique triads from the vec
    fn triads(self) -> TriadIterator {
        TriadIterator::new(self)
    }
}

fn get_day_one_input() -> Vec<usize> {
    vec![
        1384, 1396, 1072, 1903, 1387, 1763, 1600, 1862, 1992, 1585, 1909, 1352, 1288,
        1910, 1070, 1421, 1802, 1669, 1059, 1235, 1854, 1722, 1275, 198, 1476, 1588,
        1708, 1217, 1596, 1355, 1566, 1973, 1335, 1480, 1115, 1272, 1998, 1821, 2007,
        1721, 1885, 1420, 1412, 1487, 1941, 1835, 1558, 1061, 1582, 1940, 1942, 1210,
        1350, 1175, 1047, 1456, 1548, 1110, 1510, 1995, 1644, 1968, 1297, 1198, 1471,
        1360, 1363, 1528, 1393, 1365, 1837, 1886, 2001, 1161, 1349, 1787, 988, 1331,
        1960, 1607, 1324, 97, 1986, 1955, 1773, 1443, 1852, 1368, 1050, 1378, 1239,
        1750, 1868, 816, 1965, 1661, 1728, 1981, 984, 1037, 1525, 1789, 1318, 1952,
        1359, 1358, 1869, 1641, 1240, 1542, 1959, 1022, 1475, 1733, 1081, 1889, 1138,
        1757, 1736, 1723, 1543, 1820, 1128, 1039, 1683, 1477, 1375, 1499, 676, 1195,
        1250, 220, 1581, 1328, 1187, 1485, 1216, 1769, 1139, 1064, 1908, 1516, 1490,
        1419, 1749, 1347, 1758, 1024, 1053, 1842, 1861, 1403, 1966, 1546, 1134, 1593,
        1734, 1916, 1867, 1101, 1126, 1301, 1841, 1515, 1244, 1401, 1637, 1054, 1309,
        1933, 1512, 1263, 1815, 1634, 1823, 1295, 1583, 1104, 1765, 1850, 1311, 1692,
        1905, 1149, 1780, 1330, 1666, 996, 1913, 1140, 1089, 1484, 1356, 1296, 1323,
        1160, 1881, 1123, 1166, 1929,
    ]
}

pub fn day_one_solution_one() -> usize {
    get_day_one_input()
        .pairs()
        .filter(|(i, j)| i + j == 2020)
        .nth(0)
        .map(|(i, j)| i * j)
        .unwrap()
}

pub fn day_one_solution_two() -> usize {
    get_day_one_input()
        .triads()
        .filter(|(i, j, k)| i + j + k == 2020)
        .nth(0)
        .map(|(i, j, k)| i * j * k)
        .unwrap()
}
