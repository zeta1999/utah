
use types::*;


#[derive(Clone)]
pub struct Impute<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    data: I,
    strategy: ImputeStrategy,
}

impl<'a, I> Impute<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    pub fn new(df: I, s: ImputeStrategy) -> Impute<'a, I>
        where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
    {

        Impute {
            data: df,
            strategy: s,
        }
    }
}

impl<'a, I> Iterator for Impute<'a, I>
    where I: Iterator<Item = (OuterType, RowView<'a, InnerType>)>
{
    type Item = (OuterType, Row<InnerType>);
    fn next(&mut self) -> Option<Self::Item> {

        match self.data.next() {

            None => return None,
            Some((val, dat)) => {
                match self.strategy {
                    ImputeStrategy::Mean => unsafe {
                        let size = dat.len();
                        let first_element = dat.uget(0).to_owned();
                        let sum = (0..size).fold(first_element, |x, y| x + dat.uget(y).to_owned());

                        let mean = match dat.uget(0) {
                            &InnerType::Float(_) => sum / InnerType::Float(size as f64),
                            &InnerType::Int32(_) => sum / InnerType::Int32(size as i32),
                            &InnerType::Int64(_) => sum / InnerType::Int64(size as i64),
                            _ => InnerType::Empty,
                        };
                        return Some((val,
                                     dat.map(|x| {
                            match x {
                                &InnerType::Empty => mean.to_owned(),
                                _ => x.to_owned(),
                            }

                        })));
                    },
                    ImputeStrategy::Mode => {
                        let max = dat.iter().max().map(|x| x.to_owned()).unwrap();
                        return Some((val,
                                     dat.map(|x| {
                            match x {
                                &InnerType::Empty => max.to_owned(),
                                _ => x.to_owned(),
                            }

                        })));
                    }
                }
            }
        }
    }
}