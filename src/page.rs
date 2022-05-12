use std::slice::Iter;

use reqwest::Url;

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub current: Url,
    pub next: Option<Url>,
    pub prev: Option<Url>,
    pub first: Url,
    pub last: Option<Url>,
}

impl<T> Page<T> {
    pub fn all(&self) -> Vec<T> {
        todo!()
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'iter, T> IntoIterator for &'iter Page<T> {
    type Item = &'iter T;
    type IntoIter = Iter<'iter, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
