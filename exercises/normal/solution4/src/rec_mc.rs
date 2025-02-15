pub fn dp_rec_mc(mut amount: u32) -> u32 {
    let money = [100, 50, 30, 20, 10, 5, 2, 1];
    let mut cnt = 0;
    for &cash in money.iter() {
        while amount >= cash {
            amount -= cash;
            cnt += 1;
        }
    }
    if amount!= 0 {
        return 0;
    } else {
        return cnt;
    }
}