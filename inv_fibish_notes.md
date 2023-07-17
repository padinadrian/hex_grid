
1 => 1      (6 * 0)  + 1    (6 * (0))
7 => 3      (6 * 1)  + 1    (6 * (0 + 1))
19 => 5     (6 * 3)  + 1    (6 * (0 + 1 + 2))
37 => 7     (6 * 6)  + 1    (6 * (0 + 1 + 2 + 3))
61 => 9     (6 * 10) + 1    (6 * (0 + 1 + 2 + 3 + 4))


number_of_layers = inv_fibish
width = (number_of_layers - 1) * 2 + 1

fn inv_fibish(mut f_n: i32) {
    let mut n = 1;
    while f_n > n { f_n -= n; }
    n
}