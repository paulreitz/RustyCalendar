use chrono::{DateTime, Local, Datelike, NaiveDate, Duration};

pub struct DateManager {
    current_date: DateTime<Local>,
    selected_date: DateTime<Local>,
}

impl Default for DateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DateManager {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            current_date: now,
            selected_date: now,
        }
    }

    pub fn selected_month(&self) -> String {
        self.selected_date.format("%B").to_string()
    }

    pub fn selected_year(&self) -> i32 {
        self.selected_date.year()
    }

    // Need to find a way to put this in the window title.
    // pub fn formatted_current_date(&self) -> String {
    //     self.current_date.format("%B %d, %Y").to_string()
    // }

    pub fn calendar_dates(&self) -> Vec<NaiveDate> {
        let mut dates = Vec::new();
        let first_of_month = self.selected_date.date_naive()
            .with_day(1)
            .expect("Failed to get first day of month");
        let days_from_prev = (first_of_month.weekday().num_days_from_monday() + 1) % 7;
        let mut current_date = first_of_month - Duration::days(days_from_prev as i64);

        for _ in 0..42 {
            dates.push(current_date);
            current_date += Duration::days(1);
        }

        dates
    }

    pub fn is_selected_month(&self, date: NaiveDate) -> bool {
        date.month() == self.selected_date.month() &&
        date.year() == self.selected_date.year()
    }

    pub fn is_current_date(&self, date: NaiveDate) -> bool {
        date == self.current_date.date_naive()
    }

    pub fn next_month(&mut self) {
        let current_month = self.selected_date.month();
        let current_year = self.selected_date.year();
        
        // If December, move to January of next year
        if current_month == 12 {
            self.selected_date = self.selected_date
                .with_day(1).unwrap()
                .with_month(1).unwrap()
                .with_year(current_year + 1).unwrap();
        } else {
            // Otherwise just increment the month
            self.selected_date = self.selected_date
                .with_day(1).unwrap()
                .with_month(current_month + 1).unwrap();
        }
    }

    pub fn previous_month(&mut self) {
        let current_month = self.selected_date.month();
        let current_year = self.selected_date.year();
        
        // If January, move to December of previous year
        if current_month == 1 {
            self.selected_date = self.selected_date
                .with_day(1).unwrap()
                .with_month(12).unwrap()
                .with_year(current_year - 1).unwrap();
        } else {
            // Otherwise just decrement the month
            self.selected_date = self.selected_date
                .with_day(1).unwrap()
                .with_month(current_month - 1).unwrap();
        }
    }
}