use anyhow::Result;
use polars::prelude::*;
use std::fs;

fn main() {
    let file_contents =
        fs::read_to_string("data/input.txt").expect("Should have been able to read the file");
    println!("Result for part 1 is {}", result_1(&file_contents).unwrap());
    // println!("Result for part 2 is {}", result_2(&file_contents).unwrap());
}

fn result_1(input: &str) -> Result<i32> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let tree_map = tree_map(lines);
    let mut cols = tree_map.get_column_names();
    cols.reverse();

    let cummax_from_top = cummax_height_from_n(&tree_map)?;
    let cummax_from_left = cummax_height_from_n(&tree_map.transpose()?)?;
    let cummax_from_bottom = cummax_height_from_n(&reverse_df(&tree_map)?)?;
    let cummax_from_right = cummax_height_from_n(&reverse_df(&tree_map.transpose()?)?)?;

    let visible_from_top = trees_visible(&tree_map, cummax_from_top)?;
    let visible_from_left =
        trees_visible(&tree_map.transpose().unwrap(), cummax_from_left)?.transpose()?;
    let visible_from_bottom =
        reverse_df(&trees_visible(&reverse_df(&tree_map)?, cummax_from_bottom)?)?;
    let visible_from_right =
        trees_visible(&reverse_df(&tree_map.transpose()?)?, cummax_from_right)?
            .transpose()?
            .select(&cols)?;

    let visible_mask = bitor(
        bitor(visible_from_top, visible_from_left)?,
        bitor(visible_from_bottom, visible_from_right)?,
    )?;

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

fn cummax_height_from_n(df: &DataFrame) -> Result<DataFrame> {
    Ok(df
        .clone()
        .lazy()
        .select([all().cummax(false).shift(1).fill_null(-1)])
        .collect()?)
}

fn reverse_df(df: &DataFrame) -> Result<DataFrame> {
    Ok(df
        .clone()
        .lazy()
        .select([all().reverse().suffix("")])
        .collect()?)
}

fn bitor(df_1: DataFrame, df_2: DataFrame) -> Result<DataFrame> {
    Ok(DataFrame::new(
        df_1.iter()
            .zip(df_2.iter())
            .map(|(series1, series2)| series1.bitor(series2).unwrap())
            .collect(),
    )?)
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
        let file_contents = fs::read_to_string("data/demo_input.txt")
            .expect("Should have been able to read the file");
        let res = result_1(&file_contents).unwrap();
        assert_eq!(res, 21);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt")
    //         .expect("Should have been able to read the file");
    //     let res = result(&file_contents, Part::Two).unwrap();
    //     assert_eq!(res, 8);
    // }
}
