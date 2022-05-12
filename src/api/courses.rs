use crate::{models::courses::Course, page::Page, result::Result, Canvas};

pub struct CourseHandler<'a> {
    canvas: &'a Canvas,
}

impl<'a> CourseHandler<'a> {
    pub(crate) fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }

    pub async fn my_courses(&self) -> Result<Page<Course>> {
        self.canvas.get_page("courses/", None::<&()>).await
    }
}
