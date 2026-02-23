pub struct Paginator {
    page: usize,
    per_page: usize,
}

impl Paginator {
    pub fn new(per_page: usize) -> Self {
        Self { page: 0, per_page }
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn next(&mut self) {
        self.page += 1;
    }

    pub fn prev(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
    }

    pub fn offset(&self) -> usize {
        self.page * self.per_page
    }

    pub fn limit(&self) -> usize {
        self.per_page
    }
}