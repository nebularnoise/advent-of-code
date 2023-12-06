// t :       (ms)      time the button is pressed
// t_max :   (ms)      maximum allowed time for the race
// v :       (mm/ms)   speed that the boat has when the button is released
// d :       (mm)      distance that the boat eventually makes
// k :       (mms/ms²) speed gained per millisecond of pressing the button

// d = (k * t) * (t_max - t)
//
//     |_____|   |_________|
//        v         delta_t
//
// the distance is fully characterized by the following second degree polynomial:
//    d = -k * t²  + k*t_max * t
//
// Its shape is an upside down parabolla:
//
//    d ^
//      |
//      |            xx
//      |          x    x
//      |        x        x
//      |       x          x
//      |      x            x
//      |     x              x
//      |    x                x
//      +------------------------------> t
//
//
// we need to find all speeds that reach beyond the current record d0
//
//
//    d ^
//      |
//      |            xx
//      |          x    x
//      |        x        x
//   d0 +-------x----------x------------
//      |      x|          |x
//      |     x |          | x
//      |    x  |          |  x
//      +-------+----------+-----------> t
//              t1         t2
//
// t1 and t2 are the roots of the following second degree polynomial:
//
//          -k * t² + k*t_max * t - d0
//
// or        a * t² + b * t + c
//
// with      a = -k
//           b = k * t_max
//           c = -d0
//
//
// the general solution of this problem is the following:
//
// delta = b² - 4*ac
//
// t1 = (-b - sqrt(delta)) / (2*a)
// t2 = (-b + sqrt(delta)) / (2*a)
//
// if delta is negative, there are no real solutions, only imaginary ones
// if delta is equal to zero, the two roots t1 and t2 are equal

use std::ops::Range;

fn main() {}

struct SecondDegreePolynomial {
    a: i64,
    b: i64,
    c: i64,
}

impl SecondDegreePolynomial {
    fn real_roots(&self) -> Option<(f64, f64)> {
        let delta = self.b * self.b - (4 * self.a * self.c);

        if delta < 0 {
            return None;
        }

        let (a, b, sq_delta) = (self.a as f64, self.b as f64, (delta as f64).sqrt());

        Some(((-b + sq_delta) / (2.0 * a), (-b - sq_delta) / (2.0 * a)))
    }
}

const K: i64 = 1;
struct BoatRace {
    t_max: usize,
    current_record: usize,
}

impl BoatRace {
    fn winning_moves(&self) -> Option<Range<usize>> {
        let pol = SecondDegreePolynomial {
            a: -K,
            b: K * self.t_max as i64,
            c: -(self.current_record as i64),
        };
        if let Some((t1, t2)) = pol.real_roots() {
            let is_integer = |f: f64| f.fract() < (10.0 * f64::EPSILON);
            let t1 = if is_integer(t1) {
                t1 as usize + 1
            } else {
                t1.ceil() as usize
            };
            let t2 = if is_integer(t2) {
                t2 as usize
            } else {
                t2.floor() as usize + 1
            };

            if t1 > t2 {
                None
            } else {
                Some(t1..t2)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn winning_moves() {
        let races = vec![
            BoatRace {
                t_max: 7,
                current_record: 9,
            },
            BoatRace {
                t_max: 15,
                current_record: 40,
            },
            BoatRace {
                t_max: 30,
                current_record: 200,
            },
        ];

        assert_eq!(
            races.iter().map(|r| r.winning_moves()).collect::<Vec<_>>(),
            vec![Some(2..6), Some(4..12), Some(11..20)]
        );

        assert_eq!(
            races
                .iter()
                .filter_map(|r| r.winning_moves().map(|r| r.len()))
                .product::<usize>(),
            288
        );
    }
}
