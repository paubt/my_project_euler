fn main() {
    println!("{}", from_to_sun_first_month());
}

fn from_to_sun_first_month() -> u32 {
    let mut sunday_first_count = 0;
    let stop_year = 2000;
    let mut year = 1901;
    let mut day = 1;
    while year <= stop_year {
        for month in 0..12 {
            // check this month
            if day == 6 {
                sunday_first_count += 1;
            }
            // update the month
            let mut month_length: u32 = 0;
            // February
            if month == 1 {
                if year % 4 == 0 {
                    if year % 100 == 0 {
                        if year % 400 == 0 {
                            month_length = 29;
                        } else {
                            month_length = 28;
                        }
                    } else {
                        month_length = 29;
                    }
                } else {
                    month_length = 28;
                }
            }
            // 30 days
            else if month == 3 || month == 5 || month == 8 || month == 10 {
                month_length = 30;
            }
            // and 31 days
            else {
                month_length = 31;
            }
            day = (day + month_length) % 7;
        }
        year += 1;
    }
    sunday_first_count
}
