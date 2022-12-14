use anyhow::Result;
use polars::prelude::*;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    println!("Result for part 1 is {}", result_1(&file_contents).unwrap());
    // println!("Result for part 2 is {}", result_2(&file_contents).unwrap());
}

fn result_1(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let tree_map = tree_map(lines);
    let cols: Vec<&str> = tree_map.get_column_names().into_iter().rev().collect();

    let cummax_from_top = tree_map.cummax_height_from_n()?;
    let cummax_from_left = tree_map.transpose()?.cummax_height_from_n()?;
    let cummax_from_bottom = tree_map.reverse().cummax_height_from_n()?;
    let cummax_from_right = tree_map.transpose()?.reverse().cummax_height_from_n()?;

    let visible_from_top = trees_visible(&tree_map, cummax_from_top)?;
    let visible_from_left = trees_visible(&tree_map.transpose()?, cummax_from_left)?.transpose()?;
    let visible_from_bottom = trees_visible(&tree_map.reverse(), cummax_from_bottom)?.reverse();
    let visible_from_right = trees_visible(&tree_map.transpose()?.reverse(), cummax_from_right)?
        .transpose()?
        .select(&cols)?;

    let visible_mask = visible_from_top
        .bitor(visible_from_left)?
        .bitor(visible_from_bottom.bitor(visible_from_right)?)?;

    let result = visible_mask
        .sum()
        .hsum(NullStrategy::Ignore)?
        .unwrap()
        .sum::<i32>()
        .unwrap();

    Ok(result)
}

fn tree_map(lines: Vec<&str>) -> DataFrame {
    let mut rows: Vec<Series> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let digits: Series = line
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .collect();
        rows.push(Series::new(&idx.to_string(), digits));
    }
    DataFrame::new(rows).unwrap().transpose().unwrap()
}

fn trees_visible(tree_map: &DataFrame, cummax: DataFrame) -> Result<DataFrame> {
    let mask = tree_map
        .get_column_names()
        .iter()
        .map(|name| {
            when(col(name).gt(col(&format!("{name}_right"))))
                .then(col(name))
                .otherwise(lit(NULL))
        })
        .collect::<Vec<Expr>>();

    Ok(tree_map
        .clone()
        .lazy()
        .with_context(&[cummax.lazy().select(&[all().suffix("_right")])])
        .select(&mask)
        .collect()?
        .lazy()
        .select([all().is_not_null()])
        .collect()?)
}

trait ElementWise {
    fn bitor(&self, df: DataFrame) -> Result<DataFrame>;
    fn cummax_height_from_n(&self) -> Result<DataFrame>;
}

impl ElementWise for DataFrame {
    fn bitor(&self, df_2: DataFrame) -> Result<DataFrame> {
        Ok(DataFrame::new(
            self.iter()
                .zip(df_2.iter())
                .map(|(series1, series2)| series1.bitor(series2).unwrap())
                .collect(),
        )?)
    }

    fn cummax_height_from_n(&self) -> Result<DataFrame> {
        Ok(self
            .clone()
            .lazy()
            .select([all().cummax(false).shift(1).fill_null(-1)])
            .collect()?)
    }
}

// fn result_2(input: &str) -> Result<i32> {
//     let lines: Vec<&str> = input.trim().split("\n").collect();
//     let _tree_map = tree_map(lines);
//
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        let res = result_1(&file_contents).unwrap();
        assert_eq!(res, 21);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt")
    //         .expect("Should have been able to read the file");
    //     let res = result_2(&file_contents).unwrap();
    //     assert_eq!(res, 8);
    // }
}
